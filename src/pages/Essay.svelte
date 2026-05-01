<script lang="ts">
  import { marked } from "marked";
  import { onMount } from "svelte";

  let docHTML: string;

  onMount(async () => {
    const response = await import("./2025-07-15.md?raw");
    const doc = response.default;
    docHTML = await marked.parse(doc);
  });
</script>

<article>
  <div class="essay-sidebar">
    <div class="sidebar-line"></div>
    <span class="sidebar-label">ESSAY</span>
  </div>

  <div class="essay-content">
    <div class="essay-notice">
      <span>此处为测试占位文章，后续将从服务器获取列表</span>
    </div>
    <h1>有了树莓派以后，我是如何改造家里的网络的？</h1>
    <div class="essay-divider"></div>
    {@html docHTML}
  </div>
</article>

<style>
  article {
    display: flex;
    gap: 40px;
    max-width: 900px;
    margin: 120px auto 80px;
    padding: 0 24px;
    text-align: justify;
    line-height: 1.85;
    color: var(--color-text);
  }

  .essay-sidebar {
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding-top: 8px;

    @media (max-width: 700px) {
      display: none;
    }

    .sidebar-line {
      width: 1px;
      height: 60px;
      background: linear-gradient(
        to bottom,
        var(--color-chartreuse),
        transparent
      );
      opacity: 0.5;
    }

    .sidebar-label {
      writing-mode: vertical-rl;
      font-family: var(--font-display);
      font-size: 10px;
      letter-spacing: 0.3em;
      color: var(--color-chartreuse);
      opacity: 0.5;
    }
  }

  .essay-content {
    flex: 1;
    min-width: 0;
  }

  .essay-notice {
    font-family: var(--font-body);
    font-size: 13px;
    color: var(--color-gold);
    border: 1px solid var(--color-gold);
    border-color: rgba(184, 152, 46, 0.25);
    padding: 10px 16px;
    margin-bottom: 32px;
    display: inline-block;
    opacity: 0.8;
  }

  .essay-divider {
    width: 100%;
    height: 1px;
    background: linear-gradient(
      90deg,
      var(--color-chartreuse),
      transparent
    );
    opacity: 0.2;
    margin: 24px 0 40px;
  }

  h1 {
    font-family: var(--font-display);
    text-align: left;
    font-size: 2.2em;
    line-height: 1.3;
    color: var(--color-text);
    margin: 0;
    font-weight: 700;
    letter-spacing: -0.01em;
  }

  :global(pre) {
    text-align: left;
    overflow-x: auto;
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    padding: 20px;
    border-radius: 0;
    margin: 24px 0;
    font-size: 0.9em;
    color: var(--color-text);
  }

  :global(code) {
    background-color: var(--color-surface);
    padding: 2px 8px;
    border-radius: 0;
    font-family: 'SF Mono', 'Fira Code', monospace;
    font-size: 0.88em;
    color: var(--color-chartreuse);
  }

  :global(pre code) {
    background-color: transparent;
    padding: 0;
    border-radius: 0;
    color: var(--color-text);
  }

  article :global(img) {
    max-width: 100%;
    height: auto;
    margin: 28px 0;
    border: 1px solid var(--color-border);
  }

  article :global(p) {
    margin: 0 0 1.2em 0;
    font-family: var(--font-body);
    font-size: 16px;
    color: var(--color-text-muted);
    line-height: 1.9;
  }

  article :global(h2) {
    font-family: var(--font-display);
    font-size: 1.4em;
    margin: 2em 0 0.6em;
    color: var(--color-text);
    text-align: left;
  }

  article :global(h3) {
    font-family: var(--font-display);
    font-size: 1.15em;
    margin: 1.6em 0 0.5em;
    color: var(--color-text);
    text-align: left;
  }

  article :global(blockquote) {
    border-left: 2px solid var(--color-chartreuse);
    margin: 24px 0;
    padding: 4px 20px;
    color: var(--color-text-muted);
    font-style: italic;
  }

  article :global(ul), article :global(ol) {
    padding-left: 24px;
    color: var(--color-text-muted);
  }

  article :global(li) {
    margin-bottom: 6px;
  }
</style>
