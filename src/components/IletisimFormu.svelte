<script lang="ts">
  import { contactLabels } from "../lib/contact";
  let { lang = "tr" }: { lang?: string } = $props();
  let name = $state(""),
    email = $state(""),
    subject = $state(""),
    message = $state(""),
    sent = $state(false);
  const t = $derived(contactLabels[lang] ?? contactLabels.tr);
  function submit(event: SubmitEvent) {
    event.preventDefault();
    name = "";
    email = "";
    subject = "";
    message = "";
    sent = true;
  }
</script>

<form class="form-stack" onsubmit={submit}>
  <label
    >{t[0]}<input
      name="name"
      autocomplete="name"
      required
      minlength="2"
      maxlength="80"
      bind:value={name}
    /></label
  >
  <label
    >{t[1]}<input
      name="email"
      type="email"
      autocomplete="email"
      required
      maxlength="160"
      bind:value={email}
    /></label
  >
  <label>{t[2]}<input name="subject" required maxlength="120" bind:value={subject} /></label>
  <label
    >{t[3]}<textarea
      name="message"
      required
      minlength="5"
      maxlength="2000"
      rows="5"
      bind:value={message}></textarea></label
  >
  <button type="submit" class="btn">{t[4]}</button>
  <p role="status" aria-live="polite">{sent ? t[5] : ""}</p>
</form>
