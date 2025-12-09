<script lang="ts">
  import { onDestroy } from "svelte";
  import LeftCard from "@/components/LeftCard.svelte";
  import EmailIcon from "@/assets/icons/email.svg";
  import GithubIcon from "@/assets/icons/github.svg";
  import BilibiliIcon from "@/assets/icons/bilibili.svg";
  import NeteaseMusicIcon from "@/assets/icons/netease-music.svg";
  import Typed from "typed.js";
  import { _, locale } from "svelte-i18n";
  import type { Language } from "@/i18n";
  import TopDescription from "@/data/top_descriptions.json";

  let typed: Typed | undefined = undefined;
  let typedElement: HTMLSpanElement;
  const PerryWavesPromise = import("@/components/PerryWaves.svelte");

  const contact = [
    {
      name: "Email",
      icon: EmailIcon,
      link: "mailto:hello@amagicpear.top",
    },
    {
      name: "Github",
      icon: GithubIcon,
      link: "https://github.com/AMagicPear",
    },
    {
      name: "Bilibili",
      icon: BilibiliIcon,
      link: "https://space.bilibili.com/52833994",
    },
    {
      name: "Netease Music",
      icon: NeteaseMusicIcon,
      link: "https://music.163.com/#/artist?id=34318509",
    },
  ];

  $: typeStrings = TopDescription[$locale as Language] || TopDescription.en;

  $: if (typedElement && typeStrings) {
    typed?.destroy();
    typed = new Typed(typedElement, {
      strings: typeStrings,
      typeSpeed: 90,
      backSpeed: 60,
      backDelay: 1000,
      loop: true,
    });
  }

  onDestroy(() => {
    typed?.destroy();
  });
</script>

<section id="top-showcase">
  <div class="background-element">
    {#await PerryWavesPromise then PerryWaves}
      <PerryWaves.default />
    {/await}
  </div>
  <div class="floating-element">
    <LeftCard cardTitle={$_("top_showcase.hello")}>
      <p>
        {$_("top_showcase.i_am")}&nbsp;<span bind:this={typedElement}></span>
      </p>
    </LeftCard>
    <LeftCard cardTitle={$_("top_showcase.platforms")}>
      <div class="contact-list">
        {#each contact as item}
          <a href={item.link} target="_blank">
            <img src={item.icon} alt={item.name} />
          </a>
        {/each}
      </div>
    </LeftCard>
  </div>
</section>

<style lang="scss">
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
    overflow-anchor: none;
  }

  .floating-element {
    position: absolute;
    top: 50%;
    left: 30%;
    transform: translate(-50%, -50%);
    z-index: 3;
    pointer-events: none;
    transition: scale 0.2s ease-in-out;
    > :global(*) {
      pointer-events: all;
    }
    @media screen and (max-width: 768px) {
      scale: 0.8;
    }

    .contact-list {
      display: flex;
      flex-direction: row;
      justify-content: center;
      align-items: flex-start;
      gap: 12px;
      img {
        max-width: 30px;
        max-height: 30px;
        margin-right: 10px;
        transition: transform 0.2s ease-in-out;

        &:hover {
          transform: scale(1.1);
        }
      }
    }

    > :global(.left-card:nth-child(1)) {
      transform: translate(0, -100px) rotate(6deg);
    }
    > :global(.left-card:nth-child(2)) {
      transform: translate(0, 0);
    }
  }
</style>
