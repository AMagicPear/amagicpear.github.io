<script lang="ts">
  import ChaperHeader from "./ChaperHeader.svelte";
  import CoolCard from "./CoolCard.svelte";
  import worksData from "@/data/works";
  import { _, locale } from "svelte-i18n";
  import type { Language } from "@/i18n";
  
  let currentWorks = $derived(worksData[$locale as Language] || worksData["en"]);
</script>

<ChaperHeader title={$_("myworks.title")} />
<div class="works-container">
  {#each currentWorks as work}
    <CoolCard {...work} />
  {/each}
</div>

<style lang="scss">
  .works-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
  }
</style>