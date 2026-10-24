<div align="center">
<a href="https://www.istinye.edu.tr"><picture><source media="(prefers-color-scheme: dark)" srcset="public/isu-logo-dark.svg"><img alt="İstinye Üniversitesi" src="public/isu-logo.svg" width="280"></picture></a>
</div>

# Kampus's Coffee

**Kampüsün kahvesi, sınıfının kapısında.** Tek kafeden kişiselleştirilmiş kahve seçimi ve aynı sınıfa toplu sipariş için eğitim demosu.

[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)](https://v2.tauri.app/)
[![Astro 7](https://img.shields.io/badge/Astro-7-BC52EE?logo=astro)](https://astro.build/)
[![Svelte 5](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte)](https://svelte.dev/)
[![React 19](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev/)
[![Bun](https://img.shields.io/badge/Bun-1.4-222222?logo=bun)](https://bun.sh/)
[![Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue)](LICENSE)

<a id="icindekiler"></a>
## İçindekiler

- [Kampus's Coffee](#kampuss-coffee)
- [İçindekiler](#icindekiler)
- [Akademik bilgiler](#akademik)
- [Proje ve özellikler](#proje)
- [Teknolojiler](#teknolojiler)
- [Kurulum ve çalıştırma](#kurulum)
- [Belgeler ve görevler](#belgeler)
- [Lisans](#lisans)

<a id="akademik"></a>
## Akademik bilgiler

| Bilgi | Değer |
|---|---|
| Üniversite | [İstinye Üniversitesi](https://www.istinye.edu.tr) |
| Birim / program | Meslek Yüksekokulu — Bilişim Güvenliği Teknolojisi |
| Ders | MYO063 — Mobil Programlama |
| Dönem | 2026–2027 Güz |
| Öğretim görevlisi | Öğr. Gör. Keyvan Arasteh Abbasabad |
| Eğitmen bağlantıları | [Qrofessor](https://qrofessor.com) · [GitHub](https://github.com/keyvanarasteh) · [LinkedIn](https://www.linkedin.com/in/keyvanarasteh/) · [İSÜ](https://www.istinye.edu.tr) |
| Öğrenci | **Mustafa Mert Köksal** |
| GitHub / iletişim | [mertksll](https://github.com/mertksll) |
| Fork | [mertksll/kampus-coffee](https://github.com/mertksll/kampus-coffee) |
| Kaynak şablon | [keyvanarasteh/hello-mobil](https://github.com/keyvanarasteh/hello-mobil) |

<a id="proje"></a>
## Proje ve özellikler

Ders arasında sırada bekleyen öğrenciler için kafe ve kahve seçimini, sınıfa teslimat fikriyle birleştirir. Bu sürümde:

- Üç kafe, 24 fotoğraflı kahve; marka logoları, arama, sıcak/soğuk filtresi ve fiyat sıralaması.
- Boyut, süt ve adet seçimi; hızlı ekleme, favoriler, tek kafeli sepet ve adet düzenleme.
- Aynı sınıfa 5 ve üzeri kahvede %10 demo indirim.
- Demo takip kodu, aktif/tamamlanan sipariş filtresi, durum adımları ve tekrar sipariş.
- Cihazda saklanan isim, kafe ve sınıf tercihleri; profil istatistikleri ve kontrollü geçmiş silme.
- Açık / koyu tema, farklı ekranlara uyumlu görünüm.
- TR / EN / AR / FA bilgi sayfaları ve sipariş rehberi, sağdan sola dil desteği ve demo iletişim formu.

Kafe adları örnektir; ortaklık veya gerçek entegrasyon yoktur. Ödeme, gerçek kurye ve sunucu hesabı bu aşamanın kapsamı dışındadır. Web demosunda kod tarayıcıda; Tauri sürümünde `siparis_olustur` Rust komutuyla üretilir. Windows x64 üzerinde `bun run tauri dev` derlemesi ve uygulama sürecinin başlatılması doğrulandı; [çalıştırma kaydı](docs/kanit/tauri-dev.txt). Android/iOS ve dağıtım paketleri henüz doğrulanmadı. Ayrıntılar: [proje fikri](docs/proje-fikri.md), [iş kuralları](docs/kurallar.md), [teslim durumu](docs/teslim.md).

<a id="teknolojiler"></a>
## Teknolojiler

| Katman | Teknoloji |
|---|---|
| Yerel uygulama | Tauri v2, Rust |
| Statik sayfalar | Astro 7, MDX |
| Etkileşim | Svelte 5 runes, React 19 |
| Dil ve stil | TypeScript, CSS token'ları |
| Paket ve test | Bun; Node.js 22.12+ |

<a id="kurulum"></a>
## Kurulum ve çalıştırma

Önce [kurulum önkoşullarını](docs/kurulum.md) tamamlayın. Web için Rust gerekmez; Tauri için Rust ve işletim sistemi araçları gerekir.

```bash
git clone https://github.com/mertksll/kampus-coffee.git
cd kampus-coffee
bun install
bun run dev
```

Web: http://127.0.0.1:1420. Aşağıdaki komutlar ayrı ayrı çalıştırılır:

```bash
bun run tauri dev
bun run build
bun run check
bun run test
```

Derleme çıktısı `dist/` dizinidir. Platform çıktıları ve hedefleri [mimari belgede](docs/mimari-agac.md); bu ZIP kaynak koddur, APK/EXE paketi değildir.

<a id="belgeler"></a>
## Belgeler ve görevler

[Proje fikri](docs/proje-fikri.md) · [Klasör mimarisi](docs/klasor-mimarisi.md) · [Sayfa ve platform mimarisi](docs/mimari-agac.md) · [Marka kılavuzu](docs/branding.md) · [Kurulum](docs/kurulum.md) · [Kurallar](docs/kurallar.md) · [Kaynaklar](docs/kaynaklar.md) · [Ajan uyum testleri](docs/ajan-uyum-testleri.md) · [Teslim kontrolü](docs/teslim.md) · [Hafta 3 görevleri](docs/tasks/week-3/) · [Hafta 4 görevleri](docs/tasks/week-4/) · [Ajan kuralları](AGENTS.md)

<a id="lisans"></a>
## Lisans

Kaynak şablon ve bu uyarlama [Apache License 2.0](LICENSE) kapsamındadır. Üniversite ve üçüncü taraf marka adlarının hakları ilgili sahiplerine aittir. Bu proje MYO063 kapsamında hazırlanmış bir eğitim çalışmasıdır.

## Batch 02

[Hafta 4 ilerleme ve doğrulama](docs/ilerleme-batch-02.md) · [Veri modeli](docs/veri-modeli.md) · [Platform desteği](docs/platform-destegi.md) · [lat.md](lat.md/lat.md)
