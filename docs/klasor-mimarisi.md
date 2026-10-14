# Klasör mimarisi

Bu belge dizin ağacının tek kaynağıdır; tek tek bileşen dosyaları listelenmez.

```text
kampus-coffee/
├── package.json        # Bağımlılıklar, sürüm ve geliştirme komutları
├── bun.lock            # Sabitlenmiş bağımlılık çözümü
├── astro.config.mjs    # Static çıktı, React/Svelte/MDX, 127.0.0.1:1420
├── svelte.config.js    # Svelte TypeScript ön işleme
├── tsconfig.json       # TypeScript ve $lib yolu
├── AGENTS.md           # Ajan kuralları ve belge indeksi
├── CLAUDE.md           # AGENTS yönlendirmesi
├── GEMINI.md           # AGENTS yönlendirmesi
├── README.md           # Kurumsal tanıtım ve başlangıç
├── LICENSE             # Apache-2.0
├── assets/             # 1024 px kaynak uygulama ikonu
├── public/             # Logo, favicon, üniversite görselleri
├── src-tauri/          # Rust çekirdeği, pencere ayarları, platform ikonları
├── src/
│   ├── layouts/        # Dil, yön, üst başlık ve ortak gezinme
│   ├── pages/          # Astro ve MDX dosya tabanlı rotalar
│   ├── components/     # Svelte arayüzü ve React etkileşimleri
│   ├── lib/            # Menü verisi, iş kuralları, tema ve çeviri etiketleri
│   ├── types/          # Sepet ve sipariş TypeScript arayüzleri
│   └── styles/         # Ortak CSS ve iki tema
├── tests/              # İndirim ve saklanan veri doğrulama testleri
└── docs/               # Planlar, kurallar, öğretmen görevleri ve build kanıtı
```

`node_modules/`, `.astro/`, `dist/` ve `src-tauri/target/` üretilmiş çıktılardır; kaynak teslimine eklenmez. Rotalar [mimari ağaçta](mimari-agac.md), renkler [marka belgesinde](branding.md) tutulur.

---

## 🗄️ Geçici Yerel Arka Uç (`src-tauri/src/backend/`)

SQLite tabanlı geçici arka uç (migration, rol/izin, kaynak denetleyicileri, iş işlevleri) `src-tauri/src/backend/` altındadır; ayrıntı, tablolar ve komut listesi tek kaynakta: [`docs/backend.md`](backend.md).
