# Mimari

Astro statik sayfaları, Svelte etkileşimleri ve Tauri Rust çekirdeği aynı kaynak ağacındadır. Bu ayrım web demosunun yerel komut olmadan da çalışmasını sağlar.

Menü ve filtreler [[src/components/CoffeeApp.svelte]], genel kart [[src/lib/components/ui/Kart.svelte]], ayrıntılar [[src/components/ProductDetail.svelte]] içindedir. Rotalar [[docs/mimari-agac.md]] belgesinde tanımlanır.

## Veri sözleşmesi

Ürün, sepet kalemi, sipariş ve profil tipleri merkezi tutulur; örnek kayıtlar gerçek geçmişe karışmaz.

[[src/lib/types/index.ts]] bütün tipleri dışa aktarır. [[src/lib/data.ts#products]] 24 kahve içerir. Alan açıklamaları [[docs/veri-modeli.md]] içindedir.

## Native sınırı

Arayüz Rust komutunu tek giriş noktasından çağırır. Tarayıcı aynı sonuç biçimini Web Crypto ile üretir.

[[src/lib/native.ts#siparisOlustur]] sipariş sınırıdır; [[src/lib/native.ts#platform]] hedefi bildirir. Platform değerlendirmesi [[docs/platform-destegi.md]] içindedir.
