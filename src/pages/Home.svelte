<script>
  import EchoNotes from "@/components/EchoNotes.svelte";
  import MyWorks from "@/components/MyWorks.svelte";
  import TopShowcase from "@/components/TopShowcase.svelte";
  import { _ } from "svelte-i18n";
</script>

<EchoNotes />
<TopShowcase />
<section id="sub-content">
  <MyWorks />
  <h2>{$_("menu.constructing")}……</h2>
</section>

<style>
  @media screen and (min-width: 1020px) {
    #sub-content {
      padding-inline: 10vw;
    }
  }
</style>
