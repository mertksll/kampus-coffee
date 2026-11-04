# İstem günlüğü — Görev 31-36 (Yetkiler / İzinler)

## İstem
qLar (`agents-apps/qLar`) izin desenini örnek alarak uygulamaya izin altyapısı ekle:
`permissions_status` (asla diyalog açmaz) ve `permission_request` komutları; projenin gerçek izni
olarak **bildirim** (notification) — "siparişiniz hazır". Arayüzde /izinler ekranı; Rust çağrıları
yalnız native.ts üzerinden. En az yetki; başka izin istenmez.

## Plan (uygulandı)
- `src-tauri/src/permissions.rs`: `PermissionSnapshot`, `permissions_status`, `permission_request`, `bildirim_gonder`.
- `lib.rs`: `tauri-plugin-notification` plugin + komut kaydı.
- `Cargo.toml`: `tauri-plugin-notification = "2"`.
- `capabilities/default.json`: `notification:default`.
- `src/lib/types/izin.ts` + `src/lib/native.ts`: `izinDurumu`, `izinIste`, `bildirimGonder`.
- `src/pages/izinler.astro` + `src/components/Izinler.svelte` (Svelte 5 runes, onMount + try/catch).
- `docs/izinler.md`, `docs/platform-destegi.md` güncel; profil sayfasından bağlantı.

## Doğrulama
- `cargo check` / `bun run build` sonucu bu PR'da belirtildi.
- Canlı izin diyaloğu/bildirim cihaz veya masaüstü derlemesinde test edilmelidir; web önizlemede sistem izni yoktur.

## Not
Bu PR öğretim görevlisi tarafından başlangıç (scaffold) olarak gönderildi; öğrenci Files changed inceleyip
build sonrası merge eder (Görev 31-36). Geliştirme: ret akışı ekranları (Görev 34) ve çok dilli metin.
