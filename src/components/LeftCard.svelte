<script lang="ts">
  export let cardTitle: string;
</script>

<div class="left-card">
  <div class="card-rule"></div>
  <h2>{cardTitle}</h2>
  <div class="card-slot">
    <slot></slot>
  </div>
  <div class="card-corner"></div>
</div>

<style lang="scss">
  /* LeftCard is always rendered on the dark TopShowcase hero background.
     Colors are hardcoded for dark theme — independent of system mode. */

  $accent: #c8ff36;

  .left-card {
    position: relative;
    width: calc(260px + 6vw + 2vh);
    background: rgba(13, 13, 12, 0.85);
    backdrop-filter: blur(12px);
    border: 1px solid rgba($accent, 0.25);
    color: #f5f0e8;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    padding: 28px 24px 28px 28px;
    clip-path: polygon(0 0, 100% 0, 100% calc(100% - 12px), calc(100% - 12px) 100%, 0 100%);
    transition: all 0.4s cubic-bezier(0.22, 0.61, 0.36, 1);

    .card-rule {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 3px;
      background: linear-gradient(90deg, $accent, transparent 70%);
      transform-origin: left;
      transition: transform 0.4s cubic-bezier(0.22, 0.61, 0.36, 1);
    }

    .card-corner {
      position: absolute;
      bottom: 0;
      right: 0;
      width: 20px;
      height: 20px;
      border-left: 1px solid rgba($accent, 0.4);
      border-top: 1px solid rgba($accent, 0.4);
      transform: translate(4px, 4px) rotate(45deg);
      transition: all 0.4s cubic-bezier(0.22, 0.61, 0.36, 1);
    }

    h2 {
      font-family: var(--font-display);
      font-size: 14px;
      text-transform: uppercase;
      letter-spacing: 0.3em;
      color: $accent;
      margin: 0 0 16px 0;
      opacity: 0.8;
    }

    .card-slot {
      width: 100%;
    }

    &:hover {
      background: rgba(13, 13, 12, 0.95);
      border-color: rgba($accent, 0.5);
      translate: 0 -4px;

      .card-rule {
        transform: scaleX(1.3);
      }

      .card-corner {
        border-color: rgba($accent, 0.7);
        transform: translate(2px, 2px) rotate(45deg);
      }
    }
  }
</style>
