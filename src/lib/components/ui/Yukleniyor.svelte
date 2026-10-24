<script lang="ts">let {metin='Yükleniyor'}: {metin?: string} = $props();</script>
<div class="empty-state" role="status" aria-busy="true"><p>{metin}</p><div class="skeleton"></div><div class="skeleton"></div><div class="skeleton"></div></div>
<style>.skeleton{height:var(--ui-space);margin-block:var(--ui-gap);border-radius:var(--radius);background:var(--kenar)}</style>
