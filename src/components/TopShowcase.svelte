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

  <div class="hero-grid">
    <div class="hero-accent-line"></div>

    <div class="floating-element">
      <div class="card-wrapper">
        <LeftCard cardTitle={$_("top_showcase.hello")}>
          <p class="hero-intro">
            {$_("top_showcase.i_am")}&nbsp;<span bind:this={typedElement} class="typed-target"></span>
          </p>
        </LeftCard>
      </div>

      <div class="card-wrapper card-offset">
        <LeftCard cardTitle={$_("top_showcase.platforms")}>
          <div class="contact-list">
            {#each contact as item}
              <a href={item.link} target="_blank" class="contact-item" title={item.name}>
                <img src={item.icon} alt={item.name} />
              </a>
            {/each}
          </div>
        </LeftCard>
      </div>
    </div>

    <div class="hero-vertical-text">
      <span>P</span>
      <span>E</span>
      <span>R</span>
      <span>R</span>
      <span>Y</span>
      <span class="dot"></span>
      <span>H</span>
      <span>O</span>
      <span>M</span>
      <span>E</span>
    </div>
  </div>

  <div class="scroll-indicator">
    <div class="scroll-line"></div>
  </div>
</section>

<style lang="scss">
  #top-showcase {
    position: relative;
    width: 100%;
    height: 100vh;
    background: linear-gradient(170deg, #141412 0%, #0d0d0c 40%, #11100e 100%);
    overflow: hidden;
  }

  .background-element {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .hero-grid {
    position: relative;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }

  .hero-accent-line {
    position: absolute;
    top: 50%;
    right: 8vw;
    width: 1px;
    height: 200px;
    background: linear-gradient(
      to bottom,
      transparent,
      rgba(200, 255, 54, 0.3) 30%,
      rgba(200, 255, 54, 0.3) 70%,
      transparent
    );
    transform: translateY(-50%);
    opacity: 0.6;
  }

  .hero-vertical-text {
    position: absolute;
    right: 4vw;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    font-family: var(--font-display);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.15em;
    color: rgba(245, 240, 232, 0.2);
    pointer-events: none;
    user-select: none;

    .dot {
      width: 3px;
      height: 3px;
      background: #c8ff36;
      border-radius: 50%;
      margin: 2px 0;
      opacity: 0.5;
    }

    @media (max-width: 768px) {
      display: none;
    }
  }

  .floating-element {
    position: absolute;
    top: 50%;
    left: 12vw;
    transform: translateY(-50%);
    z-index: 3;
    pointer-events: none;

    @media screen and (min-width: 1400px) {
      left: 18vw;
    }

    @media screen and (max-width: 768px) {
      left: 50%;
      transform: translate(-50%, -50%);
      scale: 0.85;
    }

    > * {
      pointer-events: all;
    }

    .card-wrapper {
      &:nth-child(1) {
        transform: translate(0, -120px) rotate(3deg);
      }

      &:nth-child(2) {
        transform: translate(30px, 20px) rotate(-1.5deg);
      }

      @media screen and (max-width: 768px) {
        &:nth-child(1) {
          transform: translate(0, -80px) rotate(3deg);
        }
        &:nth-child(2) {
          transform: translate(10px, 10px) rotate(-1.5deg);
        }
      }
    }
  }

  .hero-intro {
    font-family: var(--font-body);
    font-size: 18px;
    line-height: 1.6;
    color: var(--color-paper);
    margin: 0;

    .typed-target {
      color: #c8ff36;
      font-weight: 600;
    }
  }

  .contact-list {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    gap: 14px;

    .contact-item {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 36px;
      height: 36px;
      border-radius: 50%;
      border: 1px solid rgba(245, 240, 232, 0.15);
      transition: all 0.3s cubic-bezier(0.22, 0.61, 0.36, 1);

      img {
        width: 18px;
        height: 18px;
        opacity: 0.7;
        filter: brightness(0) invert(1);
        transition: all 0.3s cubic-bezier(0.22, 0.61, 0.36, 1);
      }

      &:hover {
        border-color: #c8ff36;
        background: rgba(200, 255, 54, 0.08);

        img {
          opacity: 1;
          transform: scale(1.1);
        }
      }
    }
  }

  .scroll-indicator {
    position: absolute;
    bottom: 40px;
    left: 50%;
    transform: translateX(-50%);

    .scroll-line {
      width: 1px;
      height: 48px;
      background: linear-gradient(
        to bottom,
        rgba(200, 255, 54, 0.5),
        transparent
      );
      animation: scrollPulse 2s ease-in-out infinite;
    }
  }

  @keyframes scrollPulse {
    0%, 100% {
      opacity: 0.3;
      height: 48px;
    }
    50% {
      opacity: 0.8;
      height: 60px;
    }
  }
</style>
