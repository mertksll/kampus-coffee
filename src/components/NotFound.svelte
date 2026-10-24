<script lang="ts">
 import {onMount} from 'svelte';
 import ProductDetail from './ProductDetail.svelte';
 import {language,type Language} from '$lib/i18n';
 let lang=$state<Language>('tr');
 onMount(()=>{lang=language(window.location.pathname.split('/')[1]);document.documentElement.lang=lang;document.documentElement.dir=lang==='ar'||lang==='fa'?'rtl':'ltr';});
</script>
<ProductDetail productId={-1} {lang} />
