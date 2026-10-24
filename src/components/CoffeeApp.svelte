<script lang="ts">
  import { onMount } from "svelte";
  import Yukleniyor from '$lib/components/ui/Yukleniyor.svelte';
  import BosDurum from '$lib/components/ui/BosDurum.svelte';
  import HataDurumu from '$lib/components/ui/HataDurumu.svelte';
  import { loadList, type ListState } from '$lib/lists';
  import { messages, menuPath, type Language } from '$lib/i18n';
  import Icon from "./Icon.svelte";
  import CoffeeCard from "./CoffeeCard.svelte";
  import {
    products,
    cafes,
    brands,
    brandFor,
    sizes,
    milks,
    statuses,
    tl,
    totals,
    unitPrice,
    validCart,
    validOrders,
    mergeCart,
    emptyProfile,
    validProfile,
    type Product,
    type CartItem,
    type Order,
    type Profile,
  } from "$lib/coffee";
  let { view = "menu", productId = 1, lang = 'tr' }: { view?: string; productId?: number; lang?: Language } = $props();
  const words = $derived(messages[lang]);
  let listState = $state<ListState>('yukleniyor');
  let loadedProducts = $state<Product[]>([]);
  async function load(scenario = '') {
    listState = 'yukleniyor';
    try {
      if (view === 'orders') orders = await loadList(() => orders, scenario);
      else loadedProducts = await loadList(() => products, scenario);
      listState = 'dolu';
    } catch { listState = 'hata'; }
  }
  let cart = $state<CartItem[]>([]),
    orders = $state<Order[]>([]),
    favorites = $state<number[]>([]),
    profile = $state<Profile>({ ...emptyProfile });
  let query = $state(""),
    cafe = $state("Tümü"),
    kind = $state("Tümü"),
    sort = $state("featured"),
    onlyFavorites = $state(false);
  let size = $state("Standart"),
    milk = $state("Normal süt"),
    quantity = $state(1),
    building = $state(""),
    floor = $state(""),
    room = $state("");
  let notice = $state(""),
    noticeError = $state(false),
    busy = $state(false),
    ready = $state(false),
    lastOrder = $state<Order | null>(null),
    orderTab = $state("all"),
    confirmClear = $state(false),
    savedProfile = $state(false);
  const selected = $derived(products.find((p) => p.id === productId) ?? products[0]);
  const sum = $derived(totals(cart));
  const recommended = $derived(
    products
      .filter(
        (p) =>
          p.cafe ===
          (cart.length ? products.find((p) => p.id === cart[0].productId)!.cafe : profile.cafe),
      )
      .slice(0, 3),
  );
  const visibleOrders = $derived(
    orders.filter(
      (o) => orderTab === "all" || (orderTab === "active" ? o.status < 2 : o.status === 2),
    ),
  );
  const filtered = $derived.by(() => {
    let result = loadedProducts.filter(
      (p) =>
        (cafe === "Tümü" || p.cafe === cafe) &&
        (kind === "Tümü" || p.kind === kind) &&
        (!onlyFavorites || favorites.includes(p.id)) &&
        (p.name + " " + p.cafe).toLocaleLowerCase("tr").includes(query.toLocaleLowerCase("tr")),
    );
    if (sort === "price-up") result.sort((a, b) => a.price - b.price);
    if (sort === "price-down") result.sort((a, b) => b.price - a.price);
    return result;
  });
  const initials = $derived(
    profile.name
      .trim()
      .split(/\s+/)
      .map((s) => s[0] ?? "")
      .slice(0, 2)
      .join("")
      .toLocaleUpperCase("tr"),
  );
  const savings = $derived(orders.reduce((sum, o) => sum + o.discount, 0));
  onMount(() => {
    try {
      const c = JSON.parse(localStorage.getItem("kampus-cart") ?? "[]"),
        o = JSON.parse(localStorage.getItem("kampus-orders") ?? "[]"),
        f = JSON.parse(localStorage.getItem("kampus-favorites") ?? "[]"),
        p = JSON.parse(localStorage.getItem("kampus-profile") ?? "null");
      if (validCart(c)) cart = c;
      if (validOrders(o)) orders = o;
      if (Array.isArray(f))
        favorites = f.filter((id) => Number.isInteger(id) && products.some((p) => p.id === id));
      if (validProfile(p)) {
        profile = p;
        building = p.building;
        floor = p.floor;
        room = p.room;
      }
    } catch {
      notify("Cihazdaki kayıt okunamadı. Yeni bir kahve molasıyla başlayabilirsin.", true);
    }
    milk = selected.defaultMilk;
    if (view === 'menu') {
      const params=new URLSearchParams(window.location.search);
      query=params.get('q') ?? ''; cafe=params.get('cafe') ?? 'Tümü'; kind=params.get('kind') ?? 'Tümü'; sort=params.get('sort') ?? 'featured'; onlyFavorites=params.get('fav')==='1';
    }
    ready = true;
    void load(new URLSearchParams(window.location.search).get('durum') ?? '');
  });
  function filterParams() {
    return new URLSearchParams({q:query,cafe,kind,sort,fav:onlyFavorites?'1':'0'}).toString();
  }
  function detailLink(id: number) { return `${menuPath(lang)}urun/${id}/?${filterParams()}`; }
  $effect(()=>{
    const params=filterParams();
    if(ready && view==='menu') window.history.replaceState(null,'',`${menuPath(lang)}?${params}${window.location.hash}`);
  });
  function notify(text: string, error = false) {
    notice = text;
    noticeError = error;
  }
  function signal() {
    window.dispatchEvent(new Event("kampus:change"));
  }
  function save() {
    try {
      localStorage.setItem("kampus-cart", JSON.stringify(cart));
      localStorage.setItem("kampus-orders", JSON.stringify(orders));
      localStorage.setItem("kampus-favorites", JSON.stringify(favorites));
      signal();
    } catch {
      notify("Cihaz kaydı kullanılamıyor. Bu oturumdaki değişiklikler kaybolabilir.", true);
    }
  }
  function favorite(id: number) {
    favorites = favorites.includes(id) ? favorites.filter((x) => x !== id) : [...favorites, id];
    save();
  }
  function add(p: Product = selected, custom = false) {
    try {
      const item = {
        productId: p.id,
        size: custom ? size : "Standart",
        milk: custom ? milk : p.defaultMilk,
        quantity: custom ? quantity : 1,
      };
      cart = mergeCart(cart, [item]);
      notify(p.name + " sepete eklendi.");
      save();
    } catch (e) {
      notify((e as Error).message, true);
    }
  }
  function changeQuantity(index: number, delta: number) {
    const n = cart[index].quantity + delta;
    if (n >= 1 && n <= 99) {
      cart[index].quantity = n;
      save();
    }
  }
  function remove(index: number) {
    cart.splice(index, 1);
    save();
  }
  function saveProfile(event: SubmitEvent) {
    event.preventDefault();
    profile = {
      ...profile,
      name: profile.name.trim(),
      building: profile.building.trim(),
      floor: profile.floor.trim(),
      room: profile.room.trim(),
    };
    try {
      localStorage.setItem("kampus-profile", JSON.stringify(profile));
      signal();
      savedProfile = true;
      notify("Profilin ve teslimat tercihin kaydedildi.");
    } catch {
      notify("Profil kaydedilemedi. Tarayıcı depolama iznini kontrol et.", true);
    }
  }
  async function checkout(event: SubmitEvent) {
    event.preventDefault();
    if (busy || !ready || !cart.length) return;
    if (!building.trim() || !floor.trim() || !room.trim()) {
      notify("Bina, kat ve sınıf bilgilerini tamamla.", true);
      return;
    }
    if (orders.length >= 100) {
      notify("Geçmişte 100 sipariş var. Profil bölümünden kayıtları temizleyebilirsin.", true);
      return;
    }
    busy = true;
    try {
      let code = "";
      for (let n = 0; n < 10; n++) {
        if ("__TAURI_INTERNALS__" in window) {
          const { invoke } = await import("@tauri-apps/api/core");
          code = await invoke<string>("siparis_olustur");
        } else {
          code =
            "KMP-" +
            Array.from(crypto.getRandomValues(new Uint8Array(4)), (b) =>
              b.toString(16).padStart(2, "0"),
            )
              .join("")
              .toUpperCase();
        }
        if (!orders.some((o) => o.code === code)) break;
        code = "";
      }
      if (!/^KMP-[0-9A-F]{8}$/.test(code)) throw Error();
      lastOrder = {
        code,
        cafe: products.find((p) => p.id === cart[0].productId)!.cafe,
        items: cart.map((i) => ({ ...i })),
        address: `${building.trim()} / ${floor.trim()} / ${room.trim()}`,
        total: sum.total,
        discount: sum.discount,
        status: 0,
        created: new Date().toISOString(),
      };
      orders.unshift(lastOrder);
      cart = [];
      notice = "";
      save();
    } catch {
      notify("Sipariş oluşturulamadı. Sepetin korundu; tekrar deneyebilirsin.", true);
    } finally {
      busy = false;
    }
  }
  function reorder(order: Order) {
    try {
      cart = mergeCart(cart, order.items);
      notify("Kahvelerin sepete eklendi.");
      save();
    } catch (e) {
      notify((e as Error).message, true);
    }
  }
  function clearHistory() {
    orders = [];
    confirmClear = false;
    save();
    notify("Sipariş geçmişin temizlendi.");
  }
  function resetFilters() {
    query = "";
    cafe = "Tümü";
    kind = "Tümü";
    onlyFavorites = false;
    sort = "featured";
  }
</script>

<div class="app-page" class:menu-page={view === "menu"}>
  {#if view === "menu"}
    <section class="coffee-hero">
      <div class="hero-copy">
        <span class="eyebrow"><span class="little-line"></span>KAMPÜSÜN YENİ KAHVE MOLASI</span>
        <h1>Güzel bir mola,<br /><em>iyi bir kahveyle.</em></h1>
        <p>
          Sevdiğin kafeyi seç, kahveni tam istediğin gibi hazırla.<br class="desktop-only" /> Biz molanı
          sınıfına taşıyalım.
        </p>
        <a class="btn compact" href="#kahveler">Kahveni keşfet <Icon name="arrow" size={18} /></a>
        <div class="hero-bottom">
          <span class="mini-logos"
            >{#each brands as brand}<img
                src={brand.image}
                alt={brand.name}
                width="36"
                height="36"
              />{/each}</span
          ><span>3 sevilen kafe.<br /><strong>24 farklı kahve molası.</strong></span>
        </div>
      </div>
      <div class="hero-photo">
        <img
          src="/coffee/hero.webp"
          alt="Ahşap masa üzerinde latte art ile hazırlanmış kahve"
          width="800"
          height="750"
          fetchpriority="high"
        /><span class="hero-sticker"
          ><Icon name="coffee" size={22} /><span
            >Bugün kendine<br /><strong>bir kahve ısmarla.</strong></span
          ></span
        >
      </div>
    </section>
    <div class="benefit-row">
      <span><Icon name="pin" /> Sınıfına kadar kahve keyfi</span><span
        ><Icon name="gift" /> 5 kahvede %10 toplu sipariş indirimi</span
      ><span><Icon name="heart" /> Her zevke bir kahve</span>
    </div>
    <section class="brand-section" aria-labelledby="brands-title">
      <div class="section-heading">
        <div>
          <p class="eyebrow">BİLDİĞİN LEZZETLER</p>
          <h2 id="brands-title">Hangi kafede buluşalım?</h2>
        </div>
        <button
          class="text-button"
          onclick={() => {
            cafe = "Tümü";
          }}>Tüm kafeler <Icon name="arrow" size={16} /></button
        >
      </div>
      <div class="brand-grid">
        {#each brands as brand}<button
            class="brand-tile"
            class:chosen={cafe === brand.name}
            aria-pressed={cafe === brand.name}
            onclick={() => (cafe = cafe === brand.name ? "Tümü" : brand.name)}
            ><span class="brand-image"
              ><img src={brand.image} alt={brand.name + " logosu"} width="76" height="76" /></span
            ><span class="brand-copy"
              ><strong>{brand.name}</strong><small>{brand.label}</small><span>8 kahve seçeneği</span
              ></span
            ><Icon name="chevron" size={18} /></button
          >{/each}
      </div>
    </section>
    <section id="kahveler" class="catalog" aria-labelledby="catalog-title">
      <div class="section-heading">
        <div>
          <p class="eyebrow">MOLANIN TADI SENİN ELİNDE</p>
          <h2 id="catalog-title">{cafe === "Tümü" ? "Bugün ne içelim?" : cafe + " menüsü"}</h2>
        </div>
        <label class="sort-control"
          ><span class="sr-only">Kahveleri sırala</span><select bind:value={sort}
            ><option value="featured">Önerilen sıralama</option><option value="price-up"
              >Fiyat: düşükten yükseğe</option
            ><option value="price-down">Fiyat: yüksekten düşüğe</option></select
          ></label
        >
      </div>
      <div class="catalog-controls">
        <div class="filter-pills" aria-label="İçecek filtresi">
          {#each ["Tümü", "Sıcak", "Soğuk"] as k}<button
              class:active={kind === k}
              aria-pressed={kind === k}
              onclick={() => (kind = k)}
              >{#if k === "Tümü"}<Icon name="grid" size={16} />{:else if k === "Sıcak"}<Icon
                  name="coffee"
                  size={16}
                />{:else}<Icon name="clock" size={16} />{/if}{k === "Tümü"
                ? "Tüm kahveler"
                : k + " kahveler"}</button
            >{/each}<button
            class:active={onlyFavorites}
            aria-pressed={onlyFavorites}
            onclick={() => (onlyFavorites = !onlyFavorites)}
            ><Icon name="heart" size={16} />Favorilerim</button
          >
        </div>
        <label class="search-box"
          ><Icon name="search" size={19} /><input
            aria-label="Kahve veya kafe ara"
            type="search"
            bind:value={query}
            placeholder="Kahveni bul…"
          /></label
        >
      </div>
      <p class="results-count">
        {filtered.length} kahve seni bekliyor{cafe !== "Tümü" ? " · " + cafe : ""}
      </p>
      {#if listState === 'yukleniyor'}<Yukleniyor metin={words.loading} />
      {:else if listState === 'hata'}<HataDurumu mesaj={words.error} dugmeMetni={words.retry} tekrarDene={() => load()} />
      {:else if !filtered.length}<BosDurum baslik={words.empty} aciklama={words.emptyText} dugmeMetni={words.clear} onclick={resetFilters} />
      {:else}<div class="coffee-grid">
        {#each filtered as p}<CoffeeCard href={detailLink(p.id)} product={p} favorite={favorites.includes(p.id)} onFavorite={favorite} onAdd={(p) => add(p)} {ready} />{/each}
      </div>{/if}
    </section>
    <section class="group-banner">
      <span class="group-icon"><Icon name="gift" size={38} /></span>
      <div>
        <p class="eyebrow">KAHVE BAHANE, ARKADAŞLAR ŞAHANE.</p>
        <h2>Birlikte söyle, %10 az öde.</h2>
        <p>Aynı kafeden, aynı sınıfa 5 ve üzeri kahvede toplu sipariş indirimi.</p>
      </div>
      <a href="#kahveler" class="btn banner-button"
        >Birlikte bir mola <Icon name="arrow" size={18} /></a
      >
    </section>
  {:else if view === "detail"}
    <a href="/" class="breadcrumb"><Icon name="back" size={17} />Kahvelere dön</a>
    <div class="product-detail">
      <div class="detail-image">
        <img
          src={selected.image}
          alt={selected.name + " için kahve fotoğrafı"}
          width="880"
          height="980"
        /><button
          class="favorite-button"
          class:saved={favorites.includes(selected.id)}
          aria-label="Favori durumunu değiştir"
          aria-pressed={favorites.includes(selected.id)}
          onclick={() => favorite(selected.id)}
          disabled={!ready}><Icon name="heart" /></button
        >
      </div>
      <section class="detail-options">
        <div class="detail-brand">
          <img src={brandFor(selected.cafe).image} alt="" width="38" height="38" /><span
            >{selected.cafe}</span
          ><span class="pill">{selected.kind}</span>
        </div>
        <h1>{selected.name}</h1>
        <p class="detail-description">{selected.description}</p>
        <strong class="detail-price"
          >{tl(unitPrice({ productId: selected.id, size, milk, quantity: 1 }))}<small>
            / adet</small
          ></strong
        >
        <div class="option-group">
          <h3>Kahvenin boyu</h3>
          <div class="size-options">
            {#each sizes as s}<button
                class:chosen={size === s}
                aria-pressed={size === s}
                onclick={() => (size = s)}
                ><Icon name="coffee" size={s === "Büyük" ? 29 : 22} /><strong>{s}</strong><small
                  >{s === "Standart" ? "Klasik mola" : "+15 TL"}</small
                ></button
              >{/each}
          </div>
        </div>
        <label class="milk-label"
          >Süt tercihin<select bind:value={milk}
            >{#each milks as m}<option>{m}</option>{/each}</select
          ><small>Yulaf sütü +10 TL</small></label
        >
        <div class="detail-add">
          <div class="stepper">
            <button aria-label="Adedi azalt" disabled={quantity <= 1} onclick={() => quantity--}
              ><Icon name="minus" size={17} /></button
            ><input
              aria-label="Kahve adedi"
              type="number"
              min="1"
              max="99"
              step="1"
              bind:value={quantity}
            /><button aria-label="Adedi artır" disabled={quantity >= 99} onclick={() => quantity++}
              ><Icon name="plus" size={17} /></button
            >
          </div>
          <button class="btn" disabled={!ready} onclick={() => add(selected, true)}
            >Sepete ekle <span
              >{tl(
                unitPrice({ productId: selected.id, size, milk, quantity: 1 }) * (quantity || 1),
              )}</span
            ></button
          >
        </div>
        <a class="text-button" href="/sepet/">Sepetimi gör <Icon name="arrow" size={17} /></a>
        <div class="detail-note">
          <Icon name="gift" />
          <p>
            Arkadaşlarınla 5 kahveye tamamla,<br /><strong
              >%10 toplu sipariş indirimini yakala.</strong
            >
          </p>
        </div>
      </section>
    </div>
  {:else if view === "cart"}
    <div class="page-heading">
      <div>
        <p class="eyebrow">MOLANA BİR ADIM KALDI</p>
        <h1>Sepetim<span class="heading-count">{sum.quantity}</span></h1>
        <p>Kahvelerin hazırsa, sınıfını seçelim.</p>
      </div>
      <a class="text-button" href="/">Alışverişe devam <Icon name="arrow" size={16} /></a>
    </div>
    {#if lastOrder}<section class="success-state">
        <span class="success-icon"><Icon name="check" size={36} /></span>
        <p class="eyebrow">SİPARİŞ KAYDIN OLUŞTURULDU</p>
        <h2>Sipariş kaydın hazır!</h2>
        <p>{lastOrder.cafe} · {lastOrder.address}</p>
        <bdi class="order-code">{lastOrder.code}</bdi>
        <p class="muted">Sipariş kodunu ve kayıtlarını Siparişlerim bölümünde bulabilirsin.</p>
        <a class="btn compact" href="/siparisler/"
          >Siparişimi görüntüle <Icon name="arrow" size={17} /></a
        >
      </section>
    {:else if cart.length}<div class="checkout-grid">
        <div class="checkout-main">
          <section class="panel cart-panel">
            <div class="panel-heading">
              <img
                src={brandFor(products.find((p) => p.id === cart[0].productId)!.cafe).image}
                alt=""
                width="40"
                height="40"
              />
              <div>
                <h2>{products.find((p) => p.id === cart[0].productId)!.cafe}</h2>
                <p>Tek kafeden, aynı sınıfa.</p>
              </div>
              <span class="pill">{sum.quantity} kahve</span>
            </div>
            {#each cart as item, index}{@const p = products.find((p) => p.id === item.productId)!}
              <article class="cart-item">
                <a href={"/urun/" + p.id + "/"}
                  ><img class="cart-photo" src={p.image} alt={p.name} width="100" height="112" /></a
                >
                <div class="cart-info">
                  <a href={"/urun/" + p.id + "/"}>{p.name}</a>
                  <p>{item.size} · {item.milk}</p>
                  <div class="stepper small">
                    <button
                      aria-label={p.name + " adedini azalt"}
                      disabled={busy || item.quantity <= 1}
                      onclick={() => changeQuantity(index, -1)}
                      ><Icon name="minus" size={14} /></button
                    ><span>{item.quantity}</span><button
                      aria-label={p.name + " adedini artır"}
                      disabled={busy || item.quantity >= 99}
                      onclick={() => changeQuantity(index, 1)}
                      ><Icon name="plus" size={14} /></button
                    >
                  </div>
                </div>
                <div class="cart-item-end">
                  <button
                    class="icon-button muted"
                    aria-label={p.name + " sepetten çıkar"}
                    disabled={busy}
                    onclick={() => remove(index)}><Icon name="trash" size={18} /></button
                  ><strong>{tl(unitPrice(item) * item.quantity)}</strong>
                </div>
              </article>{/each}
          </section>
          <div class="discount-card">
            <Icon name="gift" size={25} />
            <div>
              <strong
                >{sum.quantity >= 5
                  ? "Güzel haber: %10 indirim senin!"
                  : `İndirime sadece ${5 - sum.quantity} kahve kaldı.`}</strong
              >
              <p>
                {sum.quantity >= 5
                  ? tl(sum.discount) + " daha az ödeyeceksin."
                  : "Arkadaşlarını da kahve molasına davet et."}
              </p>
              <div
                class="progress-track"
                role="progressbar"
                aria-label="Toplu sipariş indirimi ilerlemesi"
                aria-valuenow={Math.min(sum.quantity, 5)}
                aria-valuemin="0"
                aria-valuemax="5"
              >
                <span style:width={Math.min((sum.quantity / 5) * 100, 100) + "%"}></span>
              </div>
            </div>
          </div>
        </div>
        <form class="panel order-summary" onsubmit={checkout}>
          <h2>Sipariş özeti</h2>
          <div class="address-title">
            <Icon name="pin" />
            <h3>Nereye gelsin?</h3>
          </div>
          <label
            >Bina<input
              name="building"
              required
              maxlength="50"
              bind:value={building}
              placeholder="Örn. A Blok"
            /></label
          >
          <div class="two-fields">
            <label
              >Kat<input
                name="floor"
                required
                maxlength="20"
                bind:value={floor}
                placeholder="2"
              /></label
            ><label
              >Sınıf<input
                name="room"
                required
                maxlength="50"
                bind:value={room}
                placeholder="A-203"
              /></label
            >
          </div>
          <div class="summary-lines">
            <div><span>Ara toplam</span><span>{tl(sum.subtotal)}</span></div>
            <div class:discount-line={sum.discount > 0}>
              <span>Toplu sipariş indirimi</span><span>−{tl(sum.discount)}</span>
            </div>
            <div><span>Teslimat</span><span>Ücret eklenmez</span></div>
          </div>
          <div class="summary-total"><span>Toplam</span><strong>{tl(sum.total)}</strong></div>
          <button class="btn" disabled={!ready || busy}
            >{busy ? "Oluşturuluyor…" : "Siparişi oluştur"}<Icon name="arrow" size={18} /></button
          >
          <p class="checkout-note">
            <Icon name="shield" size={16} />Devam etmeden önce sınıf bilgilerini kontrol et.
          </p>
        </form>
      </div>
    {:else}<section class="empty-state large">
        <span class="empty-icon"><Icon name="bag" size={38} /></span>
        <p class="eyebrow">BİR KAHVEYE NE DERSİN?</p>
        <h2>Sepetin henüz boş.</h2>
        <p>
          İster sıcak bir latte, ister buz gibi bir americano.<br />Sana iyi gelecek kahve burada.
        </p>
        <a class="btn compact" href="/">Kahvemi seçeyim <Icon name="arrow" size={18} /></a>
      </section>{/if}
    {#if !lastOrder}<section class="recommendations">
        <div class="section-heading">
          <div>
            <p class="eyebrow">MOLANA EŞLİK ETSİN</p>
            <h2>Bunları da sevebilirsin.</h2>
          </div>
        </div>
        <div class="coffee-grid">
          {#each recommended as p}<CoffeeCard
              product={p}
              favorite={favorites.includes(p.id)}
              onFavorite={favorite}
              onAdd={(p) => add(p)}
              {ready}
            />{/each}
        </div>
      </section>{/if}
  {:else if view === "orders"}
    <div class="page-heading">
      <div>
        <p class="eyebrow">HER KAHVENİN BİR HİKÂYESİ VAR</p>
        <h1>Siparişlerim</h1>
        <p>Kahve molaların ve sipariş durumların bir arada.</p>
      </div>
      <span class="soft-label"><Icon name="coffee" size={18} />{orders.length} kahve molası</span>
    </div>
    <div class="filter-pills order-tabs">
      {#each [["all", "Tümü"], ["active", "Devam eden"], ["past", "Tamamlanan"]] as tab}<button
          class:active={orderTab === tab[0]}
          aria-pressed={orderTab === tab[0]}
          onclick={() => (orderTab = tab[0])}
          >{tab[1]}<span
            >{orders.filter(
              (o) => tab[0] === "all" || (tab[0] === "active" ? o.status < 2 : o.status === 2),
            ).length}</span
          ></button
        >{/each}
    </div>
    {#if listState === 'yukleniyor'}<Yukleniyor metin={words.loading} />
    {:else if listState === 'hata'}<HataDurumu mesaj={words.error} dugmeMetni={words.retry} tekrarDene={() => load()} />
    {:else if !visibleOrders.length}<BosDurum baslik={words.orders} aciklama={words.orderText} dugmeMetni={words.menu} onclick={() => { window.location.href = menuPath(lang); }} />{/if}
    <div class="orders-list">
      {#each (listState === 'dolu' ? visibleOrders : []) as order}<article class="panel order-card">
          <div class="order-card-top">
            <div class="order-brand">
              <img src={brandFor(order.cafe).image} alt="" width="48" height="48" />
              <div>
                <h2>{order.cafe}</h2>
                <p>
                  {new Date(order.created).toLocaleDateString("tr-TR", {
                    day: "numeric",
                    month: "long",
                  })} · {new Date(order.created).toLocaleTimeString("tr-TR", {
                    hour: "2-digit",
                    minute: "2-digit",
                  })}
                </p>
              </div>
            </div>
            <span class="status-pill" class:complete={order.status === 2}
              ><span></span>{statuses[order.status]}</span
            >
          </div>
          <div class="order-product-row">
            <div class="order-thumbnails">
              {#each order.items.slice(0, 3) as item}{@const p = products.find(
                  (p) => p.id === item.productId,
                )!}<img src={p.image} alt={p.name} width="64" height="64" />{/each}
            </div>
            <div>
              <strong>{order.items.reduce((n, i) => n + i.quantity, 0)} kahve</strong>
              <p><Icon name="pin" size={14} />{order.address}</p>
            </div>
            <strong class="order-total">{tl(order.total)}</strong>
          </div>
          <ol class="order-timeline" aria-label="Sipariş durumu">
            {#each statuses as status, i}<li
                class:done={order.status >= i}
                aria-current={order.status === i ? "step" : undefined}
              >
                <span
                  ><Icon name={i === 0 ? "coffee" : i === 1 ? "truck" : "check"} size={17} /></span
                ><strong>{status}</strong>
              </li>{/each}
          </ol>
          <details class="order-details">
            <summary
              >Sipariş detayları <bdi>{order.code}</bdi><Icon name="chevron" size={15} /></summary
            >
            <ul>
              {#each order.items as item}<li>
                  <span
                    >{item.quantity} × {products.find((p) => p.id === item.productId)!.name}<small
                      >{item.size} · {item.milk}</small
                    ></span
                  >
                </li>{/each}
            </ul>
            <p>Toplu sipariş indirimi: {tl(order.discount)}</p>
          </details>
          <div class="order-actions">
            <small>Sipariş durumunu buradan güncelleyebilirsin.</small>{#if order.status < 2}<button
                class="text-button"
                onclick={() => {
                  order.status = order.status === 0 ? 1 : 2;
                  save();
                }}>Sonraki durum <Icon name="chevron" size={15} /></button
              >{/if}<button class="outline compact" onclick={() => reorder(order)}
              ><Icon name="refresh" size={17} />Tekrar sipariş ver</button
            >
          </div>
        </article>{/each}
    </div>
  {:else}
    <div class="page-heading">
      <div>
        <p class="eyebrow">SANA AİT BİR KAHVE KÖŞESİ</p>
        <h1>Profilim</h1>
        <p>Küçük tercihler, tam sana göre bir mola.</p>
      </div>
    </div>
    <div class="profile-grid">
      <aside class="profile-aside">
        <section class="panel identity-card">
          <span class="profile-avatar"
            >{#if initials}{initials}{:else}<Icon name="user" size={40} />{/if}</span
          >
          <h2>{profile.name.trim() || "Kahve dostu"}</h2>
          <p>Bir kahve, bin güzel sohbet.</p>
          <span class="soft-label">Öğrenci profili</span>
          <div class="profile-stats">
            <a href="/siparisler/"><strong>{orders.length}</strong><span>Sipariş</span></a><a
              href="#favoriler"><strong>{favorites.length}</strong><span>Favori</span></a
            ><span><strong>{tl(savings)}</strong><span>İndirim</span></span>
          </div>
        </section>
        <section class="panel profile-links">
          <h3>Biraz daha keşfet</h3>
          {#each [["/hakkinda/", "coffee", "Hakkımızda"], ["/iletisim/", "user", "Bize ulaş"], ["/gizlilik/", "shield", "Gizlilik"], ["/kosullar/", "orders", "Kullanım koşulları"]] as row}<a
              href={row[0]}
              ><Icon name={row[1]} size={18} />{row[2]}<Icon name="chevron" size={15} /></a
            >{/each}
        </section>
      </aside>
      <div class="profile-main">
        <form class="panel profile-form" onsubmit={saveProfile}>
          <div class="panel-title">
            <Icon name="user" />
            <div>
              <h2>Kişisel tercihlerim</h2>
              <p>Bilgilerin yalnızca bu cihazda saklanır.</p>
            </div>
          </div>
          <div class="two-fields">
            <label
              >Kahvene hangi ismi yazalım?<input
                name="name"
                maxlength="50"
                bind:value={profile.name}
                placeholder="İsim"
                oninput={() => (savedProfile = false)}
              /></label
            ><label
              >Favori kafen<select bind:value={profile.cafe}
                >{#each cafes as c}<option>{c}</option>{/each}</select
              ></label
            >
          </div>
          <div class="panel-title address-section" id="teslimat">
            <Icon name="pin" />
            <div>
              <h2>Kahve durağım</h2>
              <p>Sınıfını kaydet, siparişlerinde hazır olsun.</p>
            </div>
          </div>
          <label
            >Bina<input
              name="building"
              maxlength="50"
              bind:value={profile.building}
              placeholder="Örn. A Blok"
            /></label
          >
          <div class="two-fields">
            <label
              >Kat<input
                name="floor"
                maxlength="20"
                bind:value={profile.floor}
                placeholder="Örn. 2"
              /></label
            ><label
              >Sınıf<input
                name="room"
                maxlength="50"
                bind:value={profile.room}
                placeholder="Örn. A-203"
              /></label
            >
          </div>
          <div class="profile-save">
            <small
              ><Icon name="shield" size={15} />Tercihlerini istediğin zaman değiştirebilirsin.</small
            ><button class="btn compact" disabled={!ready}
              ><Icon name="check" size={17} />{savedProfile
                ? "Kaydedildi"
                : "Tercihlerimi kaydet"}</button
            >
          </div>
        </form>
        <section id="favoriler" class="panel favorite-panel">
          <div class="panel-title">
            <Icon name="heart" />
            <div>
              <h2>Favori kahvelerim</h2>
              <p>Bir sonraki molana sakladıkların.</p>
            </div>
          </div>
          {#if favorites.length}<div class="favorite-list">
              {#each products.filter((p) => favorites.includes(p.id)) as p}<div
                  class="favorite-row"
                >
                  <a href={"/urun/" + p.id + "/"}
                    ><img src={p.image} alt="" width="58" height="58" /><span
                      ><strong>{p.name}</strong><small>{p.cafe}</small></span
                    ></a
                  ><span>{tl(p.price)}</span><button
                    class="icon-button"
                    aria-label={p.name + " favorilerden çıkar"}
                    onclick={() => favorite(p.id)}><Icon name="close" size={17} /></button
                  >
                </div>{/each}
            </div>{:else}<p class="favorite-empty">
              Menüdeki kalbe dokun; sevdiğin kahveler burada toplansın.
            </p>
            <a class="text-button" href="/">Favori kahvemi bul <Icon name="arrow" size={16} /></a
            >{/if}
        </section>
        <section class="data-settings">
          <span
            ><strong>Cihazdaki sipariş geçmişi</strong><small
              >Silmek sepetini, profilini veya favorilerini etkilemez.</small
            ></span
          >{#if confirmClear}<span class="confirm-actions"
              ><button class="text-button" onclick={() => (confirmClear = false)}>Vazgeç</button
              ><button class="danger-button" onclick={clearHistory}>Geçmişi sil</button></span
            >{:else}<button
              class="text-button danger-text"
              disabled={!orders.length}
              onclick={() => (confirmClear = true)}>Geçmişi temizle</button
            >{/if}
        </section>
      </div>
    </div>
  {/if}
</div>
{#if notice}<div class="toast" class:error={noticeError} role="status" aria-live="polite">
    <Icon name={noticeError ? "coffee" : "check"} size={20} />
    <p>{notice}</p>
    {#if !noticeError && view !== "profile"}<a href="/sepet/">Sepeti gör</a>{/if}<button
      class="icon-button"
      aria-label="Bildirimi kapat"
      onclick={() => (notice = "")}><Icon name="close" size={17} /></button
    >
  </div>{/if}
