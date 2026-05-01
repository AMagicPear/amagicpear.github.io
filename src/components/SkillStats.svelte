<script>
  import ChaperHeader from "./ChaperHeader.svelte";
  import { _ } from "svelte-i18n";

  const skills = [
    { key: "frontend", icon: "&#x25C9;" },
    { key: "backend", icon: "&#x25A3;" },
    { key: "design", icon: "&#x25CB;" },
    { key: "hardware", icon: "&#x25A7;" },
  ];
</script>

<ChaperHeader title={$_("stats.title")} />

<section class="stats-section">
  <div class="stats-grid">
    {#each skills as skill}
      <div class="stat-card">
        <div class="stat-icon">{@html skill.icon}</div>
        <div class="stat-label">{$_(`stats.${skill.key}`)}</div>
        <div class="stat-desc">{$_(`stats.${skill.key}_desc`)}</div>
      </div>
    {/each}
  </div>
</section>

<style lang="scss">
  .stats-section {
    position: relative;
    z-index: 2;
    max-width: 880px;
    margin: 10px auto 0;
    padding: 0 24px;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;

    @media (max-width: 768px) {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  .stat-card {
    padding: 28px 20px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    text-align: center;
    transition: all 0.35s cubic-bezier(0.22, 0.61, 0.36, 1);

    &:hover {
      border-color: var(--color-chartreuse-dim);
      translate: 0 -3px;
    }

    .stat-icon {
      font-size: 22px;
      color: var(--color-chartreuse);
      margin-bottom: 14px;
      opacity: 0.7;
    }

    .stat-label {
      font-family: var(--font-display);
      font-size: 13px;
      font-weight: 700;
      color: var(--color-text);
      margin-bottom: 8px;
      text-transform: uppercase;
      letter-spacing: 0.1em;
    }

    .stat-desc {
      font-family: var(--font-body);
      font-size: 13px;
      line-height: 1.6;
      color: var(--color-text-muted);
    }
  }
</style>
