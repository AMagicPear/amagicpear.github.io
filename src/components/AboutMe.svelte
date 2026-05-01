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

<ChaperHeader title={$_("education.title")} />

<section class="edu-section">
  <div class="edu-timeline">
    <div class="edu-item">
      <div class="edu-marker"></div>
      <div class="edu-card">
        <div class="edu-date">{$_("education.njupt.period")}</div>
        <h3>{$_("education.njupt.school")}</h3>
        <div class="edu-degree">{$_("education.njupt.degree")}</div>
        <div class="edu-dept">{$_("education.njupt.school_short")}</div>
      </div>
    </div>

    <div class="edu-item">
      <div class="edu-marker future"></div>
      <div class="edu-card">
        <div class="edu-date">{$_("education.suda.period")}</div>
        <h3>{$_("education.suda.school")}</h3>
        <div class="edu-degree">{$_("education.suda.degree")}</div>
        <div class="edu-dept">{$_("education.suda.school_short")}</div>
        <div class="edu-note">{$_("education.suda.note")}</div>
      </div>
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

  /* Education */
  .edu-section {
    position: relative;
    z-index: 2;
    max-width: 800px;
    margin: 10px auto 0;
    padding: 0 24px;
  }

  .edu-timeline {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .edu-item {
    display: flex;
    align-items: flex-start;
    gap: 20px;
    position: relative;
  }

  .edu-marker {
    flex-shrink: 0;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--color-chartreuse);
    margin-top: 28px;
    box-shadow: 0 0 8px var(--color-chartreuse-glow);

    &.future {
      background: transparent;
      border: 2px solid var(--color-chartreuse);
      box-shadow: none;
    }
  }

  .edu-card {
    flex: 1;
    text-align: left;
    padding: 24px 28px;
    margin-bottom: 4px;
    border-left: 1px solid var(--color-border);
    transition: border-color 0.3s ease;

    &:hover {
      border-left-color: var(--color-chartreuse);
    }

    .edu-date {
      font-family: var(--font-display);
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.2em;
      color: var(--color-chartreuse);
      opacity: 0.7;
      margin-bottom: 8px;
    }

    h3 {
      font-family: var(--font-display);
      font-size: 18px;
      font-weight: 700;
      color: var(--color-text);
      margin: 0 0 6px 0;
    }

    .edu-degree {
      font-family: var(--font-body);
      font-size: 15px;
      color: var(--color-text-muted);
      margin-bottom: 4px;
    }

    .edu-dept {
      font-family: var(--font-body);
      font-size: 13px;
      color: var(--color-text-muted);
      opacity: 0.7;
    }

    .edu-note {
      font-family: var(--font-display);
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.15em;
      color: var(--color-gold);
      margin-top: 10px;
      padding: 4px 12px;
      border: 1px solid rgba(184, 152, 46, 0.25);
      display: inline-block;
    }
  }
</style>
