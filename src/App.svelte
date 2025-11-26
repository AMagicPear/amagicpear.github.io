<script lang="ts">
  import LeftCard from "./lib/LeftCard.svelte";
  </script>

<main>
  <section id="top-showcase">
    <!-- <h1>amagicpear new site test</h1> -->
     <LeftCard/>
  </section>
  <section id="sub-content">
    <h2>More Info</h2>
  </section>
</main>

<style>
  #top-showcase {
    width: 100%;
    height: 100vh;
    background-color: #212420;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

</style>
