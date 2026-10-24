<script lang="ts">
 import { onMount } from 'svelte';
 import { products, sizes, milks, tl, unitPrice, mergeCart, validCart } from '$lib/coffee';
 import { messages, menuPath, type Language } from '$lib/i18n';
 import HataDurumu from '$lib/components/ui/HataDurumu.svelte';
 let {productId=-1,lang='tr'}: {productId?:number;lang?:Language} = $props();
 const product = $derived(products.find(p=>p.id===productId));
 const words = $derived(messages[lang]);
 let size = $state('Standart'),milk=$state('Normal süt'),quantity=$state(1),notice=$state(''),error=$state(false),ready=$state(false),back=$state(menuPath(lang));
 onMount(()=>{milk=product?.defaultMilk ?? 'Normal süt';back=menuPath(lang)+window.location.search;ready=true;});
 function add(){
  if(!product)return;
  try {
   const value:unknown=JSON.parse(localStorage.getItem('kampus-cart') ?? '[]');
   if(!validCart(value))throw new Error('Invalid cart');
   const cart=mergeCart(value,[{productId:product.id,size,milk,quantity}]);
   localStorage.setItem('kampus-cart',JSON.stringify(cart));
   window.dispatchEvent(new Event('kampus:change'));notice=words.added;error=false;
  } catch {error=true;notice=words.failed;}
 }
</script>
<div class="app-page">
 <a class="breadcrumb" href={back}>{words.menu}</a>
 {#if product}
 <div class="product-detail"><div class="detail-image"><img src={product.image} alt={product.name} width="880" height="980" /></div>
 <section class="detail-options"><span class="pill">{product.cafe} · {product.kind}</span><h1>{product.name}</h1><p class="detail-description">{product.description}</p>
 <strong class="detail-price">{tl(unitPrice({productId:product.id,size,milk,quantity:1}))}</strong>
 <label class="milk-label">{words.size}<select bind:value={size}>{#each sizes as s,i}<option value={s}>{i === 0 ? words.standard : words.large}</option>{/each}</select></label>
 <label class="milk-label">{words.milk}<select bind:value={milk}>{#each milks as m,i}<option value={m}>{({tr:milks,en:['Dairy milk','Oat milk','No milk'],ar:['حليب عادي','حليب الشوفان','دون حليب'],fa:['شیر معمولی','شیر جو دوسر','بدون شیر']})[lang][i]}</option>{/each}</select></label>
 <label>{words.quantity}<input type="number" min="1" max="99" step="1" bind:value={quantity} /></label>
 <button class="btn" disabled={!ready} onclick={add}>{words.add} · {tl(unitPrice({productId:product.id,size,milk,quantity:1})*(quantity||1))}</button>
 {#if error}<HataDurumu mesaj={notice} dugmeMetni={words.retry} tekrarDene={add} />{:else if notice}<p role="status">{notice}</p>{/if}
 </section></div>
 {:else}<HataDurumu mesaj={words.missing} dugmeMetni={words.menu} tekrarDene={()=>{window.location.href=back;}} /><p>{words.missingText}</p>{/if}
</div>
