<script lang="ts">
  import ChaperHeader from "./ChaperHeader.svelte";
  import CoolCard from "./CoolCard.svelte";
  const worksPromise = import("@/data/works");
</script>

<ChaperHeader title="我的作品" />
<div class="works-container">
  {#await worksPromise then works}
    {#each works.default as work}
      <CoolCard {...work} />
    {/each}
  {/await}
</div>

<style lang="scss">
  .works-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
  }
</style>
