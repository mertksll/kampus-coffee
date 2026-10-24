# Rust komutları

`siparis_olustur(quantity: u32)` toplam adedi 1-9900 arasında doğrular. Başarı `{code: string, quantity: number}`, hata `{code: "gecersiz_adet", message: string}`. Takip kodu güvenlik anahtarı değildir. Negatif/ondalıklı girdiler native.ts sınırında reddedilir; Rust u32 çözümlemesi de bunları reddeder. Sıfır ve üst sınır taşması Rust içinde tipli hata döndürür.

`platform_bilgisi()` derleme hedefinin işletim sistemini döndürür. Beş hedef: android, ios, macos, windows, linux. UI yalnız `native.ts` kullanır. Tarayıcı aynı sözleşmeyle Web Crypto yedeğini kullanır. Sipariş geçmişi çakışma denetimi 10 denemeyle sınırlıdır.

Rust testleri: `cd src-tauri && cargo test`. GUI/WebKit geliştirme paketleri gerekir; cihaz testleri yerine geçmez. `orders.rs` içinde başarılı, sıfır ve üst sınır olmak üzere üç test vardır.
