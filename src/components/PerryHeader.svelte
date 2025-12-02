<script lang="ts">
  import LostInLightRoundWhite from "@/assets/icons/LostInLightRoundWhite.svg";
</script>

<div id="header-container">
  <div class="subcontainer">
    <div class="left">
      <img src={LostInLightRoundWhite} alt="" />
      <span>一只会魔法的梨<span class="title-en">&nbsp;|&nbsp;AMagicPear</span></span>
    </div>
    <div class="right">
      <a href="/"><span class="active">展示页</span></a>
      <span style="color: rgba(255, 255, 255, 0.5);">更多页面建设中</span>
    </div>
  </div>
</div>

<style lang="scss">
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
    background-color: rgba(0, 0, 0, 0.1);
    .subcontainer {
      width: 92%;
      max-width: 1080px;
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
        img {
          width: 32px;
          height: 32px;
        }
        span {
          font-size: 19px;
        }
        pointer-events: none;
        user-select: none;
      }

      .right {
        span {
          font-size: 16px;
          position: relative;
          &::after {
            content: '';
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
