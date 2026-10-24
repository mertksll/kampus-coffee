<script lang="ts">let {baslik,aciklama,dugmeMetni,onclick}: {baslik:string;aciklama:string;dugmeMetni?:string;onclick?:()=>void} = $props();</script>
<section class="empty-state" role="status"><h2>{baslik}</h2><p>{aciklama}</p>{#if dugmeMetni && onclick}<button class="btn compact" {onclick}>{dugmeMetni}</button>{/if}</section>
