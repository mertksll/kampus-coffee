import type { Snippet } from 'svelte';
export interface KartProps {
 baslik: string; // Kartın erişilebilir başlığı.
 altMetin?: string; // İsteğe bağlı açıklama.
 gorsel?: string; // İsteğe bağlı görsel adresi.
 gorselAlt?: string; // Görselin alternatif metni.
 etiket?: string; // İsteğe bağlı durum etiketi.
 etiketTuru?: 'bilgi' | 'basari' | 'uyari' | 'hata'; // Etiket anlamı.
 href?: string; // Ayrıntı sayfası.
 onclick?: () => void; // Adres yerine eylem.
 children?: Snippet; // Ekrana özgü ek eylemler.
}
