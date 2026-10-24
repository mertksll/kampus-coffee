# Platform desteği

Bu tablo uygulama yolunu belirtir, beş cihazda test yapıldığı iddiası değildir.

| Özellik | Android | iOS | macOS | Windows | Linux | Web |
|---|---|---|---|---|---|---|
| Menü, arama, filtre, favori | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor |
| Sepet ve kişiselleştirme | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor |
| Demo takip kodu | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor | farklı yolla |
| Cihaz geçmişi/profil/tema | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor |
| Çok dilli bilgi/detay | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor | destekleniyor |

Native hedefler ortak Rust komutunu, web Web Crypto kullanır. localStorage WebView içinde kullanılır; depolama reddedilirse kayıt hatası bildirilir. Beş platform arasında desteklenmeyen ek bir cihaz özelliği kullanılmaz, gizlenecek platforma özel düğme yoktur. Gerçek ödeme, kurye, sunucu hesabı ve bildirim entegrasyonu hiçbir platformda sunulmaz.

`platform()` native derleme hedefini Rust üzerinden bildirir; webde `web`. `destekleniyorMu()` menü/sipariş/depolama özellikleri için merkezi destek sınırıdır. Cihaz testleri, dağıtım paketleri ve bilinmeyen statik rota davranışı ayrıca doğrulanmalıdır. Önceki Windows kanıtı eski sürüme aittir.

## Gerçek doğrulama
Linux cargo test (3 test) ve Chromium akışları [Actions 40096092271](https://github.com/mertksll/kampus-coffee/actions/runs/40096092271) üzerinde başarılıdır. Fiziksel beş cihaz testi yapılmış sayılmaz.
