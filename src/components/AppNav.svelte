<script lang="ts">
  import { onMount } from "svelte";
  import {menuPath, language} from "$lib/i18n";
  import Icon from "./Icon.svelte";
  import { validCart, totals } from "$lib/coffee";
  let { currentPath = "/", lang = "tr" }: { currentPath?: string; lang?: string } = $props();
  let count = $state(0);
  const names: Record<string, string[]> = {
    tr: ["Keşfet", "Sepetim", "Siparişlerim", "Profilim"],
    en: ["Explore", "My cart", "My orders", "Profile"],
    ar: ["استكشف", "سلتي", "طلباتي", "الملف"],
    fa: ["کشف", "سبد من", "سفارش‌ها", "پروفایل"],
  };
  const routes = $derived([menuPath(language(lang)), "/sepet/", menuPath(language(lang))+"siparisler/", "/profil/"]);
  const icons = ["coffee", "bag", "orders", "user"];
  onMount(() => {
    const read = () => {
      try {
        const data = JSON.parse(localStorage.getItem("kampus-cart") ?? "[]");
        count = validCart(data) ? totals(data).quantity : 0;
      } catch {
        count = 0;
      }
    };
    read();
    window.addEventListener("kampus:change", read);
    window.addEventListener("storage", read);
    return () => {
      window.removeEventListener("kampus:change", read);
      window.removeEventListener("storage", read);
    };
  });
</script>

<nav
  class="app-nav"
  aria-label={lang === "tr"
    ? "Ana menü"
    : lang === "ar"
      ? "القائمة الرئيسية"
      : lang === "fa"
        ? "منوی اصلی"
        : "Main navigation"}
>
  {#each routes as route, i}<a
      href={route}
      aria-current={currentPath === route || (i === 0 && currentPath.startsWith("/urun/"))
        ? "page"
        : undefined}
      ><span class="nav-icon"
        ><Icon name={icons[i]} size={21} />{#if i === 1 && count > 0}<span class="cart-count"
            >{count > 99 ? "99+" : count}</span
          >{/if}</span
      ><span>{(names[lang] ?? names.tr)[i]}</span></a
    >{/each}
</nav>
