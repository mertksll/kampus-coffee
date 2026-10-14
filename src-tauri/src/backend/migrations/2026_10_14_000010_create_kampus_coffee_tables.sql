-- kampus-coffee tabloları. Sütun adları ön yüz tipleriyle birebir (camelCase); tablo adları snake_case.

CREATE TABLE "brands" (
    "name" TEXT PRIMARY KEY,
    "image" TEXT NOT NULL,
    "label" TEXT NOT NULL,
    CHECK ("name" IN ('Starbucks', 'Kahve Dünyası', 'Espressolab'))
);

CREATE TABLE "products" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "cafe" TEXT NOT NULL CHECK ("cafe" IN ('Starbucks', 'Kahve Dünyası', 'Espressolab')),
    "name" TEXT NOT NULL,
    "kind" TEXT NOT NULL CHECK ("kind" IN ('Sıcak', 'Soğuk')),
    "price" REAL NOT NULL,
    "image" TEXT NOT NULL DEFAULT '',
    "description" TEXT NOT NULL DEFAULT '',
    "tag" TEXT NOT NULL DEFAULT '',
    "defaultMilk" TEXT NOT NULL DEFAULT 'Normal süt' CHECK ("defaultMilk" IN ('Normal süt', 'Yulaf sütü', 'Sütsüz')),
    "active" INTEGER NOT NULL DEFAULT 1 CHECK ("active" IN (0, 1))
);
CREATE INDEX products_cafe_index ON "products" ("cafe");

CREATE TABLE "cafe_accounts" (
    "cafe" TEXT PRIMARY KEY,
    "open" INTEGER NOT NULL DEFAULT 1 CHECK ("open" IN (0, 1)),
    "hours" TEXT NOT NULL DEFAULT '{"from":"08:00","to":"20:00"}',
    "contacts" TEXT NOT NULL DEFAULT '[]',
    "zones" TEXT NOT NULL DEFAULT '[]',
    CHECK ("cafe" IN ('Starbucks', 'Kahve Dünyası', 'Espressolab'))
);

CREATE TABLE "customers" (
    "id" TEXT PRIMARY KEY,
    "profile" TEXT NOT NULL,
    "phone" TEXT,
    "favorites" TEXT NOT NULL DEFAULT '[]',
    "status" TEXT NOT NULL DEFAULT 'active' CHECK ("status" IN ('active', 'blocked')),
    "createdAt" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE "couriers" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT NOT NULL,
    "status" TEXT NOT NULL DEFAULT 'free' CHECK ("status" IN ('free', 'busy', 'off')),
    "activeOrder" TEXT
);

CREATE TABLE "orders" (
    "code" TEXT PRIMARY KEY,
    "cafe" TEXT NOT NULL CHECK ("cafe" IN ('Starbucks', 'Kahve Dünyası', 'Espressolab')),
    "customerId" TEXT REFERENCES "customers" ("id") ON DELETE SET NULL,
    "items" TEXT NOT NULL,
    "address" TEXT NOT NULL,
    "total" REAL NOT NULL,
    "discount" REAL NOT NULL DEFAULT 0.0,
    "status" INTEGER NOT NULL DEFAULT 0 CHECK ("status" IN (0, 1, 2)),
    "created" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    "courierId" TEXT REFERENCES "couriers" ("id") ON DELETE SET NULL,
    "cancelled" TEXT
);
CREATE INDEX orders_status_index ON "orders" ("status");
CREATE INDEX orders_cafe_index ON "orders" ("cafe");
CREATE INDEX orders_created_index ON "orders" ("created");

CREATE TABLE "option_pricing" (
    "id" TEXT PRIMARY KEY,
    "size" TEXT NOT NULL,
    "milk" TEXT NOT NULL
);

CREATE TABLE "discount_rules" (
    "version" INTEGER PRIMARY KEY AUTOINCREMENT,
    "status" TEXT NOT NULL DEFAULT 'draft' CHECK ("status" IN ('draft', 'published')),
    "minQuantity" INTEGER NOT NULL,
    "rate" REAL NOT NULL,
    "cafes" TEXT NOT NULL DEFAULT '"all"',
    "validFrom" TEXT,
    "validTo" TEXT
);

CREATE TABLE "buildings" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT NOT NULL,
    "floors" TEXT NOT NULL DEFAULT '[]'
);

CREATE TABLE "order_events" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "orderCode" TEXT NOT NULL REFERENCES "orders" ("code") ON DELETE CASCADE,
    "from" INTEGER,
    "to" TEXT NOT NULL,
    "actorId" TEXT,
    "at" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    "reason" TEXT
);
CREATE INDEX order_events_orderCode_index ON "order_events" ("orderCode");

CREATE TABLE "refunds" (
    "id" TEXT PRIMARY KEY,
    "orderCode" TEXT NOT NULL REFERENCES "orders" ("code") ON DELETE CASCADE,
    "reason" TEXT NOT NULL,
    "amount" REAL NOT NULL,
    "status" TEXT NOT NULL DEFAULT 'open' CHECK ("status" IN ('open', 'approved', 'rejected')),
    "createdAt" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
