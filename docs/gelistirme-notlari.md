# Geliştirme notları

`bun run dev` sırasında `/`, `/siparisler/` veya `/ar/` adreslerine `?durum=yukleniyor`, `?durum=hata`, `?durum=bos` ekleyin. Yükleniyor deneyi 5 saniye sürer. Hata halinde Tekrar dene normal kaynağı yükler. Parametreler production derlemesinde etkisizdir. Dolu hal normal URL'dir; geçmişin dolması için demo sipariş verilir. Rastgele bir arama yazıp filtreleri temizlemeyi deneyin. Dil rotaları: `/`, `/en/`, `/ar/`, `/fa/`; ortak durum mesajları bu dillere bağlıdır.
