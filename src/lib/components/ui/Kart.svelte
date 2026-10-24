<script lang="ts">
 import type { KartProps } from '$lib/types';
 let {baslik, altMetin, gorsel, gorselAlt, etiket, etiketTuru='bilgi', href, onclick, children}: KartProps = $props();
</script>
<article class="kart">
 {#snippet content()}
  {#if gorsel}<img src={gorsel} alt={gorselAlt ?? baslik} width="640" height="480" loading="lazy" />{/if}
  <div class="kart-copy">
   {#if etiket}<span class="kart-tag" data-tur={etiketTuru}>{etiket}</span>{/if}
   <h3>{baslik}</h3>
   {#if altMetin}<p>{altMetin}</p>{/if}
  </div>
 {/snippet}
 {#if href}<a class="kart-main" {href}>{@render content()}</a>
 {:else if onclick}<button class="kart-main" {onclick}>{@render content()}</button>
 {:else}<div class="kart-main">{@render content()}</div>{/if}
 {#if children}<div class="kart-actions">{@render children()}</div>{/if}
</article>
<style>
 .kart{min-inline-size:0;overflow:hidden;border:var(--ui-border) solid var(--kenar);border-radius:var(--radius);background:var(--kart)}
 .kart-main{display:block;inline-size:100%;text-align:start;border:0;background:var(--kart);color:var(--yazi);padding:0}
 .kart-main:focus-visible{outline:var(--ui-focus) solid var(--renk-ana);outline-offset:calc(-1 * var(--ui-focus))}
 img{inline-size:100%;block-size:var(--ui-photo);object-fit:cover}
 .kart-copy,.kart-actions{padding:var(--ui-space)}
 h3,p{overflow-wrap:anywhere;margin-block:var(--ui-gap)}
 p{color:var(--yazi-soluk)}
 .kart-tag{display:inline-block;padding:var(--ui-gap);border-radius:var(--radius);background:var(--vurgu);color:var(--renk-ana)}
 .kart-tag[data-tur="hata"]{background:var(--danger-zemin);color:var(--danger)}
 .kart-tag[data-tur="uyari"]{border:var(--ui-border) dashed var(--yazi-soluk)}
 .kart-tag[data-tur="basari"]{border:var(--ui-border) solid var(--renk-ana)}
 .kart-actions{padding-block-start:0;display:flex;justify-content:space-between;gap:var(--ui-gap);flex-wrap:wrap}
</style>
