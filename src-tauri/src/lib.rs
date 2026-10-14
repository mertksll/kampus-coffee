use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
static SEQUENCE: AtomicU32 = AtomicU32::new(0);

// Eğitim demosu için takip kodu. Kimlik doğrulama veya ödeme anahtarı değildir.
// İstemci, cihazdaki geçmişe göre olası çakışmayı kontrol edip yeniden dener.
#[tauri::command]
fn siparis_olustur() -> String {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!("KMP-{:08X}", (nanos as u32).wrapping_add(sequence))
}
mod backend;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| backend::commands::setup(app))
        .invoke_handler(tauri::generate_handler![
            siparis_olustur,
            backend::commands::auth_register,
            backend::commands::auth_login,
            backend::commands::auth_logout,
            backend::commands::auth_me,
            backend::commands::user_index,
            backend::commands::user_store,
            backend::commands::user_sync_roles,
            backend::commands::user_sync_permissions,
            backend::commands::user_set_status,
            backend::commands::role_index,
            backend::commands::role_store,
            backend::commands::role_update,
            backend::commands::role_destroy,
            backend::commands::role_sync_permissions,
            backend::commands::permission_index,
            backend::commands::permission_generate,
            backend::commands::crud_resources,
            backend::commands::crud_index,
            backend::commands::crud_show,
            backend::commands::crud_store,
            backend::commands::crud_update,
            backend::commands::crud_destroy,
            backend::commands::crud_destroy_many,
            backend::commands::crud_aggregate,
            backend::commands::db_migrate_status,
            backend::commands::contact_submit,
            backend::domain::cart_totals,
            backend::domain::order_place,
            backend::domain::order_advance,
            backend::domain::order_cancel,
            backend::domain::order_assign_courier,
            backend::domain::refund_decide,
            backend::domain::discount_publish,
            backend::domain::kampus_dashboard
        ])
        .run(tauri::generate_context!())
        .expect("Kampus's Coffee başlatılamadı");
}
