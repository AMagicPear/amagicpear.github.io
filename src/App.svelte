<script lang="ts">
  import PerryHeader from "./lib/PerryHeader.svelte";
  import LeftCard from "./lib/LeftCard.svelte";
  import PerryWaves from "./lib/PerryWaves.svelte";
</script>

<main>
  <section id="top-showcase">
    <PerryHeader />
    <div class="background-element">
      <PerryWaves />
    </div>
    <div class="floating-element">
      <LeftCard />
    </div>
  </section>
  <section id="sub-content">
    <h2>Sub Content</h2>
  </section>
</main>

<footer>
  <p>AMagicPear &copy; 2025</p>
</footer>

<style>
  #top-showcase {
    position: relative;
    width: 100%;
    height: 100vh;
    /* background-color: #212420; */
    /* 改成渐变 */
    background: linear-gradient(160deg, #242320 0%, #181a18 100%);
  }

  .background-element {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .floating-element {
    position: absolute;
    top: 55%;
    left: 30%;
    transform: translate(-50%, -50%);
    /* width: 300px; */
    /* height: 200px; */
    z-index: 3;
  }
</style>
