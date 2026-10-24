/** Menüdeki fotoğraflı kahve. */
export interface Product {
  id: number; // Kalıcı ürün kimliği.
  cafe: string; // Demo kafe adı.
  name: string; // Görünen ürün adı.
  kind: "Sıcak" | "Soğuk"; // Servis sıcaklığı.
  price: number; // Standart boy TL fiyatı.
  image: string; // Yerel görsel yolu.
  description: string; // Ürün açıklaması.
  tag?: string; // İsteğe bağlı kısa etiket.
  defaultMilk: string; // Varsayılan süt seçimi.
}
