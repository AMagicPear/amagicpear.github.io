<script lang="ts">
  import LostInLightRoundWhite from "@/assets/icons/LostInLightRoundWhite.svg";
  import earthIcon from "@/assets/icons/Earth.svg";
  import { _, locale } from "svelte-i18n";
  import { fade } from "svelte/transition";
  import { Language } from "@/i18n";

  const duration = 100;
  let showTooltip = $state(false);
  let timeoutId: number;

  function handleMouseEnter() {
    showTooltip = true;
  }

  function handleMouseLeave() {
    timeoutId = setTimeout(() => {
      showTooltip = false;
    }, duration); // 延迟关闭
  }

  function handleTooltipMouseEnter() {
    clearTimeout(timeoutId);
  }

  function handleSwitchLanguage(lang: Language) {
    console.info(`Switching language to ${lang}`);
    $locale = lang;
    localStorage.setItem("locale", lang);
  }
</script>

<div id="header-container">
  <div class="subcontainer">
    <div class="left">
      <picture>
        <source srcset={LostInLightRoundWhite} type="image/svg+xml" />
        <img src={LostInLightRoundWhite} alt="" />
      </picture>
      <span
        >一只会魔法的梨<span class="title-en">&nbsp;|&nbsp;AMagicPear</span
        ></span
      >
    </div>
    <div class="right">
      <a href="/"><span class="active">{$_("pages.showcase")}</span></a>
      <span style:color="rgba(255, 255, 255, 0.5)"
        >{$_("pages.constructing")}</span
      >
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
            <h4>切换语言</h4>
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
    margin-top: 8px;
    background: #333;
    color: white;
    padding: 16px;
    border-radius: 4px;
    white-space: nowrap;
    h4 {
      margin: 0 0 10px 0;
    }
    menu {
      list-style: none;
      padding: 0;
      margin: 0;
      menuitem {
        display: block;
        padding: 4px 0;
        font-size: 14px;
        cursor: pointer;
        transition: font-weight 0.3s ease;
        &:hover {
          font-weight: 800;
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
    img {
      width: 100%;
      height: 100%;
      user-select: none;
    }
  }

  #header-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 60px;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 50;
    backdrop-filter: blur(4px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    background-color: rgba(0, 0, 0, 0.25);
    padding-inline: 4vw;
    @media screen and (min-width: 768px) {
      padding-inline: 14vw;
    }
    .subcontainer {
      width: 100%;
      display: flex;
      justify-content: space-between;
      align-items: center;
      color: #fff;

      .left,
      .right {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: calc(1.2vw + 5px);
      }

      .left {
        picture {
          width: 32px;
          height: 32px;
        }
        font-size: 19px;
        font-weight: 700;
        pointer-events: none;
        user-select: none;
      }

      .right {
        position: relative;
        span {
          font-size: 16px;
          position: relative;
          &::after {
            content: "";
            position: absolute;
            bottom: -2px;
            left: 50%;
            width: 100%;
            height: 2px;
            background-color: #fff;
            transform: translateX(-50%) scaleX(0);
            transform-origin: center;
            transition: transform 0.3s ease;
          }
          &.active {
            font-weight: 600;
            &::after {
              transform: translateX(-50%) scaleX(1);
            }
            &:hover::after {
              animation: expand 0.3s ease;
            }
          }
        }

        @keyframes expand {
          from {
            transform: translateX(-50%) scaleX(0);
          }
          to {
            transform: translateX(-50%) scaleX(1);
          }
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
