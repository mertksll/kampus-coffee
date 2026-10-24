# Görev 16 - Batch denetimi
Araç/model: ChatGPT Work, Codex GPT-6. İstem: 4. hafta görevlerini uygula.
Plan: Pencere alt sınırlarını ekle, Svelte tip hatalarını gider, veri akışlarını gözden geçir ve gerçekten çalıştırılan kontrol çıktılarını kaydet.
Düzeltmeler: $props generic biçimi yerine tipli destructuring; geçersiz kimlikte ilk ürüne dönüş kaldırıldı; geçmiş hata denemesi kalıcı veriyi değiştirmez; hata bildirimi sipariş eylemini tekrar deneyebilir. Öğrenci numarası yeni kaynak metinden kaldırıldı.
Doğrulama: check 0 hata/uyarı; 12 test geçti; 127 statik sayfa; 96 ürünün dil/yön kontrolü geçti. Gerçek çıktı docs/kanit/batch-02/ içinde.
Kalan: Cargo, gerçek cihazlar, ekran kanıtları, PR/merge ve teslim etiketi. Bunlar yapılmış gibi gösterilmedi.

Bağlantı sonrası plan: Her görevin kendi dalında check/build/test yeniden çalıştırıldı. #22-27 birleştirildi. GitHub Actions üzerinde Cargo ve Playwright doğrulaması, filtreli gezinme ve dört halin gerçek ekran kanıtı için eklendi; sonuçlar henüz varsayılmaz.
