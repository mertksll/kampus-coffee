# Kampus's Coffee — Sayfa ve platform mimarisi

## Sayfa ve özellik ağacı

```text
Kampus's Coffee
├── /                         24 kahve; fotoğraf, kafe filtresi, arama, favori ve fiyat sıralaması
├── /urun/1/ … /urun/24/        Kahve detayı, boyut/süt/adet, sepete ekleme
├── /sepet/                   Tek kafe, adet düzenleme, örnek sınıf, indirim ve demo onayı
├── /siparisler/              Aktif/tamamlanan filtre, durum adımları, detay ve tekrar sipariş
├── /profil/                  İsim/kafe/sınıf tercihleri, favoriler, istatistik, geçmiş silme
├── /hakkinda/                TR MDX, kurumsal tanıtım ve React proje bilgileri
├── /iletisim/                TR Astro + Svelte form
├── /kosullar/                TR MDX
├── /gizlilik/                TR MDX
├── /en/hakkinda/             EN MDX
├── /en/iletisim/             EN Astro + Svelte
├── /en/kosullar/             EN MDX
├── /en/gizlilik/             EN MDX
├── /ar/hakkinda/             AR MDX, RTL
├── /ar/iletisim/             AR Astro + Svelte, RTL
├── /ar/kosullar/             AR MDX, RTL
├── /ar/gizlilik/             AR MDX, RTL
├── /fa/hakkinda/             FA MDX, RTL
├── /fa/iletisim/             FA Astro + Svelte, RTL
├── /fa/kosullar/             FA MDX, RTL
├── /fa/gizlilik/             FA MDX, RTL
├── /rehber/                  TR MDX, sipariş rehberi
├── /en/rehber/               EN MDX, sipariş rehberi
├── /ar/rehber/               AR MDX, sipariş rehberi, RTL
└── /fa/rehber/               FA MDX, sipariş rehberi, RTL
```

Toplam 48 statik rota: 4 ana sayfa, 24 ürün, 20 bilgi sayfası. Bütün bilgi sayfaları alt bilgiden erişilir; profil ayrıca hakkında, iletişim, koşullar ve gizlilik bağlantılarını sunar; dil seçici aynı bilgi sayfasının çevirisine gider. Menü ve sipariş akışı bu aşamada Türkçedir. Sayfalar standart bağlantılarla açılır; sepet ve geçmiş istemcide yüklenir. Tema tüm sayfalarda ortaktır.

Akış: keşfet → kahve seçenekleri → tek kafeli sepet → örnek sınıf ve demo onayı → sipariş geçmişi. Tauri içinde Rust `siparis_olustur`; webde Web Crypto ile demo kodu. Mevcut cihaz geçmişiyle kod çakışması denetlenir; bu kod güvenlik token'ı değildir.

## Hedef platformlar

| Platform | Sistem | Hedef çıktı | Gereken araçlar / durum |
|---|---|---|---|
| macOS | Apple Silicon / Intel | .dmg, .app | macOS + Xcode araçları; native test bekliyor |
| Windows | 10 / 11 x64 | .msi, .exe | Rust MSVC, C++ Build Tools, WebView2; Windows x64 debug derlemesi ve süreç başlangıcı doğrulandı, dağıtım paketi bekliyor |
| Linux | Ubuntu / Debian | .deb, .AppImage | Rust, WebKitGTK ve sistem paketleri; native test bekliyor |
| iOS | iPhone / iPad | .ipa | macOS, Xcode, imzalama; native test bekliyor |
| Android | Telefon / tablet | .apk, .aab | Android Studio, SDK/NDK/JDK, Rust target; native test bekliyor |

Beş platform için ikon kaynakları bulunur; ikon üretimi native paketlerin oluşturulduğu anlamına gelmez. Web statik derlemesi teslim doğrulamasının temelidir.

## Duyarlı ve uyarlanabilir düzen

Bu tablo ana kahve listesindeki `.coffee-grid`, `.app-page` ve `.app-nav` kurallarını açıklar. Eşikler `src/styles/app.css` ile eşleştirilmiştir.

| Ekran | CSS aralığı | Kahve listesi ve yerleşim | Gezinme |
|---|---|---|---|
| Dar telefon | <480 px (özellikle 375–430) | Tek sütun; tek panelli hero; yatay kaydırılan kafe seçimi | Sabit alt menü |
| Geniş telefon | 480–767 px | İki sütun; hero ve detay ekranı tek panel | Sabit alt menü |
| Tablet | 768–1023 px | İki sütun; ürün/sepet detayında iki panel | Sabit alt menü |
| Geniş tablet / küçük masaüstü | 1024–1199 px | İki sütun; ürün/sepet detayında iki panel | Üst başlık içinde menü |
| Masaüstü | 1200–1599 px | Üç sütun; ortalanmış `max-width: 1240px` | Üst başlık içinde menü |
| Büyük ekran | ≥1600 px | Dört sütun; ortalanmış `max-width: 1440px` | Üst başlık içinde menü |

`.app-page` için temel üst genişlik sınırı 1240 px, 1600 px ve üzerindeki ekranlarda 1440 px'dir. Küçük ekranlarda kullanılabilir genişliğe sığar; telefon iç boşluğu 18 px, tablet iç boşluğu 24 px, masaüstü iç boşluğu 32 px, büyük ekran iç boşluğu 40 px'dir. Kartlar `minmax(0, 1fr)` sütunları kullanır.

Gezinme değişimi **1024 px**, kahve listesinin üç sütuna geçişi **1200 px** sınırındadır. Profil tercihleri sonraki sepetin teslimat alanlarını doldurur. RTL sayfalarında dil ve yön özellikleri bulunur; odak göstergesi, safe-area ve reduced-motion kuralları tanımlıdır.

375–1920 px arasındaki 13 genişlikte grid, gezinme ve maksimum genişlik kuralları [statik CSS denetiminde](kanit/responsive-audit.txt) doğrulandı. Bu denetim görsel taşma veya gerçek cihaz testi değildir. Windows uygulamasının gerçek görüntüsü [Tauri kanıtında](kanit/tauri-dev.png); diğer ekran boyutlarının görsel doğrulama sınırı [teslim belgesinde](teslim.md) açıklanır.

## Batch 02 rotaları
Menü: `/`, `/en/`, `/ar/`, `/fa/`. Ürün: `/urun/[id]/`, `/en/urun/[id]/`, `/ar/urun/[id]/`, `/fa/urun/[id]/`. Her dilde 24 statik ürün rotası; bilinmeyen adresler `404.astro` ekranına düşer. Tauri statik dosya sunucusunun bilinmeyen adres davranışı ayrıca cihazda doğrulanmalıdır. Menü arama/kafe/tür/sıralama/favori durumu URL sorgusuna yazılır ve detay bağlantısına taşınır; geri dönüş bu sorguyu kullanır.

Sipariş geçmişinin ortak durum mesajları ayrıca `/en/siparisler/`, `/ar/siparisler/`, `/fa/siparisler/` adreslerinde de kullanılır.
