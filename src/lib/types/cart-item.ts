export interface CartItem {
  productId: number; // Ürünün kalıcı kimliği.
  size: string; // Doğrulamada sizes listesine göre denetlenir.
  milk: string; // Doğrulamada milks listesine göre denetlenir.
  quantity: number; // 1-99 arası tam sayı.
}
