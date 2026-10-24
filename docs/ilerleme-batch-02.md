# Batch 02 ilerleme - 24.10.2026

Görev 10-15, PR #22-27 ile master dalına birleştirildi. Görev 16 denetimi, lat.md PR'ı ve teslim etiketi bu kayıt yazılırken sürüyor; tamamlanmış cihaz testi iddiası değildir.

| Görev | Uygulanan | Kalan doğrulama |
|---|---|---|
| 10 | Şartname, günlük düzeni, çalışma döngüsü | Öğrencinin yeni sohbet testi ve PR |
| 11 | Merkezi tipler, açıklamalar, sentetik örnekler, veri modeli | Öğrencinin ekran karşılaştırması ve PR |
| 12 | Genel Kart ve CoffeeCard adaptörü | Klavye, uzun başlık, görselsiz, RTL ekran görüntüleri ve PR |
| 13 | Üç durum bileşeni; menü/siparişler; dört dil; dev senaryoları | Dört halin gerçek ekran görüntüleri, tekrar dene/arama deneyi ve PR |
| 14 | Dört dilde 96 ürün rotası; filtreli geri dönüş; 404 | Etkileşim, yeni pencere, native bilinmeyen adres ve PR |
| 15 | Tipli Rust Result; native.ts; web yedeği; üç Rust testi; platform tablosu | cargo test, Tauri çalıştırma, cihaz kontrolleri ve PR |
| 16 | minWidth=360/minHeight=640; sürüm 0.2.0; bu rapor | PR merge'leri ve doğrulanmış master üzerinde v0.2.0-batch-02 etiketi |

## Çalıştırılan kontroller

- `bun run check`: 0 hata, 0 uyarı, 0 ipucu; Svelte 0 hata/uyarı.
- `bun test`: 12 başarılı, 0 başarısız; 37 beklenti.
- `bun run build`: 127 statik sayfa, başarılı.
- Dört dilde 96 ürün HTML çıktısında dil, RTL/LTR ve dolu başlık doğrulandı; 404 mesajı mevcut.
- `lat check`: hazırlanan lat.md dosyaları üzerinde temiz. lat.md ayrı feature dalıyla sunulur.
- `git diff --check`: temiz.

Gerçek çıktılar [kanıt dizininde](kanit/batch-02/). Cargo bu ortamda kurulu değil: Rust testleri çalıştırılmadı. Tarayıcı dosya yükleme denemesi tamamlanmadı; terminal GitHub yazma kimliği yok. Tarayıcı etkileşimi ve ekran görüntüleri doğrulanmış değildir. Headless test tarayıcısı indirme denemesi geçerli arşiv vermedi; bu da başarılı kontrol sayılmaz.

## Teslim sırası

10, 11, 12, 13, 14, 15, 16 ve ek lat.md dalları yerel olarak ayrıldı; her biri ilgili istem günlüğünü içerir. Her görev kendi PR'ıyla master'a girmeli. Native kontrol ve öğrenci doğrulaması tamamlanmadan batch bitmiş sayılmamalı. Teslim etiketi yalnız son doğrulanmış master commitine konmalı; eski v0.1.0-batch-01 taşınmaz.

Öğretmenin SQLite PR #21'i birleştirilmedi. Yönetim paneli bu hafta 4 görevlerinin kapsamına eklenmedi; kendi görev belgesiyle ayrıca ele alınmalı. Bu rapor PDF'si repoya eklenmedi. Yeni içerikte gerçek öğrenci numarası, e-posta, token ve şifre kullanılmadı; mevcut README/proje belgelerindeki öğrenci numarası metinden çıkarıldı. Eski Git geçmişi yeniden yazılmadı.

## GitHub doğrulama

- [Görev 10 PR #22](https://github.com/mertksll/kampus-coffee/pull/22)
- [Görev 11 PR #23](https://github.com/mertksll/kampus-coffee/pull/23)
- [Görev 12 PR #24](https://github.com/mertksll/kampus-coffee/pull/24)
- [Görev 13 PR #25](https://github.com/mertksll/kampus-coffee/pull/25)
- [Görev 14 PR #26](https://github.com/mertksll/kampus-coffee/pull/26)
- [Görev 15 PR #27](https://github.com/mertksll/kampus-coffee/pull/27)

Batch 02 iş akışı frontend check/test/build, gerçek Chromium gezinme testleri ve cargo test çalıştırır. Ekran görüntüleri artifact olarak tutulur; başarılı sonuç alınmadan başarılı sayılmaz. İş akışı salt okuma izni kullanır; token veya kullanıcı verisi işlemez.
