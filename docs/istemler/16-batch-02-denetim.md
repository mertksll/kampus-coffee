# Görev 16 - Batch denetimi

Araç/model: ChatGPT Work, Codex GPT-6. İstem: 4. hafta görevlerini uygula. Devam istemi: "Bağlandı".

## Plan
Her görevi ayrı PR ile sun; pencere alt sınırlarını ekle; gerçek kontrol çıktılarını ve ekran kanıtlarını kaydet. Son doğrulanmış master kaynak görüntüsüne batch etiketi koy.

## Düzeltmeler
Svelte props tipleri açık destructuring biçimine taşındı. Geçersiz kimlikte ilk ürüne dönüş kaldırıldı. Sipariş geçmişi hata denemesi kalıcı veriyi değiştirmez. Tekrar dene gerçek yüklemeyi yeniler. Öğrenci numarası kaynak metinden kaldırıldı. Tarayıcı testinin h1 seçicisi uygulama içeriğine daraltıldı; geliştirme araç çubuğu ekran kanıtlarından çıkarıldı.

## Doğrulama
Yerel check 0 hata/uyarı; 12 birim testi ve 127 sayfalık build başarılı. [Son Actions koşusu](https://github.com/mertksll/kampus-coffee/actions/runs/40096092271): web ve Rust işleri başarılı, 3 Chromium ve 3 Rust testi geçti. Dört hal ve RTL ekranları repoda. Görev PR'ları #22-28; lat.md PR #29. Etiket son PR birleşiminden sonra doğrulanmış master'a konur.

## Öğrenciye kalan
Kendi cihazında Tauri kontrolü, Görev 10 yeni sohbet deneyi ve Blackboard'a öğrenci teslimi. Ajan bunları yapılmış gibi kaydetmez. Beş gerçek cihazda çalıştırma iddiası yoktur.
