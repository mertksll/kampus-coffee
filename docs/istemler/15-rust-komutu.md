# Görev 15 - Rust sözleşmesi
Araç/model: ChatGPT Work, Codex GPT-6. İstem: 4. hafta görevlerini uygula.
Plan: String komutu tipli Result'a dönüştür; Rust'ta adet doğrula, webde aynı sözleşmeyle kod üret; invoke yalnız native.ts içinde olsun.
Düzeltme: Aynı kodun çakışma denetimi korundu. Başarı ve hata sözleşmesi iki tarafta eşlendi. Testler başarılı/sıfır/üst sınır girdilerini kapsar.
Doğrulama: Bu ortamda cargo başlangıçta yok; çalıştırılamayan native test başarılı sayılmaz. Son durum batch raporunda. Öğrenci Tauri'de sipariş, geçersiz girdi ve beş cihaz hedefini ayrıca kontrol etmeli.

## Sonuç
Linux GitHub Actions üzerinde üç Rust testi geçti. [Gerçek cargo çıktısı](../kanit/batch-02/cargo-test.txt). Cihaz testleri ayrıca öğrenciye aittir.
