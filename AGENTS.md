# Ajan çalışma kuralları

## Önce oku
Bu depo İstinye Üniversitesi MYO063 dersinin Kampus's Coffee eğitim demosudur.
Kullanıcıya ait kaynak: https://github.com/mertksll/kampus-coffee.

## Tek kaynaklı belgeler
| Belge | Kapsam |
|---|---|
| [Proje fikri](docs/proje-fikri.md) | Amaç ve gelecekteki kapsam |
| [Marka kılavuzu](docs/branding.md) | Renk token'larının tek kaynağı |
| [Klasör mimarisi](docs/klasor-mimarisi.md) | Klasör ağacının tek kaynağı |
| [Sayfa mimarisi](docs/mimari-agac.md) | Rotalar, platformlar ve duyarlı düzen |
| [Kurulum](docs/kurulum.md) | Önkoşullar ve komutlar |
| [Kurallar](docs/kurallar.md) | İş kuralları ve PR güvenliği |
| [Kaynaklar](docs/kaynaklar.md) | Resmi kaynaklar |
| [Ajan uyum testleri](docs/ajan-uyum-testleri.md) | Gerçek renk ve sayfa görevleri, kurallar ve doğrulama kanıtı |
| [Teslim](docs/teslim.md) | Kontrol listesi ve kalan işler |
| [Hafta 4 görevleri](docs/tasks/week-4/) | Öğretim görevlisinin Batch 02 yönergeleri |
| [Hafta 3 görevleri](docs/tasks/week-3/) | Öğretim görevlisinin özgün yönergeleri |

Belgelerdeki renk tablosunu veya ağaçları README ve diğer belgelere kopyalama; bağlantı ver.
Yeni docs/*.md belgesi eklersen bu dizine de bağlantı ekle.

## Teknoloji ve güvenlik
Astro statik çıktı, Svelte 5 runes ($state, $derived, $props), React, MDX, Tauri v2 ve Bun kullan.
Geliştirme adresi 127.0.0.1:1420; portu ve static çıktıyı koru.
window/document/localStorage erişimini istemciye sınırla; onMount ve try/catch kullan.
Kullanıcı girişlerini HTML olarak işleme. Gerçek ödeme, mesaj gönderimi ve marka ortaklığı yok.
Görev kapsamını büyütme; çalışmayan veya denenmeyen şeyi çalışıyor diye belgeleme.
TR/EN/AR/FA bilgi sayfalarını birlikte güncelle; AR/FA için lang ve dir=rtl kullan.
Renk değiştirirken branding tablosu ve iki CSS temasını eşleştir, kontrastı ölç.
Yeni sayfayı navigasyona, mimari-ağaç belgesine ve gerekiyorsa dil bağlantılarına ekle.

## Git ve doğrulama
[PR güvenliği](docs/kurallar.md#pr-güvenliği) kurallarını uygula.
master/main üzerine doğrudan commit atma. Yeni görevlerde feature/<ad> veya fix/<ad> dalı aç; belge düzeltmelerini de bu dallar üzerinden sun.
feat:, fix:, docs: gibi Conventional Commit başlıkları kullan.
Her görev dalını GitHub PR ile sun; açıklamada yapılan değişiklik ve AI görevini 2–3 cümlede belirt.
Files changed incelemesi ve başarılı build sonrası PR birleştir; en az bir merged PR gereklidir.
Komutlar: bun install; bun run dev; bun run build; bun run check; bun run test; bun run tauri dev.
İkonlar: bun run tauri icon assets/app-icon.png.
Derleme kanıtını docs/kanit/ altında tut. Native test yapılamadıysa açıkça yaz.
Blackboard'a teslimi öğrenci yapar; ajan teslim etmez ve öğretmene mesaj göndermez.

Her batch teslim etiketi, o batch tamamlandığında doğrulanmış master commitini göstermelidir. Değerlendirilmiş batch etiketini sonraki görevler için taşıma. Yeni ZIP'i kendi kaynak commitiyle karşılaştır; bir etiket ZIP'i ise etiketin hedefiyle eşleştir. Git geçmişindeki eski PR’siz işlemleri gizleme veya yeniden yazma; teslim kaydında kapsamını açıkça belirt.

## Çalışma döngüsü
Önce şartnameyi oku, planı sun ve kullanıcının onayını bekle; kullanıcı mevcut oturumda uygulamayı açıkça yetkilendirmişse aynı izni tekrar isteme. Onaydan sonra değiştir. Her görev ayrı feature dalı ve PR ile ilerler. `bun run build` 0 hata vermeden görev bitmiş sayılmaz. Gerçek istemi, planı, düzeltmeleri ve test sonuçlarını görev günlüğüne yaz. Öğrencinin yapmadığı elle kontrolleri yapılmış gösterme.

## Yeni belgeler
- [Görev şartnamesi](docs/gorev-sartnamesi.md)
- [İstem günlükleri](docs/istemler/README.md)

Yeni veri alanı önce src/lib/types/ içinde tanımlanır.
- [Veri modeli](docs/veri-modeli.md)

- [Geliştirme notları](docs/gelistirme-notlari.md)
