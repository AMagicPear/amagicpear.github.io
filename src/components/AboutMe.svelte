<script>
  import ChaperHeader from "./ChaperHeader.svelte";
  import { _ } from "svelte-i18n";
</script>

<ChaperHeader title={$_("about.title")} />

<section class="about-section">
  <div class="about-grid">
    <div class="about-text">
      <p>{$_("about.bio_1")}</p>
      <p>{$_("about.bio_2")}</p>
      <p>{$_("about.bio_3")}</p>
    </div>

    <div class="about-highlight">
      <div class="highlight-label">{$_("about.highlight_label")}</div>
      <div class="highlight-text">{$_("about.highlight_text")}</div>
      <div class="highlight-corner"></div>
    </div>
  </div>
</section>

<style lang="scss">
  .about-section {
    position: relative;
    z-index: 2;
    max-width: 800px;
    margin: 20px auto 0;
    padding: 0 24px;
  }

  .about-grid {
    display: flex;
    gap: 48px;
    align-items: flex-start;

    @media (max-width: 700px) {
      flex-direction: column;
      gap: 32px;
    }
  }

  .about-text {
    flex: 1;
    text-align: left;

    p {
      font-family: var(--font-body);
      font-size: 16px;
      line-height: 1.9;
      color: var(--color-text-muted);
      margin-bottom: 1.2em;

      &:last-child {
        margin-bottom: 0;
      }
    }
  }

  .about-highlight {
    flex-shrink: 0;
    width: 260px;
    position: relative;
    padding: 28px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    text-align: left;

    @media (max-width: 700px) {
      width: 100%;
    }

    .highlight-label {
      font-family: var(--font-display);
      font-size: 10px;
      text-transform: uppercase;
      letter-spacing: 0.25em;
      color: var(--color-chartreuse);
      margin-bottom: 16px;
      opacity: 0.8;
    }

    .highlight-text {
      font-family: var(--font-body);
      font-size: 15px;
      line-height: 1.75;
      color: var(--color-text);
    }

    .highlight-corner {
      position: absolute;
      bottom: 0;
      right: 0;
      width: 24px;
      height: 24px;
      border-right: 1px solid var(--color-chartreuse);
      border-bottom: 1px solid var(--color-chartreuse);
      opacity: 0.4;
    }
  }
</style>
