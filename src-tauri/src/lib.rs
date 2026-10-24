mod orders;
#[tauri::command]
fn siparis_olustur(quantity: u32) -> Result<orders::OrderCode, orders::OrderError> { orders::create_order(quantity) }
#[tauri::command]
fn platform_bilgisi() -> &'static str { std::env::consts::OS }
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
 tauri::Builder::default()
  .plugin(tauri_plugin_opener::init())
  .invoke_handler(tauri::generate_handler![siparis_olustur,platform_bilgisi])
  .run(tauri::generate_context!())
  .expect("Kampus's Coffee başlatılamadı");
}
