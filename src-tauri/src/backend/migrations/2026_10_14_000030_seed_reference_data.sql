-- Başlangıç (referans) verisi. Yinelenebilir: INSERT OR IGNORE.

-- brands
INSERT OR IGNORE INTO "brands" ("name", "image", "label") VALUES ('Starbucks', '/brands/starbucks.png', 'Sevdiğin kahve klasikleri');
INSERT OR IGNORE INTO "brands" ("name", "image", "label") VALUES ('Kahve Dünyası', '/brands/kahvedunyasi.png', 'Tanıdık lezzetler, güzel molalar');
INSERT OR IGNORE INTO "brands" ("name", "image", "label") VALUES ('Espressolab', '/brands/espressolab.png', 'Espressonun farklı tonları');

-- cafe_accounts
INSERT OR IGNORE INTO "cafe_accounts" ("cafe", "open", "hours", "contacts", "zones") VALUES ('Starbucks', 1, '{"from":"08:00","to":"20:00"}', '[]', '[]');
INSERT OR IGNORE INTO "cafe_accounts" ("cafe", "open", "hours", "contacts", "zones") VALUES ('Kahve Dünyası', 1, '{"from":"08:00","to":"20:00"}', '[]', '[]');
INSERT OR IGNORE INTO "cafe_accounts" ("cafe", "open", "hours", "contacts", "zones") VALUES ('Espressolab', 1, '{"from":"08:00","to":"20:00"}', '[]', '[]');

-- products
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (1, 'Starbucks', 'Caffè Latte', 'Sıcak', 95, '/coffee/1.webp', '', 'Yumuşak içim', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (2, 'Starbucks', 'Iced Americano', 'Soğuk', 85, '/coffee/2.webp', '', 'Sade & ferah', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (3, 'Kahve Dünyası', 'Türk Kahvesi', 'Sıcak', 65, '/coffee/3.webp', '', 'Klasik lezzet', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (4, 'Kahve Dünyası', 'Soğuk Latte', 'Soğuk', 90, '/coffee/4.webp', '', 'Serin bir mola', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (5, 'Espressolab', 'Flat White', 'Sıcak', 100, '/coffee/5.webp', '', 'Yoğun & dengeli', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (6, 'Espressolab', 'Cold Brew', 'Soğuk', 105, '/coffee/6.webp', '', 'Soğuk demleme', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (7, 'Starbucks', 'Cappuccino', 'Sıcak', 95, '/coffee/7.webp', '', 'Bol köpüklü', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (8, 'Starbucks', 'Caramel Macchiato', 'Sıcak', 115, '/coffee/8.webp', '', 'Karamel dokunuşu', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (9, 'Starbucks', 'Caffè Mocha', 'Sıcak', 110, '/coffee/9.webp', '', 'Çikolatalı', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (10, 'Starbucks', 'Iced Vanilla Latte', 'Soğuk', 110, '/coffee/10.webp', '', 'Vanilyalı', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (11, 'Starbucks', 'Espresso', 'Sıcak', 70, '/coffee/11.webp', '', 'Yoğun espresso', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (12, 'Starbucks', 'Iced Caramel Latte', 'Soğuk', 115, '/coffee/12.webp', '', 'Karamelli', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (13, 'Kahve Dünyası', 'Damla Sakızlı Türk Kahvesi', 'Sıcak', 80, '/coffee/13.webp', '', 'Aromatik', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (14, 'Kahve Dünyası', 'Cappuccino', 'Sıcak', 85, '/coffee/14.webp', '', 'Bol köpüklü', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (15, 'Kahve Dünyası', 'Caffè Latte', 'Sıcak', 85, '/coffee/15.webp', '', 'Yumuşak içim', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (16, 'Kahve Dünyası', 'Filtre Kahve', 'Sıcak', 75, '/coffee/16.webp', '', 'Sade kahve', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (17, 'Kahve Dünyası', 'Fındıklı Soğuk Latte', 'Soğuk', 100, '/coffee/17.webp', '', 'Fındık aromalı', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (18, 'Kahve Dünyası', 'Buzlu Mocha', 'Soğuk', 105, '/coffee/18.webp', '', 'Çikolatalı', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (19, 'Espressolab', 'Cortado', 'Sıcak', 90, '/coffee/19.webp', '', 'Dengeli', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (20, 'Espressolab', 'Americano', 'Sıcak', 85, '/coffee/20.webp', '', 'Sade kahve', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (21, 'Espressolab', 'Caffè Latte', 'Sıcak', 100, '/coffee/21.webp', '', 'Yumuşak içim', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (22, 'Espressolab', 'Iced Latte', 'Soğuk', 105, '/coffee/22.webp', '', 'Serin bir mola', 'Normal süt', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (23, 'Espressolab', 'Iced Americano', 'Soğuk', 90, '/coffee/23.webp', '', 'Sade & ferah', 'Sütsüz', 1);
INSERT OR IGNORE INTO "products" ("id", "cafe", "name", "kind", "price", "image", "description", "tag", "defaultMilk", "active") VALUES (24, 'Espressolab', 'Caramel Latte', 'Sıcak', 120, '/coffee/24.webp', '', 'Karamel dokunuşu', 'Normal süt', 1);

-- option_pricing
INSERT OR IGNORE INTO "option_pricing" ("id", "size", "milk") VALUES ('default', '{"Standart":0,"Büyük":15}', '{"Normal süt":0,"Yulaf sütü":10,"Sütsüz":0}');

-- discount_rules
INSERT OR IGNORE INTO "discount_rules" ("version", "status", "minQuantity", "rate", "cafes") VALUES (1, 'published', 5, 0.1, '"all"');

