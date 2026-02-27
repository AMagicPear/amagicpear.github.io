<script lang="ts">
  import PerryHeader from "@/components/PerryHeader.svelte";
  import { _ } from "svelte-i18n";
  import Home from "./pages/Home.svelte";
  import page from "page";
  import Essay from "./pages/Essay.svelte";
  import { currentPage } from "./lib/stores.js";
  import { onMount } from "svelte";

  page("/", () => {
    console.log("[page] /");
    currentPage.set("home");
  });

  page("/essay", () => {
    console.log("[page] essay");
    currentPage.set("essay");
  });

  onMount(() => {
    // page.start();
    return () => {
      page.stop();
    };
  });

  const PageComponent = $derived.by(() => {
    switch ($currentPage) {
      case "home":
        return Home;
      case "essay":
        return Essay;
      default:
        return Home;
    }
  });
</script>

<PerryHeader />
<div class="foreground"></div>
<main>
  <PageComponent/>
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

  footer {
    margin-bottom: 20px;
    p {
      font-size: 14px;
      line-height: 1rem;
    }
  }
</style>
