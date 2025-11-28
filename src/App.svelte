<script lang="ts">
  import LeftCard from "./lib/LeftCard.svelte";
  import PerryWaves from "./lib/PerryWaves.svelte";
</script>

<main>
  <section id="top-showcase">
    <div id="header">
      <h1>Perry Home</h1>
    </div>
    <div class="background-element">
      <PerryWaves />
    </div>
    <div class="floating-element">
      <LeftCard />
    </div>
  </section>
  <section id="sub-content">
    {#each Array.from({ length: 10 }, (_, i) => i) as item}
      <h2>{item}</h2>
    {/each}
  </section>
</main>

<footer>
  <p>AMagicPear &copy; 2025</p>
</footer>

<style>
  #header {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 60px;
    color: whitesmoke;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: space-between;
    padding-inline: 20px;
    z-index: 100;
    font-family: 'Courier New', Courier, monospace;
  }

  #top-showcase {
    position: relative;
    width: 100%;
    height: 100vh;
    background-color: #212420;
  }

  .background-element {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .floating-element {
    position: absolute;
    top: 50%;
    left: 30%;
    transform: translate(-50%, -50%);
    width: 300px;
    height: 200px;
    z-index: 3;
    /* pointer-events: none; */
  }
</style>
