//! Bu uygulamanın tabloları ve kaynak tanımları.
//! Sütun adları `src/` altındaki TypeScript tipleriyle ve tasarım belgelerindeki alan adlarıyla birebir aynıdır.
use crate::backend::crud::{col, req, Entity, Kind};
use crate::backend::error::{AppError, AppResult};
pub use crate::backend::schema_common::*;

/// Kayıt olan (ilk kullanıcı dışındaki) herkese verilen rol; `None` ise rolsüz kalır.
pub const DEFAULT_ROLE: Option<&str> = Some("customer");

pub static BRAND: Entity = Entity {
    resource: "Brand",
    table: "brands",
    pk: "name",
    pk_kind: Kind::Text,
    cols: &[
        req("image", Kind::Text),
        req("label", Kind::Text),
    ],
    search: &[],
    filters: &[],
    order_by: "\"name\" ASC",
    readonly: false,
};

pub static PRODUCT: Entity = Entity {
    resource: "Product",
    table: "products",
    pk: "id",
    pk_kind: Kind::Int,
    cols: &[
        req("cafe", Kind::Text),
        req("name", Kind::Text),
        req("kind", Kind::Text),
        req("price", Kind::Real),
        col("image", Kind::Text),
        col("description", Kind::Text),
        col("tag", Kind::Text),
        col("defaultMilk", Kind::Text),
        col("active", Kind::Bool),
    ],
    search: &["name", "tag"],
    filters: &["cafe", "kind", "active", "price"],
    order_by: "\"id\" ASC",
    readonly: false,
};

pub static CAFE_ACCOUNT: Entity = Entity {
    resource: "CafeAccount",
    table: "cafe_accounts",
    pk: "cafe",
    pk_kind: Kind::Text,
    cols: &[
        col("open", Kind::Bool),
        col("hours", Kind::Json),
        col("contacts", Kind::Json),
        col("zones", Kind::Json),
    ],
    search: &[],
    filters: &["open"],
    order_by: "\"cafe\" ASC",
    readonly: false,
};

pub static CUSTOMER: Entity = Entity {
    resource: "Customer",
    table: "customers",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("profile", Kind::Json),
        col("phone", Kind::Text),
        col("favorites", Kind::Json),
        col("status", Kind::Text),
        col("createdAt", Kind::Date),
    ],
    search: &["id"],
    filters: &["status"],
    order_by: "\"createdAt\" DESC",
    readonly: false,
};

pub static ORDER: Entity = Entity {
    resource: "Order",
    table: "orders",
    pk: "code",
    pk_kind: Kind::Text,
    cols: &[
        req("cafe", Kind::Text),
        col("customerId", Kind::Text),
        req("items", Kind::Json),
        req("address", Kind::Text),
        req("total", Kind::Real),
        col("discount", Kind::Real),
        col("status", Kind::Int),
        col("created", Kind::Date),
        col("courierId", Kind::Text),
        col("cancelled", Kind::Json),
    ],
    search: &["code", "address"],
    filters: &["cafe", "status", "customerId", "courierId", "created", "total"],
    order_by: "\"created\" DESC",
    readonly: false,
};

pub static OPTION_PRICING: Entity = Entity {
    resource: "OptionPricing",
    table: "option_pricing",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("size", Kind::Json),
        req("milk", Kind::Json),
    ],
    search: &[],
    filters: &[],
    order_by: "\"id\" ASC",
    readonly: false,
};

pub static DISCOUNT_RULE: Entity = Entity {
    resource: "DiscountRule",
    table: "discount_rules",
    pk: "version",
    pk_kind: Kind::Int,
    cols: &[
        col("status", Kind::Text),
        req("minQuantity", Kind::Int),
        req("rate", Kind::Real),
        col("cafes", Kind::Json),
        col("validFrom", Kind::Date),
        col("validTo", Kind::Date),
    ],
    search: &[],
    filters: &["status"],
    order_by: "\"version\" DESC",
    readonly: false,
};

pub static COURIER: Entity = Entity {
    resource: "Courier",
    table: "couriers",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("name", Kind::Text),
        col("status", Kind::Text),
        col("activeOrder", Kind::Text),
    ],
    search: &["name"],
    filters: &["status"],
    order_by: "\"name\" ASC",
    readonly: false,
};

pub static BUILDING: Entity = Entity {
    resource: "Building",
    table: "buildings",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("name", Kind::Text),
        col("floors", Kind::Json),
    ],
    search: &["name"],
    filters: &[],
    order_by: "\"name\" ASC",
    readonly: false,
};

pub static ORDER_EVENT: Entity = Entity {
    resource: "OrderEvent",
    table: "order_events",
    pk: "id",
    pk_kind: Kind::Int,
    cols: &[
        req("orderCode", Kind::Text),
        col("from", Kind::Int),
        req("to", Kind::Text),
        col("actorId", Kind::Text),
        col("at", Kind::Date),
        col("reason", Kind::Text),
    ],
    search: &[],
    filters: &["orderCode", "to", "at"],
    order_by: "\"at\" DESC",
    readonly: true,
};

pub static REFUND: Entity = Entity {
    resource: "Refund",
    table: "refunds",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("orderCode", Kind::Text),
        req("reason", Kind::Text),
        req("amount", Kind::Real),
        col("status", Kind::Text),
        col("createdAt", Kind::Date),
    ],
    search: &["orderCode", "reason"],
    filters: &["status", "orderCode"],
    order_by: "\"createdAt\" DESC",
    readonly: false,
};

pub static PROJECT: &[&Entity] = &[&BRAND, &PRODUCT, &CAFE_ACCOUNT, &CUSTOMER, &ORDER, &OPTION_PRICING, &DISCOUNT_RULE, &COURIER, &BUILDING, &ORDER_EVENT, &REFUND];

/// Bütün kaynaklar: ortak + bu uygulamanınkiler.
pub fn all() -> Vec<&'static Entity> {
    let mut v: Vec<&'static Entity> = vec![&CONTACT_MESSAGE, &CONTENT_PAGE, &AUDIT_LOG];
    v.extend(PROJECT.iter().copied());
    v
}

pub fn entity(resource: &str) -> AppResult<&'static Entity> {
    all().into_iter().find(|e| e.resource == resource)
        .ok_or_else(|| AppError::NotFound(format!("`{resource}` adında bir kaynak yok.")))
}
