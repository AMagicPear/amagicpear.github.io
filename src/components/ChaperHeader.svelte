<script lang="ts">
  export let title: string;
</script>

<div data-aos="fade-up" class="chapter-header">
  <div class="chapter-line"></div>
  <h2>{title}</h2>
  <div class="chapter-line chapter-line-right"></div>
</div>

<style lang="scss">
  .chapter-header {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;
    margin: 80px 0 40px;
    padding: 0 20px;

    .chapter-line {
      flex: 1;
      max-width: 60px;
      height: 1px;
      background: linear-gradient(
        to right,
        transparent,
        var(--color-chartreuse-dim)
      );

      &-right {
        background: linear-gradient(
          to left,
          transparent,
          var(--color-chartreuse-dim)
        );
      }
    }

    h2 {
      font-family: var(--font-display);
      font-size: 13px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.35em;
      color: var(--color-chartreuse);
      margin: 0;
      padding: 0;
      white-space: nowrap;
      opacity: 0.7;

      @media (prefers-color-scheme: light) {
        color: var(--color-chartreuse-dim);
      }
    }
  }
</style>
