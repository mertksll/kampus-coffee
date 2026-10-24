# İş kuralları

Bir sepet tek kafeye aittir; aynı örnek sınıfa beş kahvede yüzde on indirim uygulanır. Gerçek ödeme, ortaklık ve kurye entegrasyonu yoktur.

[[src/lib/coffee.ts#mergeCart]] kafe ve adet sınırlarını korur. [[src/lib/coffee.ts#totals]] tutarı hesaplar. Tam iş kuralları [[docs/kurallar.md]] içindedir.

## Cihaz kaydı

Sepet, favori, profil ve geçmiş localStorage içinde tutulur. Bozuk kayıtlar kabul edilmez; geçmiş en fazla yüz sipariştir.

[[src/lib/coffee.ts#validCart]], [[src/lib/coffee.ts#validOrders]] ve [[src/lib/coffee.ts#validProfile]] kayıt doğrulama sınırlarıdır.
