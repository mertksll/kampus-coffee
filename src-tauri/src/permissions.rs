//! İzin (yetki) yüzeyi — qLar deseninden damıtıldı.
//!
//! İki komut:
//! - `permissions_status`: izinlerin anlık durumunu döner ve **ASLA** sistem
//!   diyaloğu açmaz ("snapshot never prompts" ilkesi). Okunamayan durum
//!   `unknown`'dır, tahmin edilmez.
//! - `permission_request`: ilgili izin için OS çalışma zamanı diyaloğunu açar.
//!
//! Bu projede gerçek izin **bildirim** (notification): "Siparişiniz hazır"
//! bildirimi için. Durum/istek `tauri-plugin-notification` üzerinden gider ve
//! beş platformun hepsinde (Android, iOS, macOS, Windows, Linux) çalışır.

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_notification::{NotificationExt, PermissionState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionSnapshot {
    /// Kararlı kimlik: "notification" gibi.
    pub id: String,
    /// Kullanıcıya gösterilecek ad.
    pub title: String,
    /// "granted" | "denied" | "prompt" | "unsupported" | "unknown"
    pub status: String,
    /// Bu izin için çalışma zamanı isteği gösterilebilir mi.
    pub can_request: bool,
    /// Neden/nasıl kullanıldığının kısa açıklaması.
    pub notes: String,
}

fn notification_status(state: PermissionState) -> (&'static str, bool) {
    match state {
        PermissionState::Granted => ("granted", false),
        PermissionState::Denied => ("denied", false),
        _ => ("prompt", true),
    }
}

fn snapshot<R: tauri::Runtime>(app: &AppHandle<R>) -> Vec<PermissionSnapshot> {
    let (status, can_request) = match app.notification().permission_state() {
        Ok(state) => notification_status(state),
        Err(_) => ("unknown", false),
    };
    vec![PermissionSnapshot {
        id: "notification".into(),
        title: "Bildirimler".into(),
        status: status.into(),
        can_request,
        notes: "Siparişiniz hazır olduğunda size bildirim göndermek için.".into(),
    }]
}

/// Durumu döner; asla diyalog açmaz.
#[tauri::command]
pub fn permissions_status<R: tauri::Runtime>(app: AppHandle<R>) -> Vec<PermissionSnapshot> {
    snapshot(&app)
}

/// İlgili izin için OS diyaloğunu açar, sonra güncel durumu döner.
#[tauri::command]
pub fn permission_request<R: tauri::Runtime>(
    app: AppHandle<R>,
    id: String,
) -> Result<Vec<PermissionSnapshot>, String> {
    match id.as_str() {
        "notification" => {
            app.notification()
                .request_permission()
                .map_err(|e| e.to_string())?;
            Ok(snapshot(&app))
        }
        other => Err(format!("bilinmeyen izin: {other}")),
    }
}

/// Uygulamanın gerçek bildirim kullanımı: sipariş hazır bildirimi.
#[tauri::command]
pub fn bildirim_gonder<R: tauri::Runtime>(
    app: AppHandle<R>,
    baslik: String,
    govde: String,
) -> Result<(), String> {
    app.notification()
        .builder()
        .title(baslik)
        .body(govde)
        .show()
        .map_err(|e| e.to_string())
}
