# Proje Fikri ve Konsept Belgesi

## 1. Proje Künyesi

- **Proje Adı:** Kampus's Coffee
- **Slogan / Tek Cümlelik Tanım:** Kampüsün kahvesi, sınıfının kapısında.
- **Öğrenci Adı Soyadı:** Mustafa Mert Köksal
- **Öğrenci Numarası:** [kişisel numara kaldırıldı]
- **GitHub Kullanıcı Adı:** mertksll
- **Fork Linki:** https://github.com/mertksll/kampus-coffee
- **İlham Alınan Konsept / Platform:** Kahve sipariş uygulamaları ve Yemeksepeti / Getir'in teslimat akışı; kampüs içi kullanım için özgün uyarlama.

## 2. Proje Amacı ve Çözülen Problem

Kampus's Coffee, kampüsteki farklı kahve işletmelerini tek platformda buluşturarak öğrencilerin ders aralarında kafeye gitmeden sınıflarına kahve siparişi verebilmesini amaçlar. Öğrenciler kafe ve ürün seçebilecek, kahvelerini kişiselleştirebilecek ve bina, kat, sınıf bilgileriyle teslimat isteyebilecektir. Aynı sınıfa verilen toplu siparişler ve öğrencilere yönelik kampanyalar, teslimatı daha verimli ve kahve alışverişini daha ekonomik hâle getirecektir.

İlk sürüm, örnek kafe ve menü verileriyle çalışan bir ders projesi demosu olarak planlanmaktadır. Starbucks, Kahve Dünyası ve Espressolab gibi işletmelerin menü yapıları örnek alınabilir; örnek fiyatlar, indirimler ve sipariş durumları demo verisi olarak belirtilecektir. Gerçek işletmelerin sipariş sistemleriyle bağlantı ve gerçek kurye operasyonu, sonraki geliştirme aşamasıdır.

## 3. Temel Ekranlar ve İşlevler

### 3.1. Ana Liste Ekranı (Keşfet)

Kampüsteki kafeler, kahve menüleri, fiyatlar ve öğrenci kampanyaları listelenecektir. Kullanıcı kafe veya ürün adına göre arama yapabilecek; kafe, sıcak / soğuk içecek ve fiyat seçeneklerine göre filtreleme uygulayabilecektir. Bir kafeyi seçerek o işletmenin menüsüne geçecektir.

### 3.2. Detay ve Seçim Ekranı

Seçilen kahvenin açıklaması, fiyatı ve seçenekleri gösterilecektir. Boyut, süt tercihi ve adet seçilerek ürün sepete eklenecektir. Sepette ürünler düzenlenebilecek, bina–kat–sınıf bilgileri girilebilecek ve indirim sonrası toplam görülebilecektir.

İlk sürümde her sipariş tek kafeye ait olacaktır. Aynı siparişte, aynı sınıfa gönderilen 5 veya daha fazla kahve için örnek bir %10 toplu sipariş indirimi uygulanması planlanmaktadır. Öğrenci kampanyaları kafe bazında tanımlanacak; birden fazla uygun kampanya varsa en avantajlı tek indirim uygulanacaktır. Bu kurallar demo kampanyalarıdır.

### 3.3. Kayıt / Kod Üretme Ekranı (Rust Backend)

Sipariş onaylandığında Rust tarafında planlanan sipariş oluşturma komutu, benzersiz bir takip kodu üretecektir. Kodun formatı **KMP-XXXXXXXX** olacaktır; X karakterleri büyük harf veya rakamdan oluşacaktır. Örnek: **KMP-A7D2F9B4**. Üretilen kod mevcut siparişlerle karşılaştırılacak ve tekrar eden kod kullanılması önlenecektir.

Sipariş özeti; kafe, ürünler, teslim edilecek sınıf, indirim, toplam tutar ve takip kodunu içerecektir. Sipariş geçmişi yerel olarak saklanacak ve kullanıcı demoda **Hazırlanıyor → Kuryede → Teslim edildi** durumlarını takip edebilecektir.

### 3.4. Profil ve Ayarlar

Kullanıcı adını, tercih ettiği kafeyi ve varsayılan teslimat sınıfını görebilecek; geçmiş siparişlerine ulaşabilecektir. Açık / koyu tema seçimi ve kayıtlı teslimat tercihleri bu ekranda düzenlenecektir.

## 4. Hedef Kitle

Ana hedef kitle, ders aralarında zamanı sınırlı olan ve kahvesini sınıfa teslim almak isteyen üniversite öğrencileridir. Aynı sınıfta birlikte sipariş veren öğrenci grupları, toplu sipariş indirimlerinden yararlanabilecektir. Kampüs kafeleri ve kampüs içi teslimat görevlileri ise projenin sonraki aşamalarındaki diğer kullanıcı gruplarıdır.

## 5. Temel Veri Modeli ve Şablonun Uyarlanması

- **Kafe:** Kimlik, ad, menü ve teslimat saatleri.
- **Ürün:** Kimlik, kafe kimliği, ad, açıklama, içecek kategorisi, fiyat ve boyut / süt seçenekleri.
- **Sepet kalemi:** Ürün kimliği, seçilen seçenekler, adet ve birim fiyat.
- **Kampanya:** Kafe kimliği, minimum adet ve indirim oranı.
- **Sipariş:** Takip kodu, kafe kimliği, ürün kalemleri, bina–kat–sınıf, indirim, toplam ve durum.

PassoKlon şablonundaki etkinlik kartları kafe / kahve kartlarına, bilet seçimi kahve seçeneklerine, sepet sipariş sepetine ve Rust bilet kodu üretimi sipariş takip kodu üretimine uyarlanacaktır. Böylece mevcut arama, filtreleme, detay, sepet ve profil yapıları yeni proje fikrinde kullanılacaktır.
