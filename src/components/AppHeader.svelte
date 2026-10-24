<script lang="ts">
  import { onMount } from "svelte";
  import { tema } from "$lib/tema.svelte";
  import { validProfile } from "$lib/coffee";
  import Icon from "./Icon.svelte";
  import AppNav from "./AppNav.svelte";
  let { lang = "tr", currentPath = "/" }: { lang?: string; currentPath?: string } = $props();
  let destination = $state("Sınıfını seç");
  let initials = $state("");
  const labels: Record<string, string> = {
    tr: "Açık / koyu temayı değiştir",
    en: "Toggle light / dark theme",
    ar: "تبديل المظهر",
    fa: "تغییر پوسته",
  };
  onMount(() => {
    tema.oku();
    const read = () => {
      try {
        const p = JSON.parse(localStorage.getItem("kampus-profile") ?? "null");
        if (validProfile(p)) {
          initials = p.name
            .trim()
            .split(/\s+/)
            .map((v) => v[0] ?? "")
            .slice(0, 2)
            .join("")
            .toLocaleUpperCase("tr");
          destination = p.building && p.room ? p.building + " · " + p.room : "Sınıfını seç";
        }
      } catch {}
    };
    read();
    window.addEventListener("kampus:change", read);
    return () => window.removeEventListener("kampus:change", read);
  });
</script>

<header class="site-header">
  <div class="header-inner">
    <a href="/" class="wordmark"
      ><span class="mark"><img src="/brand-mark.png" alt="" width="36" height="36" /></span><span
        >Kampus's <b>Coffee</b><small
          >{lang === "tr"
            ? "HER MOLAYA BİR KAHVE"
            : lang === "en"
              ? "A COFFEE FOR EVERY BREAK"
              : lang === "ar"
                ? "قهوة لكل استراحة"
                : "قهوه برای هر استراحت"}</small
        ></span
      ></a
    ><AppNav {lang} {currentPath} />
    <div class="header-actions">
      {#if lang === "tr"}<a class="delivery-link" href="/profil/#teslimat"
          ><Icon name="pin" /><span
            ><small>Teslimat noktası</small><strong>{destination}</strong></span
          ><Icon name="chevron" size={14} /></a
        >{/if}<button
        class="icon-button theme-button"
        onclick={() => tema.degistir()}
        aria-label={labels[lang] ?? labels.tr}
        ><Icon name={tema.mod === "gece" ? "sun" : "moon"} /></button
      ><a class="header-avatar" href="/profil/" aria-label={lang === "tr" ? "Profilim" : "Profile"}
        >{#if initials}{initials}{:else}<Icon name="user" size={18} />{/if}</a
      >
    </div>
  </div>
</header>
