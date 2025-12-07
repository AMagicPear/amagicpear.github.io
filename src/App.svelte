<script lang="ts">
  import PerryHeader from "@/components/PerryHeader.svelte";
  import MyWorks from "@/components/MyWorks.svelte";
  import { _ } from "svelte-i18n";
  import TopShowcase from "./components/TopShowcase.svelte";
  import EchoNotes from "./components/EchoNotes.svelte";
</script>

<PerryHeader />
<EchoNotes />
<div class="foreground"></div>
<main>
  <TopShowcase />
  <section id="sub-content">
    <MyWorks />
    <h2>{$_("menu.constructing")}……</h2>
  </section>
</main>

<footer>
  <p>AMagicPear &copy; 2025</p>
</footer>

<style lang="scss">
  .foreground {
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

  #sub-content {
    @media screen and (min-width: 1020px) {
      padding-inline: 10vw;
    }
  }

  footer {
    margin-bottom: 20px;
    p {
      font-size: 14px;
      line-height: 1rem;
    }
  }
</style>
