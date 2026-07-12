<script lang="ts">
  import { marked } from "marked";
  import articleSource from "./2025-07-15.md?raw";

  let { onBack }: { onBack: () => void } = $props();
  const articleHtml = marked.parse(articleSource) as string;
</script>

<article class="essay">
  <header class="essay-intro">
    <button class="back-link" onclick={onBack}>← 返回主页</button>
    <p class="essay-kicker">FIELD NOTES / HOMELAB</p>
    <p class="essay-date">15 JULY 2025</p>
    <h1>有了树莓派以后，<br />我是如何改造家里的网络的？</h1>
    <p class="essay-lead">从 mihomo 的 TUN 模式，到路由器、交换机和树莓派各自该待在哪里。这是一篇关于把全屋网络重新理顺的折腾记录。</p>
  </header>
  <div class="essay-rule"></div>
  <div class="essay-body">{@html articleHtml}</div>
</article>
