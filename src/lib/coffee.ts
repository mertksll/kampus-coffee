import type { CartItem, Order, Profile } from "./types";
export type { CartItem, Order, Product, Profile } from "./types";
export const cafes = ["Starbucks", "Kahve Dünyası", "Espressolab"];
export { products } from './data';
import { products } from './data';

export const sizes = ["Standart", "Büyük"];
export const milks = ["Normal süt", "Yulaf sütü", "Sütsüz"];
export const statuses = ["Hazırlanıyor", "Kuryede", "Teslim edildi"];
export const tl = (value: number) =>
  value.toLocaleString("tr-TR", { style: "currency", currency: "TRY" });
export const unitPrice = (item: CartItem) =>
  (products.find((p) => p.id === item.productId)?.price ?? 0) +
  (item.size === "Büyük" ? 15 : 0) +
  (item.milk === "Yulaf sütü" ? 10 : 0);
export function totals(items: CartItem[]) {
  const quantity = items.reduce((sum, item) => sum + item.quantity, 0);
  const subtotal = items.reduce((sum, item) => sum + unitPrice(item) * item.quantity, 0);
  const discount = quantity >= 5 ? Math.round(subtotal * 0.1 * 100) / 100 : 0;
  return { quantity, subtotal, discount, total: subtotal - discount };
}
export function validCart(value: unknown): value is CartItem[] {
  if (!Array.isArray(value) || value.length > 100) return false;
  return (
    value.every(
      (item) =>
        item &&
        products.some((p) => p.id === item.productId) &&
        sizes.includes(item.size) &&
        milks.includes(item.milk) &&
        Number.isInteger(item.quantity) &&
        item.quantity >= 1 &&
        item.quantity <= 99,
    ) && new Set(value.map((item) => products.find((p) => p.id === item.productId)!.cafe)).size <= 1
  );
}
export function validOrders(value: unknown): value is Order[] {
  return (
    Array.isArray(value) &&
    value.length <= 100 &&
    value.every(
      (o) =>
        o &&
        typeof o.code === "string" &&
        /^KMP-[0-9A-F]{8}$/.test(o.code) &&
        cafes.includes(o.cafe) &&
        validCart(o.items) &&
        o.items.length > 0 &&
        o.items.every(
          (i: CartItem) => products.find((p) => p.id === i.productId)!.cafe === o.cafe,
        ) &&
        typeof o.address === "string" &&
        o.address.length <= 160 &&
        Number.isFinite(o.total) &&
        o.total >= 0 &&
        Number.isFinite(o.discount) &&
        o.discount >= 0 &&
        Number.isInteger(o.status) &&
        o.status >= 0 &&
        o.status <= 2 &&
        typeof o.created === "string" &&
        !Number.isNaN(Date.parse(o.created)),
    )
  );
}

export const brands = [
  { name: "Starbucks", image: "/brands/starbucks.png", label: "Sevdiğin kahve klasikleri" },
  {
    name: "Kahve Dünyası",
    image: "/brands/kahvedunyasi.png",
    label: "Tanıdık lezzetler, güzel molalar",
  },
  { name: "Espressolab", image: "/brands/espressolab.png", label: "Espressonun farklı tonları" },
];
export const brandFor = (name: string) => brands.find((b) => b.name === name) ?? brands[0];

export const emptyProfile: Profile = {
  name: "",
  building: "",
  floor: "",
  room: "",
  cafe: "Starbucks",
};
export function validProfile(p: unknown): p is Profile {
  if (!p || typeof p !== "object") return false;
  const v = p as Record<string, unknown>;
  return (
    ["name", "building", "floor", "room"].every(
      (k) => typeof v[k] === "string" && (v[k] as string).length <= 50,
    ) &&
    typeof v.cafe === "string" &&
    cafes.includes(v.cafe)
  );
}
export function mergeCart(cart: CartItem[], incoming: CartItem[]): CartItem[] {
  if (!validCart(incoming) || !incoming.length) throw new Error("Kahve seçeneklerini kontrol et.");
  const merged = cart.map((i) => ({ ...i }));
  const first = products.find((p) => p.id === (cart[0] ?? incoming[0]).productId)!;
  if (incoming.some((i) => products.find((p) => p.id === i.productId)!.cafe !== first.cafe))
    throw new Error(`Sepetinde ${first.cafe} kahveleri var. Her sipariş tek kafeden hazırlanır.`);
  for (const item of incoming) {
    const found = merged.find(
      (i) => i.productId === item.productId && i.size === item.size && i.milk === item.milk,
    );
    if (found) found.quantity += item.quantity;
    else merged.push({ ...item });
  }
  if (!validCart(merged)) throw new Error("Bir seçenekten en fazla 99 adet ekleyebilirsin.");
  return merged;
}
