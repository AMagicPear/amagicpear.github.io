<script lang="ts">
  import LostInLightRoundWhite from "@/assets/icons/LostInLightRoundWhite.svg";
  import earthIcon from "@/assets/icons/Earth.svg";
  import { _, locale } from "svelte-i18n";
  import { fade } from "svelte/transition";
  import { Language } from "@/i18n";
  import { currentPage } from "@/lib/stores";

  const duration = 100;
  let showTooltip = $state(false);
  let timeoutId: number;

  function handleMouseEnter() {
    showTooltip = true;
  }

  function handleMouseLeave() {
    timeoutId = setTimeout(() => {
      showTooltip = false;
    }, duration);
  }

  function handleTooltipMouseEnter() {
    clearTimeout(timeoutId);
  }

  function handleSwitchLanguage(lang: Language) {
    console.info(`Switching language to ${lang}`);
    locale.set(lang);
    localStorage.setItem("locale", lang);
  }
</script>

<div id="header-container">
  <div class="header-rule"></div>
  <div class="subcontainer">
    <div class="left">
      <a href="/" class="brand">
        <picture>
          <source srcset={LostInLightRoundWhite} type="image/svg+xml" />
          <img src={LostInLightRoundWhite} alt="" />
        </picture>
        <span class="brand-text">
          {#if $locale == Language.ZH_CN}一只会魔法的梨{/if}
          <span class="title-en">
            {#if $locale == Language.ZH_CN}&nbsp;|&nbsp;{/if}AMagicPear
          </span>
        </span>
      </a>
    </div>
    <div class="right">
      <nav>
        <a href="/" class="nav-link">
          <span class:active={$currentPage === "home"}>{$_("menu.showcase")}</span>
          <span class="nav-underline"></span>
        </a>
        <a href="/essay" class="nav-link">
          <span class:active={$currentPage === "essay"}>{$_("menu.essay")}</span>
          <span class="nav-underline"></span>
        </a>
        <span class="nav-dim">{$_("menu.constructing")}</span>
      </nav>
      <div class="earth-icon-container">
        <img
          src={earthIcon}
          alt="Earth Icon"
          onmouseenter={handleMouseEnter}
          onmouseleave={handleMouseLeave}
        />
        {#if showTooltip}
          <div
            class="translation-tip"
            onmouseenter={handleTooltipMouseEnter}
            onmouseleave={handleMouseLeave}
            role="group"
            transition:fade={{ duration }}
          >
            <h4>{$_("menu.language")}</h4>
            <menu>
              <menuitem onclick={() => handleSwitchLanguage(Language.ZH_CN)}
                >简体中文</menuitem
              >
              <menuitem onclick={() => handleSwitchLanguage(Language.EN)}
                >English</menuitem
              >
            </menu>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  .translation-tip {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 12px;
    background: #242420;
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: var(--color-paper);
    padding: 16px 20px;
    white-space: nowrap;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);

    h4 {
      margin: 0 0 12px 0;
      font-size: 12px;
      text-transform: uppercase;
      letter-spacing: 0.2em;
      color: #c8ff36;
      opacity: 0.7;
    }
    menu {
      list-style: none;
      padding: 0;
      margin: 0;
      menuitem {
        display: block;
        padding: 6px 0;
        font-size: 14px;
        cursor: pointer;
        transition: all 0.2s ease;
        color: var(--color-paper);
        opacity: 0.7;
        &:hover {
          opacity: 1;
          color: #c8ff36;
          padding-left: 4px;
        }
      }
    }
  }

  .earth-icon-container {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    cursor: pointer;
    img {
      width: 100%;
      height: 100%;
      user-select: none;
      opacity: 0.6;
      transition: opacity 0.3s ease;
    }
    &:hover img {
      opacity: 1;
    }
  }

  #header-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 64px;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 50;
    backdrop-filter: blur(12px);
    background-color: rgba(13, 13, 12, 0.75);
    padding-inline: 4vw;

    .header-rule {
      position: absolute;
      bottom: 0;
      left: 4vw;
      right: 4vw;
      height: 1px;
      background: linear-gradient(
        90deg,
        transparent,
        rgba(200, 255, 54, 0.15) 20%,
        rgba(200, 255, 54, 0.15) 80%,
        transparent
      );
    }

    @media screen and (min-width: 768px) {
      padding-inline: 14vw;
      .header-rule {
        left: 14vw;
        right: 14vw;
      }
    }

    .subcontainer {
      width: 100%;
      display: flex;
      justify-content: space-between;
      align-items: center;
      color: var(--color-paper);

      .left {
        .brand {
          display: flex;
          justify-content: center;
          align-items: center;
          gap: 10px;

          picture {
            width: 32px;
            height: 32px;
          }
        }

        .brand-text {
          font-size: 18px;
          font-weight: 700;
          pointer-events: none;
          user-select: none;
          text-align: left;
        }
      }

      .right {
        display: flex;
        align-items: center;
        gap: 24px;

        nav {
          display: flex;
          align-items: center;
          gap: 20px;
        }

        .nav-link {
          position: relative;
          span {
            font-size: 15px;
            transition: color 0.3s ease;
            color: rgba(245, 240, 232, 0.65);

            &.active {
              color: #c8ff36;
            }
          }

          .nav-underline {
            position: absolute;
            bottom: -6px;
            left: 0;
            width: 100%;
            height: 2px;
            background: #c8ff36;
            transform: scaleX(0);
            transform-origin: left;
            transition: transform 0.35s cubic-bezier(0.22, 0.61, 0.36, 1);
          }

          &:hover .nav-underline {
            transform: scaleX(1);
          }

          &:hover span {
            color: var(--color-paper);
          }
        }

        .nav-dim {
          font-size: 15px;
          color: rgba(245, 240, 232, 0.25);
          pointer-events: none;
        }
      }
    }
  }

  @media screen and (max-width: 538px) {
    .title-en {
      display: none;
    }
  }
</style>
