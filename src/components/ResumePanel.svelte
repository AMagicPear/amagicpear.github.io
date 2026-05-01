<script>
  import ChaperHeader from "./ChaperHeader.svelte";
  import { _ } from "svelte-i18n";

  const awards = [0, 1, 2, 3];
  const skills = [
    { key: "languages", icon: "&#x25C9;" },
    { key: "tools", icon: "&#x25A3;" },
    { key: "design", icon: "&#x25CB;" },
    { key: "english", icon: "&#x25C6;" },
    { key: "traits", icon: "&#x25B3;" },
  ];
</script>

<!-- Awards -->
<ChaperHeader title={$_("awards.title")} />

<section class="awards-section">
  <div class="awards-grid">
    {#each awards as i}
      <div class="award-card">
        <div class="award-medal">
          {#if i === 0}&#x1F3C5;{:else if i === 1}&#x1F948;{:else if i === 2}&#x1F949;{:else}&#x1F3C6;{/if}
        </div>
        <div class="award-content">
          <div class="award-name">{$_("awards.items." + i + ".name")}</div>
          <div class="award-meta">
            <span class="award-level">{$_("awards.items." + i + ".level")}</span>
            <span class="award-dot"></span>
            <span class="award-year">{$_("awards.items." + i + ".year")}</span>
          </div>
        </div>
      </div>
    {/each}
  </div>
</section>

<!-- Campus -->
<ChaperHeader title={$_("campus.title")} />

<section class="campus-section">
  <div class="campus-card">
    <div class="campus-date">{$_("campus.period")}</div>
    <h3>{$_("campus.role")}</h3>
    <p>{$_("campus.desc")}</p>
  </div>
</section>

<!-- Skills -->
<ChaperHeader title={$_("skills.title")} />

<section class="skills-section">
  <div class="skills-grid">
    {#each skills as skill}
      <div class="skill-card">
        <div class="skill-icon">{@html skill.icon}</div>
        <div class="skill-label">{$_("skills." + skill.key)}</div>
        <div class="skill-desc">{$_("skills." + skill.key + "_desc")}</div>
      </div>
    {/each}
  </div>
</section>

<style lang="scss">
  /* Awards */
  .awards-section {
    position: relative;
    z-index: 2;
    max-width: 800px;
    margin: 10px auto 0;
    padding: 0 24px;
  }

  .awards-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;

    @media (max-width: 600px) {
      grid-template-columns: 1fr;
    }
  }

  .award-card {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    padding: 20px 24px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    text-align: left;
    transition: all 0.3s ease;

    &:hover {
      border-color: var(--color-chartreuse-dim);
      translate: 0 -2px;
    }

    .award-medal {
      font-size: 24px;
      flex-shrink: 0;
      line-height: 1;
      margin-top: 2px;
    }

    .award-name {
      font-family: var(--font-display);
      font-size: 14px;
      font-weight: 600;
      color: var(--color-text);
      line-height: 1.5;
      margin-bottom: 6px;
    }

    .award-meta {
      display: flex;
      align-items: center;
      gap: 8px;
      font-family: var(--font-display);
      font-size: 10px;
      text-transform: uppercase;
      letter-spacing: 0.2em;
      color: var(--color-chartreuse);
      opacity: 0.7;

      .award-dot {
        width: 3px;
        height: 3px;
        border-radius: 50%;
        background: var(--color-chartreuse);
        opacity: 0.5;
      }
    }
  }

  /* Campus */
  .campus-section {
    position: relative;
    z-index: 2;
    max-width: 800px;
    margin: 10px auto 0;
    padding: 0 24px;
  }

  .campus-card {
    text-align: left;
    padding: 28px 32px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    position: relative;

    &::before {
      content: "";
      position: absolute;
      top: 0;
      left: 0;
      width: 3px;
      height: 100%;
      background: var(--color-chartreuse);
      opacity: 0.4;
    }

    .campus-date {
      font-family: var(--font-display);
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.2em;
      color: var(--color-chartreuse);
      opacity: 0.7;
      margin-bottom: 10px;
    }

    h3 {
      font-family: var(--font-display);
      font-size: 18px;
      font-weight: 700;
      color: var(--color-text);
      margin: 0 0 12px 0;
    }

    p {
      font-family: var(--font-body);
      font-size: 15px;
      line-height: 1.8;
      color: var(--color-text-muted);
      margin: 0;
    }
  }

  /* Skills */
  .skills-section {
    position: relative;
    z-index: 2;
    max-width: 880px;
    margin: 10px auto 0;
    padding: 0 24px;
  }

  .skills-grid {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 12px;

    @media (max-width: 900px) {
      grid-template-columns: repeat(2, 1fr);
    }

    @media (max-width: 500px) {
      grid-template-columns: 1fr;
    }
  }

  .skill-card {
    padding: 24px 16px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    text-align: center;
    transition: all 0.35s cubic-bezier(0.22, 0.61, 0.36, 1);

    &:hover {
      border-color: var(--color-chartreuse-dim);
      translate: 0 -3px;
    }

    .skill-icon {
      font-size: 20px;
      color: var(--color-chartreuse);
      margin-bottom: 12px;
      opacity: 0.7;
    }

    .skill-label {
      font-family: var(--font-display);
      font-size: 11px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.15em;
      color: var(--color-text);
      margin-bottom: 8px;
    }

    .skill-desc {
      font-family: var(--font-body);
      font-size: 13px;
      line-height: 1.6;
      color: var(--color-text-muted);
    }
  }
</style>
