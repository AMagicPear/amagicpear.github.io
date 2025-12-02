<script lang="ts">
  import LostInLightRoundWhite from "../assets/icons/LostInLightRoundWhite.svg";
</script>

<div id="header-container">
  <div class="subcontainer">
    <div class="left">
      <img src={LostInLightRoundWhite} alt="" />
      <span>一只会魔法的梨<span class="title-en">&nbsp;|&nbsp;AMagicPear</span></span>
    </div>
    <div class="right">
      <span class="active">展示页</span>
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
    backdrop-filter: blur(2px);
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
          &.active {
            border-bottom: 2px solid #fff;
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
