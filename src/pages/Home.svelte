<script>
  import EchoNotes from "@/components/EchoNotes.svelte";
  import MyWorks from "@/components/MyWorks.svelte";
  import TopShowcase from "@/components/TopShowcase.svelte";
  import AboutMe from "@/components/AboutMe.svelte";
  import SkillStats from "@/components/SkillStats.svelte";
  import { _ } from "svelte-i18n";
</script>

<EchoNotes />
<TopShowcase />

<section id="sub-content">
  <AboutMe />
  <SkillStats />
  <MyWorks />

  <div class="constructing-notice">
    <span class="constructing-dot"></span>
    <span>{$_("menu.constructing")}……</span>
  </div>
</section>

<style lang="scss">
  #sub-content {
    position: relative;
    z-index: 2;
    padding-bottom: 40px;

    @media screen and (min-width: 1020px) {
      padding-inline: 10vw;
    }
  }

  .constructing-notice {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin-top: 60px;
    font-family: var(--font-body);
    font-size: 14px;
    color: var(--color-text-muted);
  }

  .constructing-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--color-chartreuse-dim);
    animation: dotPulse 2s ease-in-out infinite;
  }

  @keyframes dotPulse {
    0%, 100% { opacity: 0.3; }
    50% { opacity: 0.8; }
  }
</style>
