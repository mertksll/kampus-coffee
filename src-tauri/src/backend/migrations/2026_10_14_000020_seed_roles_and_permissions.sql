-- Roller ve izinler (spatie/laravel-permission + Filament Shield düzeni). Veri-yalnız ve yinelenebilir.

INSERT OR IGNORE INTO roles (name, guard_name, created_at, updated_at) VALUES
    ('admin', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('cafe_manager', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('courier', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('support', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('content', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('viewer', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('customer', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'));

WITH actions(action) AS (VALUES ('ViewAny'), ('View'), ('Create'), ('Update'), ('Delete'), ('DeleteAny')),
     resources(resource) AS (VALUES ('Brand'), ('Product'), ('CafeAccount'), ('Customer'), ('Order'), ('OptionPricing'), ('DiscountRule'), ('Courier'), ('Building'), ('OrderEvent'), ('Refund'), ('ContactMessage'), ('ContentPage'), ('AuditLog'), ('User'), ('Role'), ('Permission'))
INSERT OR IGNORE INTO permissions (name, guard_name, created_at, updated_at)
SELECT action || ':' || resource, 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now') FROM actions, resources;

INSERT OR IGNORE INTO permissions (name, guard_name, created_at, updated_at) VALUES
    ('Advance:Order', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('Cancel:Order', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('Publish:DiscountRule', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('Reveal:PII', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'));

-- cafe_manager
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'cafe_manager' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:Product', 'View:Product', 'Create:Product', 'Update:Product',
    'Delete:Product', 'DeleteAny:Product', 'ViewAny:Order', 'View:Order',
    'Create:Order', 'Update:Order', 'Delete:Order', 'DeleteAny:Order',
    'ViewAny:Courier', 'View:Courier', 'Create:Courier', 'Update:Courier',
    'Delete:Courier', 'DeleteAny:Courier', 'ViewAny:Building', 'View:Building',
    'Create:Building', 'Update:Building', 'Delete:Building', 'DeleteAny:Building',
    'ViewAny:CafeAccount', 'View:CafeAccount', 'Create:CafeAccount', 'Update:CafeAccount',
    'Delete:CafeAccount', 'DeleteAny:CafeAccount', 'ViewAny:Customer', 'View:Customer',
    'ViewAny:OptionPricing', 'View:OptionPricing', 'ViewAny:DiscountRule', 'View:DiscountRule',
    'ViewAny:Brand', 'View:Brand', 'ViewAny:OrderEvent', 'View:OrderEvent',
    'ViewAny:Refund', 'View:Refund', 'Advance:Order', 'Cancel:Order'
);

-- courier
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'courier' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:Order', 'View:Order', 'ViewAny:OrderEvent', 'View:OrderEvent',
    'Advance:Order'
);

-- support
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'support' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:ContactMessage', 'View:ContactMessage', 'Create:ContactMessage', 'Update:ContactMessage',
    'Delete:ContactMessage', 'DeleteAny:ContactMessage', 'ViewAny:Refund', 'View:Refund',
    'Create:Refund', 'Update:Refund', 'Delete:Refund', 'DeleteAny:Refund',
    'ViewAny:Order', 'View:Order', 'ViewAny:Customer', 'View:Customer',
    'ViewAny:OrderEvent', 'View:OrderEvent'
);

-- content
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'content' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:ContentPage', 'View:ContentPage', 'Create:ContentPage', 'Update:ContentPage',
    'Delete:ContentPage', 'DeleteAny:ContentPage'
);

-- viewer
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'viewer' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:Brand', 'View:Brand', 'ViewAny:Product', 'View:Product',
    'ViewAny:CafeAccount', 'View:CafeAccount', 'ViewAny:Customer', 'View:Customer',
    'ViewAny:Order', 'View:Order', 'ViewAny:OptionPricing', 'View:OptionPricing',
    'ViewAny:DiscountRule', 'View:DiscountRule', 'ViewAny:Courier', 'View:Courier',
    'ViewAny:Building', 'View:Building', 'ViewAny:OrderEvent', 'View:OrderEvent',
    'ViewAny:Refund', 'View:Refund', 'ViewAny:ContactMessage', 'View:ContactMessage',
    'ViewAny:ContentPage', 'View:ContentPage'
);
