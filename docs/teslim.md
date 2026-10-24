# Teslim ve değerlendirme düzeltmeleri — 10 Ekim 2026

Öğrenci: **Mustafa Mert Köksal · [kişisel numara kaldırıldı] · mertksll**. Proje: **Kampus's Coffee**. Güncel depo: [mertksll/kampus-coffee](https://github.com/mertksll/kampus-coffee), `keyvanarasteh/hello-mobil` fork'udur.

## Eğitmenin geri bildirimi

Öğrencinin paylaştığı Batch 01 değerlendirmesi **105/108**: eski doğrudan master commit'i için −1, ayrı ajan uyum testi kaydı bulunmadığı için −2. Diğer kalemler tam puandır. Aşağıdaki düzeltmeler yeniden puanlama sonucu değildir; notu eğitmen belirler.

[Eski doğrudan commit f3ba476](https://github.com/mertksll/kampus-coffee/commit/f3ba476d134a9c81d17649587d9363991f23b7fa) proje fikri belgesini değiştirmiştir. Geçmiş gizlenmedi veya yeniden yazılmadı; yeni koruma bu eski işlemi geçmişe dönük düzeltmez.

## Tamamlanan düzeltmeler

- [x] Depo `kampus-coffee` olarak yeniden adlandırıldı; fork ilişkisi, işbirlikçi erişimi, proje bağlantıları ve kurulum komutları korundu/güncellendi.
- [x] Bir renk ve bir sayfa görevi ayrı dallarda uygulanıp PR ile birleştirildi. Görev istemleri, okunan kurallar, değişiklikler, gerçek ölçümler ve PR bağlantıları [ajan uyum testlerinde](ajan-uyum-testleri.md).
- [x] Yeni belge AGENTS indeksinde; bütün yerel belge bağlantıları ve CLAUDE/GEMINI yönlendirmeleri doğrulandı.
- [x] `master koruması` etkin, hedef varsayılan dal, bypass listesi boş. PR zorunlu; force push ve dal silme engelli.
- [x] Gerçek Git gönderiminde `refs/heads/master` hedefi GH013 ile reddedildi ve master değişmedi. Test, görev dalındaki commit'i doğrudan master hedefine gönderdi; yerel master'a commit atılmadı. [Gerçek ret çıktısı](kanit/master-korumasi.txt).
- [x] Aynı değişiklik [PR #16](https://github.com/mertksll/kampus-coffee/pull/16) ile normal merge edildi. Merge commit ve otomatik kaynak dalı silme etkin; PR #16–18'in kaynak dallarının silindiği doğrulandı.
- [x] Dış katkıcıların bütün PR iş akışları onay gerektirir; workflow varsayılanı salt okumadır. Yetkililer yalnız `mertksll` ve `keyvanarasteh`. [GitHub ayar kaydı](kanit/github-ayarlar.json), [PR güvenliği](kurallar.md#pr-güvenliği).
- [x] Eğitmenin #14 ve #15 PR'ları zaten birleştirilmişti; [Hafta 4 görevleri](tasks/week-4/) kaynakta bulunur. Görev 10–15 bu düzeltme çalışmasında tamamlandı olarak işaretlenmez.
- [x] **Kural seti ekran görüntüsü:** Öğrencinin sağladığı 1584 × 4616 px tam sayfa PNG, [master-korumasi.png](kanit/master-korumasi.png) olarak değiştirilmeden eklendi. Repo adı, Active durumu, boş bypass listesi, varsayılan master hedefi ve üç koruma kuralı görünür. Kayıtlı ayarlar API üzerinden yeniden doğrulandı; zorunlu PR onay sayısı 0. Böylece Görev 09.1 için ayar, doğrudan gönderim reddi, PR ile merge, güvenlik belgesi ve görüntü kanıtı tamamlandı.

## Son doğrulama

- `bun run build`: **48 statik sayfa**, çıkış 0. [Build çıktısı](kanit/build.txt).
- `bun run check`: Astro ve Svelte **0 hata / 0 uyarı**. [Tür kontrolü](kanit/check.txt).
- `bun run test`: **9 test / 25 doğrulama**, 0 başarısız. [Test çıktısı](kanit/test.txt).
- [Statik denetim](kanit/static-audit.txt): rota/dil/yön bilgileri, yerel bağlantılar, belge indeksi, marka–CSS eşleşmesi, metin kontrastı ve beş platform ikonları başarılı.
- [Sayfa denetimi](kanit/ajan-sayfa.txt): rehberin dört çevirisi, AR/FA RTL, karşılıklı dil bağlantıları ve 48 sayfanın alt menüsünden erişim başarılı.
- [JSDOM bileşen testi](kanit/dom-test.txt): 24 kahve, kafe filtresi, sepet, indirim, profil, sipariş geçmişi ve dört dilde form akışı başarılı. Bunlar gerçek tarayıcı görsel testleri değildir.
- Önceki Windows Tauri çalıştırması ve öğrencinin verdiği görüntüler korunur: [Tauri görüntüsü](kanit/tauri-dev.png), [Tauri kaydı](kanit/tauri-dev.txt), [9 Ekim web build görüntüsü](kanit/build.png). Bu görüntüler 10 Ekim değişikliklerinin yeni native testi olarak sunulmaz. Android/iOS ve dağıtım paketleri henüz doğrulanmadı.

## Düzeltme PR'ları

| PR | Görev | Sonuç |
|---|---|---|
| [#16](https://github.com/mertksll/kampus-coffee/pull/16) | Repo adı, master koruması, güvenlik politikası ve doğrudan push testi | Normal merge, build başarılı |
| [#17](https://github.com/mertksll/kampus-coffee/pull/17) | Gerçek renk görevi ve kontrast kanıtı | Normal merge, iki tema ve build başarılı |
| [#18](https://github.com/mertksll/kampus-coffee/pull/18) | Dört dilde sayfa görevi ve uyum kanıtı | Normal merge, build/check/test başarılı |

Son belge kaydı [PR #19](https://github.com/mertksll/kampus-coffee/pull/19) ile birleştirildi. Tam sayfa kural seti görüntüsü ve tamamlanma kaydı `feature/master-korumasi-kaniti` dalından ayrı PR ile sunulur. [PR listesi](https://github.com/mertksll/kampus-coffee/pulls?q=is%3Apr).

## Batch 01 etiketi ve güncel ZIP

Değerlendirilmiş [`v0.1.0-batch-01`](https://github.com/mertksll/kampus-coffee/tree/v0.1.0-batch-01) etiketi, **`e4871639cbd6ef8a5d7a96b94fcd4d7b289583c5`** commitinde korunur. O teslimin [tarihsel kontrol kaydı](https://github.com/mertksll/kampus-coffee/blob/v0.1.0-batch-01/docs/teslim.md) ve 161 dosyalık ZIP'leri bu sürüme aittir. Yeni düzeltmeler master'ı ilerletir; eski etiketten indirilen ZIP yeni ajan testlerini içermez.

Güncel kaynak için [master sayfasında](https://github.com/mertksll/kampus-coffee/tree/master) **Code → Download ZIP** kullanılır. ZIP kaynak koddur; APK/EXE kurulum paketi değildir. İndirilen arşiv kendi kaynak commiti ve dosya içerikleriyle karşılaştırılmalıdır.

## Öğrencinin tamamlayacağı işlemler

1. [GitHub master sayfasında](https://github.com/mertksll/kampus-coffee/tree/master) **Code → Download ZIP** ile güncel kaynak ZIP'ini indir; eski indirme kendiliğinden güncellenmez.
2. ZIP içinde `docs/kanit/master-korumasi.png` ve `docs/ajan-uyum-testleri.md` bulunduğunu kontrol et.
3. Eğitmenin son bildirimine göre Blackboard'a yalnız ZIP'i yükleyip gönderimi kendin tamamla. İlk teslimin 9 Ekim 23:59 / üç deneme kuralı geçmiş teslim içindi; bu düzeltme için yeni süre veya deneme hakkı bu belgede varsayılmaz.
4. Blackboard gönderim onayını kontrol et. Ajan Blackboard'a teslim yapmaz veya öğretmene mesaj göndermez.
