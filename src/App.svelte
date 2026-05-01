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
    page.start();
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

<div class="texture-overlay"></div>

<main>
  <PageComponent/>
</main>

<footer>
  <div class="footer-rule"></div>
  <div class="footer-content">
    <p>AMagicPear &copy; 2025</p>
    <span class="footer-dot"></span>
    <p class="footer-tagline">Perry Home</p>
  </div>
</footer>

<style lang="scss">
  .texture-overlay {
    pointer-events: none;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100vh;
    background-size: 200px;
    background-repeat: repeat;
    background-image: url("./assets/rR6HYXBrMmX4cRpXfXUOvpvpB0.png");
    opacity: 0.04;
    border-radius: 0;
    z-index: 400;
    mix-blend-mode: overlay;
  }

  footer {
    position: relative;
    padding: 60px 20px 40px;
    z-index: 2;

    .footer-rule {
      width: 80px;
      height: 1px;
      background: linear-gradient(
        90deg,
        transparent,
        var(--color-chartreuse-dim),
        transparent
      );
      margin: 0 auto 24px;
      opacity: 0.4;
    }

    .footer-content {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 12px;
      flex-wrap: wrap;
    }

    p {
      font-family: var(--font-body);
      font-size: 13px;
      color: var(--color-text-muted);
      line-height: 1;
    }

    .footer-dot {
      width: 3px;
      height: 3px;
      border-radius: 50%;
      background: var(--color-chartreuse-dim);
      opacity: 0.3;
    }

    .footer-tagline {
      font-family: var(--font-display);
      text-transform: uppercase;
      letter-spacing: 0.2em;
      font-size: 11px;
    }
  }
</style>
