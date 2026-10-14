# Kampus's Coffee — Geçici Yerel Arka Uç (SQLite)

> **Durum:** Sunucu yok. Veri, uygulamanın kendi veri klasöründeki bir **SQLite** dosyasında (`app.sqlite`) tutulur; kod `src-tauri/src/backend/` altındadır ve Tauri komutları olarak dışarı açılır. Sunucuya geçilince yalnız bu klasörün içi değişir; ekranların kullandığı komut adları ve veri biçimi aynı kalacak şekilde tasarlandı.

Rol ve izin düzeni, eğitmenin Laravel projelerindeki `spatie/laravel-permission` (+ Filament Shield) mantığındadır.

## 1. Laravel karşılıkları

| Laravel | Bu depoda |
|---|---|
| `database/migrations` | `src-tauri/src/backend/migrations/*.sql` + `migrations.rs`; uygulama açılışında bekleyenler çalışır (`migrations` tablosu: id, migration, batch) |
| `spatie/laravel-permission` | `rbac.rs`: roller, izinler, `assign_role`, `sync_roles`, `give_permission_to`, `can` … (tablolar Spatie ile aynı) |
| Filament Shield izin adları | `Eylem:Kaynak` (örn. `ViewAny:Order`), eylemler: ViewAny, View, Create, Update, Delete, DeleteAny |
| Süper yönetici (`Gate::before`) | `admin` rolü her izin denetimini geçer |
| Policy / `authorize()` | `policy.rs`; her denetleyici işlevi önce izni denetler |
| Resource controller | `controllers.rs` → `crud_index/show/store/update/destroy` (her kaynak için) |
| `spatie/laravel-activitylog` | `audit.rs` → `audit_logs` tablosu (kim, ne, önce/sonra, neden) |
| `artisan qtoefl:admin` | `user_store` (kullanıcıyı oluşturur/günceller ve rol atar) |
| Eloquent modeli | `schema.rs` içindeki `Entity` tanımı (sütun adları ön yüz tipleriyle aynı) |
| Form request doğrulaması | `crud.rs` tür/zorunluluk denetimi + `domain.rs` iş kuralı doğrulamaları → `{ code: "validation", errors: [{ field, message }] }` |

## 2. Veritabanı dosyası ve ilk açılış

- Dosya: uygulama veri klasörü / `app.sqlite` (Tauri `app_data_dir`). Depoya girmez.
- Açılışta migration'lar sırayla çalışır; roller, izinler ve başlangıç verisi **veri migration'larıyla** gelir (seeder gerekmez).
- **İlk kayıt olan kullanıcı `admin` olur.** Sonraki kayıtlar `customer` rolünü alır.

## 3. Tablolar

Ortak tablolar: `users`, `roles`, `permissions`, `model_has_roles`, `model_has_permissions`, `role_has_permissions`, `contact_messages`, `content_pages`, `audit_logs`.

| Kaynak (izin adı) | Tablo | Birincil anahtar | Sütunlar |
|---|---|---|---|
| `Brand` | `brands` | `name` (metin) | image*, label* |
| `Product` | `products` | `id` (otomatik artan) | cafe*, name*, kind*, price*, image, description, tag, defaultMilk, active |
| `CafeAccount` | `cafe_accounts` | `cafe` (metin) | open, hours, contacts, zones |
| `Customer` | `customers` | `id` (metin) | profile*, phone, favorites, status, createdAt |
| `Order` | `orders` | `code` (metin) | cafe*, customerId, items*, address*, total*, discount, status, created, courierId, cancelled |
| `OptionPricing` | `option_pricing` | `id` (metin) | size*, milk* |
| `DiscountRule` | `discount_rules` | `version` (otomatik artan) | status, minQuantity*, rate*, cafes, validFrom, validTo |
| `Courier` | `couriers` | `id` (metin) | name*, status, activeOrder |
| `Building` | `buildings` | `id` (metin) | name*, floors |
| `OrderEvent` | `order_events` | `id` (otomatik artan) | orderCode*, from, to*, actorId, at, reason |
| `Refund` | `refunds` | `id` (metin) | orderCode*, reason*, amount*, status, createdAt |

`*` zorunlu alan. JSON türündeki sütunlar (liste/nesne) metin olarak saklanır, okunurken nesneye çevrilir. Sütun adları, ön yüz tipleriyle birebir eşleşir.

## 4. Roller ve izinler

`admin` süper yöneticidir (atama gerekmez). Diğer roller aşağıdaki izinleri alır; `User`, `Role`, `Permission` kaynakları yalnız `admin`dedir.

| Rol | İzinler |
|---|---|
| `admin` | tümü (süper yönetici) |
| `cafe_manager` | `Product` (tümü); `Order` (tümü); `Courier` (tümü); `Building` (tümü); `CafeAccount` (tümü); `Customer` (okuma); `OptionPricing` (okuma); `DiscountRule` (okuma); `Brand` (okuma); `OrderEvent` (okuma); `Refund` (okuma); `Advance:Order`; `Cancel:Order` |
| `courier` | `Order` (okuma); `OrderEvent` (okuma); `Advance:Order` |
| `support` | `ContactMessage` (tümü); `Refund` (tümü); `Order` (okuma); `Customer` (okuma); `OrderEvent` (okuma) |
| `content` | `ContentPage` (tümü) |
| `viewer` | `Brand` (okuma); `Product` (okuma); `CafeAccount` (okuma); `Customer` (okuma); `Order` (okuma); `OptionPricing` (okuma); `DiscountRule` (okuma); `Courier` (okuma); `Building` (okuma); `OrderEvent` (okuma); `Refund` (okuma); `ContactMessage` (okuma); `ContentPage` (okuma) |
| `customer` | (uygulama kullanıcısı: kaynak izni yok; kendi kayıtlarını ilgili iş işleviyle yönetir) |

Özel yetenekler: `Advance:Order`, `Cancel:Order`, `Publish:DiscountRule`, `Reveal:PII`.

Yeni izin/rol eklemek için yeni bir veri migration'ı yazılır (`INSERT OR IGNORE`); mevcut migration değiştirilmez.

## 5. Komutlar

Rust'taki `snake_case` parametreler JS tarafında `camelCase` yazılır. Aşağıdaki parametre adları JS adlarıdır.

### Ortak

| Komut | Parametreler | Ne yapar |
|---|---|---|
| `auth_register` | name, email, password | Kayıt (ilk kullanıcı admin olur; diğerleri varsayılan rolü alır) |
| `auth_login` | email, password | Giriş |
| `auth_logout` | — | Çıkış |
| `auth_me` | — | Oturumdaki kullanıcı + roller + izinler (yoksa null) |
| `user_index` | — | Kullanıcı listesi (`ViewAny:User`) |
| `user_store` | name, email, role, password? | Kullanıcı oluştur/güncelle + rol ata (`Create:User`) |
| `user_sync_roles` | userId, roles[] | Rolleri eşitle (`Update:User`) |
| `user_sync_permissions` | userId, permissions[] | Doğrudan izinleri eşitle (`Update:User`) |
| `user_set_status` | userId, status, reason? | active / suspended / banned (`Update:User`) |
| `role_index` | — | Roller + izinleri (`ViewAny:Role`) |
| `role_store` | name, permissions?[] | Rol oluştur (`Create:Role`) |
| `role_update` | name, newName | Rol adını değiştir (`Update:Role`) |
| `role_destroy` | name, reason? | Rol sil (`Delete:Role`) |
| `role_sync_permissions` | name, permissions[] | Rolün izinlerini eşitle (`Update:Role`) |
| `permission_index` | — | İzin listesi (`ViewAny:Permission`) |
| `permission_generate` | extraResources?[] | Kaynaklar için izinleri üret — `shield:generate` (`Create:Permission`) |
| `crud_resources` | — | Kaynak tanımları (tablo, sütun, süzgeç) |
| `crud_index` | resource, query? | Liste: arama + süzgeç + sıralama + sayfalama (`ViewAny:<Kaynak>`) |
| `crud_show` | resource, id | Tek kayıt (`View`) |
| `crud_store` | resource, data | Oluştur (`Create`) |
| `crud_update` | resource, id, data | Kısmi güncelle (`Update`) |
| `crud_destroy` | resource, id, reason? | Sil (`Delete`) |
| `crud_destroy_many` | resource, ids[], reason? | Toplu sil (`DeleteAny`) |
| `crud_aggregate` | resource, groupBy, query? | Gruplu sayım (panel grafikleri) |
| `db_migrate_status` | — | Migration durumu (yalnız admin) |
| `contact_submit` | name, email, subject?, message, lang? | İletişim formu (oturum gerekmez) |

### Bu uygulamaya özel

| Komut | Parametreler | Ne yapar |
|---|---|---|
| `cart_totals` | items | Sepet toplamı (oturum gerekmez): `items` → `{ cafe, lines, totals }` |
| `order_place` | girdi | Sipariş ver. `girdi`: `{ items, building, floor, room }` — kod: `KMP-XXXXXXXX`. |
| `order_advance` | code | Durumu bir adım ilerletir (Hazırlanıyor → Kuryede → Teslim edildi). `Advance:Order` gerekir. |
| `order_cancel` | code, reason | İptal: müşteri yalnız "Hazırlanıyor" aşamasında kendi siparişini; `Cancel:Order` izni olan her zaman (teslimden önce). Neden zorunlu. |
| `order_assign_courier` | code, courierId | Kurye ata (`Update:Order`): kurye boşta olmalı. |
| `refund_decide` | id, approve, note | İade kararı (`Update:Refund`) |
| `discount_publish` | version | İndirim kuralını yayınlar (`Publish:DiscountRule`). |
| `kampus_dashboard` | from, to | Panel özeti: sipariş sayısı, ciro, indirim toplamı, ortalama sepet, kafe payı, durumlar, en çok satan 5 ürün. |

## 6. Ön yüzden çağırma

```ts
import { invoke } from "@tauri-apps/api/core";

await invoke("auth_login", { email, password });
const sayfa = await invoke("crud_index", { resource: "Brand", query: { page: 1, pageSize: 25, search: "", filters: {}, sort: { key: "name", dir: "desc" } } });
// sayfa = { items, total, page, pageSize }
```

Hatalar `{ code, message, errors? }` olarak döner. `code`: `unauthenticated` (oturum yok), `forbidden` (izin yok), `not_found`, `validation` (`errors[]` alan bazlı), `conflict`, `db`, `internal`.

Süzgeç anahtarı bir sütun adıysa tam eşleşme, dizi ise `IN`; `<sütun>Min` / `<sütun>Max` aralık verir (yalnız `filters` listesindeki sütunlar).

## 7. Yeni tablo / kaynak ekleme

1. `migrations/` altına yeni SQL dosyası (`2026_MM_GG_NNNNNN_ad.sql`); `migrations.rs` listesinin **sonuna** ekle.
2. `schema.rs` içine `Entity` tanımı ekle ve `PROJECT` listesine koy (sütun adları TypeScript tipiyle aynı).
3. İzinler için yeni bir veri migration'ı: `INSERT OR IGNORE INTO permissions …` ve rol atamaları.
4. İş kuralı gerekiyorsa `domain.rs` içine işlev + `#[tauri::command]` sarmalayıcı yaz, `lib.rs` `generate_handler!` listesine ekle.
5. `cargo test` (`src-tauri/` içinde).

## 8. Testler

`cd src-tauri && cargo test` — bellek içi SQLite ile migration'ları, rol/izin mantığını, kaynak denetleyicisini, parola özetini ve bu uygulamanın iş kurallarını sınar.

## 9. Sınırlar (geçici çözüm)

- Tek kullanıcılı yerel oturum: giriş yapan kullanıcı uygulama belleğinde tutulur; uygulama kapanınca oturum biter.
- Veri cihazdadır; cihazlar arası eşitleme ve sunucu yoktur. Silinen uygulama verisiyle birlikte veritabanı da gider.
- Parolalar Argon2id ile özetlenir; yine de bu bir **eğitim ve geliştirme altyapısıdır**, gerçek kullanıcı verisi için denetlenmiş bir sunucu gerekir.
- İzin önbelleği yoktur; her denetim veritabanından okunur.
- Arama SQLite `LIKE` ile yapılır (Türkçe büyük/küçük harf duyarlılığı sınırlıdır).

