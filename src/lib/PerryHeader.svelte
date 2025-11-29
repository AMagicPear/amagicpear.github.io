<script lang="ts">
  import { animate, hover } from "motion";
  import { onMount } from "svelte";

  let subcontainer: HTMLElement;

  onMount(() => {
    hover(subcontainer, (element) => {
      animate(
        element,
        {
          scale: 1.08,
          transform: "translateY(10px) scale(1.1)",
        },
        { type: "spring", bounce: 0.4, duration: 0.6 }
      );
      return () =>
        animate(
          element,
          {
            scale: 1,
            transform: "translateY(0px) scale(1)",
          },
          { type: "spring", bounce: 0.7, duration: 1 }
        );
    });
  });
</script>

<div id="header-container">
  <div id="subcontainer" bind:this={subcontainer}>
    <h1>AMagicPear&nbsp;&nbsp;|&nbsp;&nbsp;<span class="cn">一只会魔法的梨</span></h1>
  </div>
</div>

<style lang="scss">
  #header-container {
    position: fixed;
    top: 30px;
    left: 0;
    width: 100%;
    height: 60px;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: space-between;
    padding-inline: 20px;
    z-index: 50;

    #subcontainer {
      width: 55%;
      height: 100%;
      min-width: 500px;
      backdrop-filter: blur(2px);
      background-color: rgba(0, 0, 0, 0.1);
      display: flex;
      flex-direction: row;
      justify-content: center;
      align-items: center;
      transform: translateY(0px) scale(1);
      will-change: transform;
      cursor: none;

      h1 {
        font-size: 28px;
        user-select: none;
        text-shadow: 0 0 10px rgba(255, 162, 13, 0.5);
        color: white;
        .cn {
          letter-spacing: 2px;
        }
      }
    }
  }
</style>
