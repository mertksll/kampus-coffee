//! Kampus's Coffee iş işlevleri: sepet toplamı ve toplu indirim, sipariş verme (KMP-XXXXXXXX kodu), durum ilerletme,
//! iptal, kurye atama, iade kararı, indirim kuralı yayını, panel özeti.
//! Formüller `src/lib/coffee.ts` ile aynıdır; boyut/süt farkı ve indirim eşiği ise tablolardan gelir.
use crate::backend::audit;
use crate::backend::crud;
use crate::backend::error::{AppError, AppResult, FieldError};
use crate::backend::policy::authorize;
use crate::backend::rbac::{self, ModelRef};
use crate::backend::schema;
use crate::backend::state::AppState;
use crate::backend::util::{now_iso, NOW_SQL};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
use tauri::State;

pub const SIZES: [&str; 2] = ["Standart", "Büyük"];
pub const MILKS: [&str; 3] = ["Normal süt", "Yulaf sütü", "Sütsüz"];
pub const STATUSES: [&str; 3] = ["Hazırlanıyor", "Kuryede", "Teslim edildi"];

pub struct Item {
    pub product_id: i64,
    pub size: String,
    pub milk: String,
    pub quantity: i64,
}

pub struct Rule {
    pub min_quantity: i64,
    pub rate: f64,
}

fn parse_items(v: &Value) -> AppResult<Vec<Item>> {
    let arr = v.as_array().ok_or_else(|| AppError::validation("items", "Sepet bir liste olmalı."))?;
    arr.iter()
        .map(|i| {
            Ok(Item {
                product_id: i.get("productId").and_then(Value::as_i64).ok_or_else(|| AppError::validation("items", "productId gerekli."))?,
                size: i.get("size").and_then(Value::as_str).unwrap_or("").to_string(),
                milk: i.get("milk").and_then(Value::as_str).unwrap_or("").to_string(),
                quantity: i.get("quantity").and_then(Value::as_i64).unwrap_or(0),
            })
        })
        .collect()
}

/// Birim fiyat = ürün fiyatı + boyut farkı (Büyük +15) + süt farkı (Yulaf sütü +10)
pub fn unit_price(base: f64, size_diff: f64, milk_diff: f64) -> f64 {
    base + size_diff + milk_diff
}

/// Toplu indirim: toplam adet eşiğe ulaştıysa ara toplamın `rate` kadarı (2 ondalık).
pub fn totals_of(lines: &[(f64, i64)], rule: &Rule) -> Value {
    let quantity: i64 = lines.iter().map(|(_, q)| q).sum();
    let subtotal: f64 = lines.iter().map(|(p, q)| p * *q as f64).sum();
    let discount = if quantity >= rule.min_quantity { (subtotal * rule.rate * 100.0).round() / 100.0 } else { 0.0 };
    json!({ "quantity": quantity, "subtotal": subtotal, "discount": discount, "total": subtotal - discount })
}

fn option_diffs(c: &Connection) -> AppResult<(Value, Value)> {
    let row: Option<(String, String)> = c.query_row("SELECT size, milk FROM option_pricing WHERE id = 'default'", [], |r| Ok((r.get(0)?, r.get(1)?))).optional()?;
    let (s, m) = row.ok_or_else(|| AppError::NotFound("Seçenek fiyatları tanımlı değil.".into()))?;
    Ok((serde_json::from_str(&s)?, serde_json::from_str(&m)?))
}

fn active_rule(c: &Connection, cafe: &str) -> AppResult<Rule> {
    let rows: Vec<(i64, f64, String)> = {
        let mut st = c.prepare(
            "SELECT minQuantity, rate, cafes FROM discount_rules WHERE status = 'published'
             AND (validFrom IS NULL OR validFrom <= strftime('%Y-%m-%dT%H:%M:%SZ','now'))
             AND (validTo IS NULL OR validTo >= strftime('%Y-%m-%dT%H:%M:%SZ','now')) ORDER BY version DESC",
        )?;
        let r = st.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        r.collect::<Result<Vec<_>, _>>()?
    };
    for (min, rate, cafes) in rows {
        let v: Value = serde_json::from_str(&cafes).unwrap_or(json!("all"));
        let applies = v == json!("all") || v.as_array().map(|a| a.iter().any(|x| x.as_str() == Some(cafe))).unwrap_or(false);
        if applies {
            return Ok(Rule { min_quantity: min, rate });
        }
    }
    Ok(Rule { min_quantity: i64::MAX, rate: 0.0 }) // geçerli kural yok: indirim yok
}

/// Sepeti doğrular ve toplamı hesaplar. Dönen: `(cafe, satırlar, toplamlar)`
pub fn price_cart(c: &Connection, items: &[Item]) -> AppResult<(String, Value, Value)> {
    let mut errors: Vec<FieldError> = Vec::new();
    if items.is_empty() || items.len() > 100 {
        errors.push(FieldError { field: "items".into(), message: "Sepet 1–100 satır olmalı.".into() });
    }
    let (size_d, milk_d) = option_diffs(c)?;
    let mut cafe: Option<String> = None;
    let mut lines: Vec<(f64, i64)> = Vec::new();
    let mut detail: Vec<Value> = Vec::new();
    for (idx, it) in items.iter().enumerate() {
        let p: Option<(String, f64, bool)> = c
            .query_row("SELECT cafe, price, active FROM products WHERE id = ?1", [it.product_id], |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, i64>(2)? != 0)))
            .optional()?;
        let Some((pcafe, price, active)) = p else {
            errors.push(FieldError { field: format!("items[{idx}].productId"), message: "Ürün bulunamadı.".into() });
            continue;
        };
        if !active {
            errors.push(FieldError { field: format!("items[{idx}].productId"), message: "Ürün şu an satışta değil.".into() });
        }
        if !SIZES.contains(&it.size.as_str()) {
            errors.push(FieldError { field: format!("items[{idx}].size"), message: "Boyut geçersiz.".into() });
        }
        if !MILKS.contains(&it.milk.as_str()) {
            errors.push(FieldError { field: format!("items[{idx}].milk"), message: "Süt seçeneği geçersiz.".into() });
        }
        if !(1..=99).contains(&it.quantity) {
            errors.push(FieldError { field: format!("items[{idx}].quantity"), message: "Adet 1–99 olmalı.".into() });
        }
        match &cafe {
            None => cafe = Some(pcafe.clone()),
            Some(existing) if *existing != pcafe => errors.push(FieldError { field: "items".into(), message: "Bir siparişte yalnız tek kafenin ürünleri olabilir.".into() }),
            _ => {}
        }
        let u = unit_price(price, size_d[&it.size].as_f64().unwrap_or(0.0), milk_d[&it.milk].as_f64().unwrap_or(0.0));
        lines.push((u, it.quantity));
        detail.push(json!({ "productId": it.product_id, "size": it.size, "milk": it.milk, "quantity": it.quantity, "unitPrice": u }));
    }
    if !errors.is_empty() {
        return Err(AppError::Validation(errors));
    }
    let cafe = cafe.unwrap_or_default();
    let totals = totals_of(&lines, &active_rule(c, &cafe)?);
    Ok((cafe, json!(detail), totals))
}

fn customer_id(uid: i64) -> String {
    format!("u{uid}")
}

fn event(c: &Connection, code: &str, from: Option<i64>, to: &str, actor: Option<i64>, reason: Option<&str>) -> AppResult<()> {
    c.execute(
        &format!("INSERT INTO order_events (orderCode, \"from\", \"to\", actorId, at, reason) VALUES (?1, ?2, ?3, ?4, {NOW_SQL}, ?5)"),
        params![code, from, to, actor.map(|a| a.to_string()), reason],
    )?;
    Ok(())
}

fn trimmed<'a>(v: &'a Value, field: &str, max: usize) -> AppResult<&'a str> {
    let s = v.get(field).and_then(Value::as_str).map(str::trim).unwrap_or("");
    if s.is_empty() || s.chars().count() > max {
        return Err(AppError::validation(field, &format!("Zorunlu, en çok {max} karakter.")));
    }
    Ok(s)
}

/// Sepet toplamı (oturum gerekmez): `items` → `{ cafe, lines, totals }`
pub fn do_cart_totals(app: &AppState, items: &Value) -> AppResult<Value> {
    let items = parse_items(items)?;
    app.db.with(|c| {
        let (cafe, lines, totals) = price_cart(c, &items)?;
        Ok(json!({ "cafe": cafe, "lines": lines, "totals": totals }))
    })
}

/// Sipariş ver. `girdi`: `{ items, building, floor, room }` — kod: `KMP-XXXXXXXX`.
pub fn do_order_place(app: &AppState, girdi: &Value) -> AppResult<Value> {
    let uid = app.user().ok_or(AppError::Unauthenticated)?;
    let items = parse_items(girdi.get("items").unwrap_or(&Value::Null))?;
    app.db.with(|c| {
        let building = trimmed(girdi, "building", 50)?;
        let floor = trimmed(girdi, "floor", 50)?;
        let room = trimmed(girdi, "room", 50)?;
        let (cafe, lines, totals) = price_cart(c, &items)?;
        let open: bool = c.query_row("SELECT open FROM cafe_accounts WHERE cafe = ?1", [&cafe], |r| Ok(r.get::<_, i64>(0)? != 0)).optional()?.unwrap_or(true);
        if !open {
            return Err(AppError::Conflict(format!("{cafe} şu an sipariş almıyor.")));
        }
        let name: String = c.query_row("SELECT name FROM users WHERE id = ?1", [uid], |r| r.get(0))?;
        let profile = json!({ "name": name, "building": building, "floor": floor, "room": room, "cafe": cafe });
        c.execute(
            "INSERT INTO customers (id, profile) VALUES (?1, ?2) ON CONFLICT(id) DO UPDATE SET profile = excluded.profile",
            params![customer_id(uid), profile.to_string()],
        )?;
        let stored: Vec<Value> = items.iter().map(|i| json!({ "productId": i.product_id, "size": i.size, "milk": i.milk, "quantity": i.quantity })).collect();
        for _ in 0..10 {
            let code = crate::siparis_olustur(); // KMP-XXXXXXXX (src-tauri/src/lib.rs)
            let taken: bool = c.query_row("SELECT EXISTS(SELECT 1 FROM orders WHERE code = ?1)", [&code], |r| r.get(0))?;
            if taken {
                continue;
            }
            let rec = json!({
                "code": code, "cafe": cafe, "customerId": customer_id(uid), "items": stored,
                "address": format!("{building} / {floor} / {room}"), "total": totals["total"], "discount": totals["discount"],
                "status": 0, "created": now_iso(c)?,
            });
            let order = crud::insert(c, &schema::ORDER, &rec)?;
            event(c, &code, None, "0", Some(uid), None)?;
            audit::record(c, Some(uid), "Order.place", "Order", &code, None, None, Some(&json!({ "lines": lines, "totals": totals })))?;
            return Ok(order);
        }
        Err(AppError::Conflict("Benzersiz sipariş kodu üretilemedi, tekrar dene.".into()))
    })
}

/// Durumu bir adım ilerletir (Hazırlanıyor → Kuryede → Teslim edildi). `Advance:Order` gerekir.
pub fn do_order_advance(app: &AppState, code: &str) -> AppResult<Value> {
    app.db.with(|c| {
        let uid = authorize(c, app.user(), "Advance:Order")?;
        let o = crud::find(c, &schema::ORDER, &json!(code))?;
        if !o["cancelled"].is_null() {
            return Err(AppError::Conflict("İptal edilen sipariş ilerletilemez.".into()));
        }
        let s = o["status"].as_i64().unwrap_or(0);
        if s >= 2 {
            return Err(AppError::Conflict("Sipariş zaten teslim edildi.".into()));
        }
        let sonra = crud::update(c, &schema::ORDER, &json!(code), &json!({ "status": s + 1 }))?;
        event(c, code, Some(s), &(s + 1).to_string(), Some(uid), None)?;
        if s + 1 == 2 {
            if let Some(cid) = o["courierId"].as_str() {
                c.execute("UPDATE couriers SET status = 'free', activeOrder = NULL WHERE id = ?1", [cid])?;
            }
        }
        audit::record(c, Some(uid), "Order.advance", "Order", code, None, Some(&o), Some(&sonra))?;
        Ok(sonra)
    })
}

/// İptal: müşteri yalnız "Hazırlanıyor" aşamasında kendi siparişini; `Cancel:Order` izni olan her zaman (teslimden önce). Neden zorunlu.
pub fn do_order_cancel(app: &AppState, code: &str, reason: &str) -> AppResult<Value> {
    let uid = app.user().ok_or(AppError::Unauthenticated)?;
    app.db.with(|c| {
        if reason.trim().is_empty() {
            return Err(AppError::validation("reason", "İptal nedeni zorunlu."));
        }
        let o = crud::find(c, &schema::ORDER, &json!(code))?;
        let owner = o["customerId"].as_str() == Some(customer_id(uid).as_str());
        let staff = rbac::can(c, ModelRef::user(uid), "Cancel:Order")?;
        if !(staff || owner) {
            return Err(AppError::Forbidden("Bu sipariş size ait değil.".into()));
        }
        let s = o["status"].as_i64().unwrap_or(0);
        if !o["cancelled"].is_null() || s >= 2 {
            return Err(AppError::Conflict("Teslim edilen ya da iptal edilmiş sipariş iptal edilemez.".into()));
        }
        if !staff && s != 0 {
            return Err(AppError::Conflict("Kurye yola çıktıktan sonra sipariş iptal edilemez.".into()));
        }
        let sonra = crud::update(c, &schema::ORDER, &json!(code), &json!({ "cancelled": { "reason": reason.trim(), "at": now_iso(c)? } }))?;
        event(c, code, Some(s), "cancelled", Some(uid), Some(reason.trim()))?;
        if let Some(cid) = o["courierId"].as_str() {
            c.execute("UPDATE couriers SET status = 'free', activeOrder = NULL WHERE id = ?1", [cid])?;
        }
        audit::record(c, Some(uid), "Order.cancel", "Order", code, Some(reason.trim()), Some(&o), Some(&sonra))?;
        Ok(sonra)
    })
}

/// Kurye ata (`Update:Order`): kurye boşta olmalı.
pub fn do_order_assign_courier(app: &AppState, code: &str, courier_id: &str) -> AppResult<Value> {
    app.db.with(|c| {
        let uid = authorize(c, app.user(), "Update:Order")?;
        let o = crud::find(c, &schema::ORDER, &json!(code))?;
        let k = crud::find(c, &schema::COURIER, &json!(courier_id))?;
        if k["status"] != json!("free") {
            return Err(AppError::Conflict("Kurye boşta değil.".into()));
        }
        if o["status"].as_i64().unwrap_or(0) >= 2 || !o["cancelled"].is_null() {
            return Err(AppError::Conflict("Bu sipariş için kurye atanamaz.".into()));
        }
        let sonra = crud::update(c, &schema::ORDER, &json!(code), &json!({ "courierId": courier_id }))?;
        c.execute("UPDATE couriers SET status = 'busy', activeOrder = ?1 WHERE id = ?2", params![code, courier_id])?;
        audit::record(c, Some(uid), "Order.assignCourier", "Order", code, None, Some(&o), Some(&sonra))?;
        Ok(sonra)
    })
}

/// İade kararı (`Update:Refund`)
pub fn do_refund_decide(app: &AppState, id: &str, approve: bool, note: Option<&str>) -> AppResult<Value> {
    app.db.with(|c| {
        let uid = authorize(c, app.user(), "Update:Refund")?;
        let r = crud::find(c, &schema::REFUND, &json!(id))?;
        if r["status"] != json!("open") {
            return Err(AppError::Conflict("Bu iade talebi zaten karara bağlanmış.".into()));
        }
        let sonra = crud::update(c, &schema::REFUND, &json!(id), &json!({ "status": if approve { "approved" } else { "rejected" } }))?;
        audit::record(c, Some(uid), "Refund.decide", "Refund", id, note, Some(&r), Some(&sonra))?;
        Ok(sonra)
    })
}

/// İndirim kuralını yayınlar (`Publish:DiscountRule`).
pub fn do_discount_publish(app: &AppState, version: i64) -> AppResult<Value> {
    app.db.with(|c| {
        let uid = authorize(c, app.user(), "Publish:DiscountRule")?;
        let once = crud::find(c, &schema::DISCOUNT_RULE, &json!(version))?;
        let rate = once["rate"].as_f64().unwrap_or(0.0);
        if !(0.0..=1.0).contains(&rate) || once["minQuantity"].as_i64().unwrap_or(0) < 1 {
            return Err(AppError::validation("rate", "Oran 0–1, eşik en az 1 olmalı."));
        }
        c.execute("UPDATE discount_rules SET status = 'published' WHERE version = ?1", [version])?;
        let sonra = crud::find(c, &schema::DISCOUNT_RULE, &json!(version))?;
        audit::record(c, Some(uid), "DiscountRule.publish", "DiscountRule", &version.to_string(), None, Some(&once), Some(&sonra))?;
        Ok(sonra)
    })
}

/// Panel özeti: sipariş sayısı, ciro, indirim toplamı, ortalama sepet, kafe payı, durumlar, en çok satan 5 ürün.
pub fn do_kampus_dashboard(app: &AppState, from: Option<&str>, to: Option<&str>) -> AppResult<Value> {
    app.db.with(|c| {
        authorize(c, app.user(), "ViewAny:Order")?;
        let (lo, hi) = (from.unwrap_or("0000"), to.unwrap_or("9999"));
        let (n, revenue, discount): (i64, f64, f64) = c.query_row(
            "SELECT COUNT(*), COALESCE(SUM(total), 0), COALESCE(SUM(discount), 0) FROM orders WHERE cancelled IS NULL AND created >= ?1 AND created <= ?2",
            params![lo, hi],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )?;
        let grup = |sql: &str| -> AppResult<Vec<Value>> {
            let mut st = c.prepare(sql)?;
            let rows = st.query_map(params![lo, hi], |r| Ok(json!({ "value": r.get::<_, String>(0)?, "count": r.get::<_, i64>(1)? })))?;
            Ok(rows.collect::<Result<Vec<_>, _>>()?)
        };
        let cafes = grup("SELECT cafe, COUNT(*) FROM orders WHERE cancelled IS NULL AND created >= ?1 AND created <= ?2 GROUP BY cafe ORDER BY 2 DESC")?;
        let status = grup("SELECT CAST(status AS TEXT), COUNT(*) FROM orders WHERE cancelled IS NULL AND created >= ?1 AND created <= ?2 GROUP BY status ORDER BY status")?;
        let top = grup(
            "SELECT p.name, SUM(json_extract(i.value, '$.quantity')) AS q FROM orders o, json_each(o.items) i
             JOIN products p ON p.id = json_extract(i.value, '$.productId')
             WHERE o.cancelled IS NULL AND o.created >= ?1 AND o.created <= ?2 GROUP BY p.id ORDER BY q DESC LIMIT 5",
        )?;
        Ok(json!({ "orders": n, "revenue": revenue, "discountTotal": discount, "averageBasket": if n == 0 { 0.0 } else { revenue / n as f64 }, "cafes": cafes, "statuses": status, "topProducts": top }))
    })
}

// ───────────────────────── Tauri komutları ─────────────────────────

#[tauri::command]
pub fn cart_totals(state: State<'_, AppState>, items: Value) -> AppResult<Value> {
    do_cart_totals(&state, &items)
}
#[tauri::command]
pub fn order_place(state: State<'_, AppState>, girdi: Value) -> AppResult<Value> {
    do_order_place(&state, &girdi)
}
#[tauri::command]
pub fn order_advance(state: State<'_, AppState>, code: String) -> AppResult<Value> {
    do_order_advance(&state, &code)
}
#[tauri::command]
pub fn order_cancel(state: State<'_, AppState>, code: String, reason: String) -> AppResult<Value> {
    do_order_cancel(&state, &code, &reason)
}
#[tauri::command]
pub fn order_assign_courier(state: State<'_, AppState>, code: String, courier_id: String) -> AppResult<Value> {
    do_order_assign_courier(&state, &code, &courier_id)
}
#[tauri::command]
pub fn refund_decide(state: State<'_, AppState>, id: String, approve: bool, note: Option<String>) -> AppResult<Value> {
    do_refund_decide(&state, &id, approve, note.as_deref())
}
#[tauri::command]
pub fn discount_publish(state: State<'_, AppState>, version: i64) -> AppResult<Value> {
    do_discount_publish(&state, version)
}
#[tauri::command]
pub fn kampus_dashboard(state: State<'_, AppState>, from: Option<String>, to: Option<String>) -> AppResult<Value> {
    do_kampus_dashboard(&state, from.as_deref(), to.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::boot_memory;
    use crate::backend::controllers as c;

    #[test]
    fn indirim_ve_birim_fiyat() {
        let kural = Rule { min_quantity: 5, rate: 0.1 };
        // Caramel Latte 120 + Büyük 15 + Yulaf sütü 10 = 145; 5 adet → 725; %10 indirim 72,50
        let u = unit_price(120.0, 15.0, 10.0);
        assert_eq!(u, 145.0);
        let t = totals_of(&[(u, 5)], &kural);
        assert_eq!((t["subtotal"].as_f64(), t["discount"].as_f64(), t["total"].as_f64()), (Some(725.0), Some(72.5), Some(652.5)));
        // 4 adet: indirim yok
        assert_eq!(totals_of(&[(95.0, 4)], &kural)["discount"].as_f64(), Some(0.0));
    }

    #[test]
    fn siparis_akisi() {
        let app = boot_memory().unwrap();
        c::auth_register(&app, "Yönetici", "y@ornek.com", "gizli-parola-1").unwrap();
        let yanlis = json!([{ "productId": 1, "size": "Standart", "milk": "Normal süt", "quantity": 1 }, { "productId": 3, "size": "Standart", "milk": "Sütsüz", "quantity": 1 }]);
        assert_eq!(do_cart_totals(&app, &yanlis).unwrap_err().code(), "validation"); // iki farklı kafe
        let sepet = json!([{ "productId": 24, "size": "Büyük", "milk": "Yulaf sütü", "quantity": 5 }]);
        assert_eq!(do_cart_totals(&app, &sepet).unwrap()["totals"]["total"], json!(652.5));
        // müşteri sipariş verir
        c::auth_logout(&app);
        c::auth_register(&app, "Müşteri", "m@ornek.com", "gizli-parola-2").unwrap();
        let o = do_order_place(&app, &json!({ "items": sepet, "building": "A Blok", "floor": "2", "room": "204" })).unwrap();
        let kod = o["code"].as_str().unwrap().to_string();
        assert!(kod.starts_with("KMP-"));
        assert_eq!(o["address"], json!("A Blok / 2 / 204"));
        assert_eq!(do_order_advance(&app, &kod).unwrap_err().code(), "forbidden"); // müşteri ilerletemez
        // müşteri kendi siparişini iptal edebilir (Hazırlanıyor)
        let iptal = do_order_cancel(&app, &kod, "vazgeçtim").unwrap();
        assert!(iptal["cancelled"]["reason"].is_string());
        assert_eq!(do_order_cancel(&app, &kod, "yine").unwrap_err().code(), "conflict");
        // adres eksik
        assert!(do_order_place(&app, &json!({ "items": sepet, "building": "", "floor": "2", "room": "204" })).is_err());
        // yönetici ilerletir
        let k2 = do_order_place(&app, &json!({ "items": sepet, "building": "B", "floor": "1", "room": "101" })).unwrap();
        let kod2 = k2["code"].as_str().unwrap().to_string();
        c::auth_logout(&app);
        c::auth_login(&app, "y@ornek.com", "gizli-parola-1").unwrap();
        assert_eq!(do_order_advance(&app, &kod2).unwrap()["status"], json!(1));
        assert_eq!(do_order_advance(&app, &kod2).unwrap()["status"], json!(2));
        assert_eq!(do_order_advance(&app, &kod2).unwrap_err().code(), "conflict");
        let d = do_kampus_dashboard(&app, None, None).unwrap();
        assert_eq!(d["orders"], json!(1)); // iptal edilen sayılmaz
    }
}
