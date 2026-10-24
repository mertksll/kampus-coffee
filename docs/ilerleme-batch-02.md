# Batch 02 - Hafta 4

24.10.2026. Görev 10-16 ve lat.md ayrı PR'larla sunuldu. Bu kayıt kod, PR ve otomatik doğrulama kanıtlarını ayırır. Öğrencinin elle denemesi yapılmış gibi gösterilmez.

| Görev | Uygulama ve kanıt | PR |
|---|---|---|
| 10 | Şartname, istem günlüğü, çalışma döngüsü | [#22](https://github.com/mertksll/kampus-coffee/pull/22) |
| 11 | Merkezi tipler ve sentetik örnekler, veri modeli | [#23](https://github.com/mertksll/kampus-coffee/pull/23) |
| 12 | Veriden bağımsız, klavyeyle erişilen Kart | [#24](https://github.com/mertksll/kampus-coffee/pull/24) |
| 13 | İki listede dört hal, dört dil, gerçek ekran görüntüleri | [#25](https://github.com/mertksll/kampus-coffee/pull/25) |
| 14 | Dört dilde 96 ürün rotası, filtreli dönüş, 404 | [#26](https://github.com/mertksll/kampus-coffee/pull/26) |
| 15 | Tipli Rust Result, native.ts, platform tablosu, üç Rust testi | [#27](https://github.com/mertksll/kampus-coffee/pull/27) |
| 16 | Pencere alt sınırları, 0.2.0, otomatik doğrulama ve bu kayıt | [#28](https://github.com/mertksll/kampus-coffee/pull/28) |
| Ek | lat.md proje haritası ve tek ajan kural kaynağı | [#29](https://github.com/mertksll/kampus-coffee/pull/29) |

## Doğrulama

- Her görev kendi dalında `bun run check`, `bun run build` ve iş kuralı testlerinden geçirildi.
- Son sürüm: tip denetimi 0 hata/uyarı; 12 birim testi başarılı; 127 statik sayfa.
- Gerçek Chromium: dört liste hali, tekrar dene, arama temizleme, Enter ile detay, filtreli geri dönüş, doğrudan adres, 404, dört dil ve dar RTL için 3 senaryo başarılı.
- Rust: Linux üzerinde `cargo +stable test --manifest-path src-tauri/Cargo.toml --locked`, 3 test başarılı.
- 96 ürün HTML çıktısında dil, yön ve başlık denetimi başarılı.
- lat.md: `lat check` temiz. Kurulum sürümü kaydı korunur, geçici ayrıştırma önbelleği commit edilmez.

Son web kanıtı: [Actions run 40096092271](https://github.com/mertksll/kampus-coffee/actions/runs/40096092271). Aynı son koşuda Rust işi de başarılıdır; ilk başarılı tam koşunun kısa çıktısı da kanıt dizininde korunur. Yerel kontrol çıktıları [kanıt dizininde](kanit/batch-02/); dört hal ve RTL görüntüleri [Görev 13 günlüğünde](istemler/13-liste-ve-uc-durum.md) ve [Görev 14 günlüğünde](istemler/14-detay-ekrani.md). PNG asılları Actions artifact'ında, aynı ekranların okunabilir JPEG kopyaları repodadır.

## Öğrenci kontrolü ve teslim

- Kendi cihazında `bun run tauri dev` ile sipariş ve kalıcı kayıt akışını bir kez dene. Beş gerçek cihazda doğrulama yapılmış değildir; Linux cargo testi Windows/macOS/Android/iOS çalıştırma kanıtı yerine geçmez.
- Görev 10'daki yeni sohbet deneyi ve öğrencinin elle kontrolü kullanıcı tarafından tamamlanır.
- Blackboard yükleme/gönderimini öğrenci yapar; ajan teslim etmez veya öğretmene mesaj göndermez.
- `v0.2.0-batch-02` etiketi PR'lar tamamlandıktan sonra doğrulanmış master kaynak görüntüsüne konur. Önceki batch etiketi taşınmaz.

Öğretmenin SQLite PR #21'i ve ayrı yönetim paneli görevi bu Hafta 4 uygulamasına dahil edilmedi. Yüklenen durum PDF'si ve .env repoya eklenmedi. Kaynak metindeki öğrenci numarası kaldırıldı; eski Git geçmişi yeniden yazılmadı.
