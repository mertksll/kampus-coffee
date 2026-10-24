<script lang="ts">let {mesaj,tekrarDene,dugmeMetni='Tekrar dene'}: {mesaj:string;tekrarDene:()=>void;dugmeMetni?:string} = $props();</script>
<section class="empty-state" role="alert"><h2>{mesaj}</h2><button class="btn compact" onclick={tekrarDene}>{dugmeMetni}</button></section>
