# Veri modeli

Bütün sözleşmeler `src/lib/types/index.ts` üzerinden dışa aktarılır. Örnekler `src/lib/data.ts` içindedir: 24 ürün, 6 sepet kalemi, 6 sipariş ve 6 profil. Bunlar sentetiktir; sipariş geçmişine eklenmez.

## profile

| Alan | Tip | Açıklama |
|---|---|---|
| `name` | `string` | Cihazdaki demo görünen adı. |
| `building` | `string` | Örnek bina. |
| `floor` | `string` | Örnek kat. |
| `room` | `string` | Örnek sınıf. |
| `cafe` | `string` | Tercih edilen kafe. |

## order

| Alan | Tip | Açıklama |
|---|---|---|
| `code` | `string` | KMP- ve 8 hexadecimal karakter. |
| `cafe` | `string` | Tek kaynak kafe. |
| `items` | `CartItem[]` | Sipariş kalemleri. |
| `address` | `string` | Örnek bina, kat ve sınıf. |
| `total` | `number` | İndirim sonrası TL tutarı. |
| `discount` | `number` | TL indirim tutarı. |
| `status` | `OrderStatus` | 0 hazırlanıyor, 1 kuryede, 2 teslim. |
| `created` | `string` | ISO 8601 kayıt zamanı. |

## cart-item

| Alan | Tip | Açıklama |
|---|---|---|
| `productId` | `number` | Ürünün kalıcı kimliği. |
| `size` | `string` | Doğrulamada sizes listesine göre denetlenir. |
| `milk` | `string` | Doğrulamada milks listesine göre denetlenir. |
| `quantity` | `number` | 1-99 arası tam sayı. |

## product

| Alan | Tip | Açıklama |
|---|---|---|
| `id` | `number` | Kalıcı ürün kimliği. |
| `cafe` | `string` | Demo kafe adı. |
| `name` | `string` | Görünen ürün adı. |
| `kind` | `'Sıcak' | 'Soğuk'` | Servis sıcaklığı. |
| `price` | `number` | Standart boy TL fiyatı. |
| `image` | `string` | Yerel görsel yolu. |
| `description` | `string` | Ürün açıklaması. |
| `tag?` | `string` | İsteğe bağlı kısa etiket. |
| `defaultMilk` | `string` | Varsayılan süt seçimi. |

## İlişkiler
CartItem.productId → Product.id; Order.items → CartItem[]; Profile.cafe → menüdeki kafe adı. Order.status sabit 0/1/2 değerleridir. İsteğe bağlı alan: Product.tag; diğer alanlar zorunludur.
