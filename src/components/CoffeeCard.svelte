<script lang="ts">
  import Kart from "$lib/components/ui/Kart.svelte";
  import Icon from "./Icon.svelte";
  import { tl, brandFor, type Product } from "$lib/coffee";
  let {
    product,
    href,
    favorite = false,
    onFavorite,
    onAdd,
    ready = true,
  }: {
    product: Product;
    href?: string;
    favorite?: boolean;
    onFavorite: (id: number) => void;
    onAdd: (product: Product) => void;
    ready?: boolean;
  } = $props();
</script>
<Kart baslik={product.name} altMetin={`${product.cafe} · ${product.tag ?? product.description} · ${tl(product.price)}`} gorsel={product.image} gorselAlt={product.name + ' kahve fotoğrafı'} etiket={product.kind} href={href ?? '/urun/' + product.id + '/'}>
 <button class="text-button" disabled={!ready} aria-pressed={favorite} aria-label={product.name + ' favori durumunu değiştir'} onclick={() => onFavorite(product.id)}><Icon name="heart" size={18} />{favorite ? 'Favoride' : 'Favori'}</button>
 <button class="btn compact" disabled={!ready} aria-label={product.name + ' standart boy sepete ekle'} onclick={() => onAdd(product)}><Icon name="plus" size={18} />{tl(product.price)}</button>
</Kart>
