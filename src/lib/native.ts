import type { OrderCode, NativeError, Platform, Feature } from './types';
const isNative = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
export async function platform(): Promise<Platform> {
 if (!isNative()) return 'web';
 const {invoke}=await import('@tauri-apps/api/core');
 return invoke<Platform>('platform_bilgisi');
}
/** Bu üç özellik WebView + Rust ile beş native hedefte de uygulanmıştır. */
export function destekleniyorMu(feature: Feature): boolean {
 return ['menu','siparis','depolama'].includes(feature);
}
export async function siparisOlustur(quantity: number): Promise<OrderCode> {
 if (!Number.isInteger(quantity) || quantity < 1 || quantity > 9900)
  throw {code:'gecersiz_adet',message:'Adet 1-9900 arası tam sayı olmalıdır.'} satisfies NativeError;
 try {
  if (isNative()) {
   const {invoke}=await import('@tauri-apps/api/core');
   return await invoke<OrderCode>('siparis_olustur',{quantity});
  }
  const code='KMP-'+Array.from(crypto.getRandomValues(new Uint8Array(4)), b=>b.toString(16).padStart(2,'0')).join('').toUpperCase();
  return {code,quantity};
 } catch(error) {
  if (error && typeof error==='object' && 'code' in error && 'message' in error) throw error;
  throw {code:'ortam',message:'Sipariş kodu üretilemedi.'} satisfies NativeError;
 }
}
