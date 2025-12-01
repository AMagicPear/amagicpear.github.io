<script lang="ts">
  export let cardTitle: string;
</script>

<div class="left-card">
  <div class="left-card-offset"></div>
  <h2>{cardTitle}</h2>
  <slot></slot>
</div>

<style lang="scss">
  .left-card-offset {
    position: absolute;
    top: -16px;
    left: 16px;
    width: 100%;
    height: 100%;
    border: 1px solid rgb(160, 255, 36);
    pointer-events: none;
    transition: border-color 0.3s ease-in-out;
  }

  .left-card {
    width: calc(240px + 6vw + 2vh);
    background-color: rgb(160, 255, 36);
    color: black;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    padding-left: 20px;
    padding-bottom: 20px;
    transition:
      background-color 0.3s ease-in-out,
      translate 0.3s ease-in-out;

    &:hover {
      background-color: rgb(184, 255, 91);
      translate: 0px -2px;
      .left-card-offset {
        border-color: rgb(184, 255, 91);
      }
    }
  }
</style>
