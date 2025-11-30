<script lang="ts">
  import PerryHeader from "./components/PerryHeader.svelte";
  import LeftCard from "./components/LeftCard.svelte";
  import PerryWaves from "./components/PerryWaves.svelte";
</script>

<div class="background"></div>
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

  .background {
    pointer-events: none;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100vh;
    background-size: 128px;
    background-repeat: repeat;
    background-image: url("./assets/rR6HYXBrMmX4cRpXfXUOvpvpB0.png");
    opacity: 0.06;
    border-radius: 0;
    z-index: 400;
  }

  footer {
    position: fixed;
    left: 2vw;
    bottom: 2px;
    z-index: 50;
    p {
      color: rgb(210, 246, 210);
      font-size: 14px;
      line-height: 1rem;
    }
  }
</style>
