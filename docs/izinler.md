# İzinler (yetkiler)

Bu uygulama yalnız gerçekten kullandığı izni ister. İlke: **açıklayamadığımız izni istemeyiz.**

İzin yüzeyi qLar deseninden damıtıldı: iki Rust komutu — `permissions_status` (durumu döner, **asla** diyalog açmaz) ve `permission_request` (OS diyaloğunu açar). Arayüz bunları yalnız `src/lib/native.ts` üzerinden çağırır. Ekran: [`/izinler`](../src/pages/izinler.astro).

## İstenen izinler

| İzin | Ne için | Hangi ekran | Veri nereye gider |
|---|---|---|---|
| Bildirim (notification) | "Siparişiniz hazır" bildirimi | /izinler (test), sipariş akışı | Hiçbir yere; bildirim yalnız cihazda gösterilir |

Başka hassas izin (konum, kamera, mikrofon, kişiler) **istenmez**: uygulamanın fikri bunları gerektirmiyor.

## Durum değerleri

`granted` (izin verildi) · `denied` (reddedildi) · `prompt` (henüz sorulmadı) · `unsupported` (bu platformda yok) · `unknown` (okunamadı — tahmin edilmez).

## Platform

Bildirim izni `tauri-plugin-notification` ile beş native hedefte (Android, iOS, macOS, Windows, Linux) çalışır. Tarayıcı önizlemesinde sistem izni yoktur; arayüz "uygulama içinde görünür" notunu gösterir. Ayrıntı: [platform-destegi.md](platform-destegi.md).

## Reddedilince

İzin reddedilirse uygulama çökmeden çalışır; bildirim gösterilmez, sipariş durumu ekranda yine görünür. Kalıcı rette kullanıcı cihaz ayarlarından izni açabilir.
