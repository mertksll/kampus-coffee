export interface OrderCode {
 code: string; // Rust ile aynı takip kodu.
 quantity: number; // Doğrulanmış toplam kahve adedi.
}
export interface NativeError {
 code: 'gecersiz_adet' | 'ortam' | 'bilinmeyen'; // Hatanın sabit kodu.
 message: string; // Güvenli hata açıklaması.
}
export type Platform = 'android' | 'ios' | 'macos' | 'windows' | 'linux' | 'web';
export type Feature = 'menu' | 'siparis' | 'depolama';
