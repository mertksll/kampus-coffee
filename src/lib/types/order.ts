import type { CartItem } from "./cart-item";
export type OrderStatus = 0 | 1 | 2;
export interface Order {
  code: string; // KMP- ve 8 hexadecimal karakter.
  cafe: string; // Tek kaynak kafe.
  items: CartItem[]; // Sipariş kalemleri.
  address: string; // Örnek bina, kat ve sınıf.
  total: number; // İndirim sonrası TL tutarı.
  discount: number; // TL indirim tutarı.
  status: OrderStatus; // 0 hazırlanıyor, 1 kuryede, 2 teslim.
  created: string; // ISO 8601 kayıt zamanı.
}
