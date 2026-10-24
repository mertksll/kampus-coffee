# Görev 16 - Batch denetimi
Araç/model: ChatGPT Work, Codex GPT-6. İstem: 4. hafta görevlerini uygula.
Plan: Pencere alt sınırlarını ekle, Svelte tip hatalarını gider, veri akışlarını gözden geçir ve gerçekten çalıştırılan kontrol çıktılarını kaydet.
Düzeltmeler: $props generic biçimi yerine tipli destructuring; geçersiz kimlikte ilk ürüne dönüş kaldırıldı; geçmiş hata denemesi kalıcı veriyi değiştirmez; hata bildirimi sipariş eylemini tekrar deneyebilir. Öğrenci numarası yeni kaynak metinden kaldırıldı.
Doğrulama: check 0 hata/uyarı; 12 test geçti; 127 statik sayfa; 96 ürünün dil/yön kontrolü geçti. Gerçek çıktı docs/kanit/batch-02/ içinde.
Kalan: Cargo, gerçek cihazlar, ekran kanıtları, PR/merge ve teslim etiketi. Bunlar yapılmış gibi gösterilmedi.

Bağlantı sonrası plan: Her görevin kendi dalında check/build/test yeniden çalıştırıldı. #22-27 birleştirildi. GitHub Actions üzerinde Cargo ve Playwright doğrulaması, filtreli gezinme ve dört halin gerçek ekran kanıtı için eklendi; sonuçlar henüz varsayılmaz.

İlk Actions web işi başarılı. Tekrar koşuda Astro geliştirme araç çubuğunun kendi h1 başlıkları testin genel h1 seçicisiyle çakıştı; seçici uygulama ana içeriğine daraltıldı. Uygulama davranışında hata gözlenmedi; test sonucu yeniden doğrulanacak.

Son web koşusu 40096092271 başarılı; geliştirme araç çubuğu kapatıldı ve uygulama içeriğine daraltılan seçici ile 3 Chromium senaryosu geçti. Ekran görüntüleri bu koşudan alındı. Rust komutunun üç testi ilk tam koşuda başarılı.
