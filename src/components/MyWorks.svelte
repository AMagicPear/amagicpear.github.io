<script lang="ts">
  import ChaperHeader from "./ChaperHeader.svelte";
  import CoolCard from "./CoolCard.svelte";
  import works from "@/data/works.json"
</script>

<ChaperHeader title="我的作品" />
<div class="works-container">
  {#each works as work}
    <CoolCard cardTitle={work.title} description={work.description} classify={work.classify} link={work.link}/>
  {/each}
</div>

<style lang="scss">
  .works-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    /* max-width: ; */
  }
</style>
