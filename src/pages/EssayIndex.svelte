<script lang="ts">
  import { posts } from "@/data/posts";

  let {
    onHome,
    onOpen,
  }: {
    onHome: () => void;
    onOpen: (slug: string) => void;
  } = $props();

  function open(event: MouseEvent, slug: string) {
    if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    event.preventDefault();
    onOpen(slug);
  }
</script>

<article class="essay essay-index">
  <button class="back-link" onclick={onHome}>← 返回主页</button>
  <header class="essay-intro">
    <p class="essay-kicker">随笔 / NOTES</p>
    <h1>折腾过的事，<br />都记在这里。</h1>
    <p class="essay-lead">网络、小工具，还有那些半夜想不明白的问题。写下来，也顺便留给以后的自己。</p>
  </header>
  <div class="essay-rule"></div>

  <ol class="post-list">
    {#each posts as post, index}
      <li>
        <a class="post-item" href={`/essay/${post.slug}`} onclick={(event) => open(event, post.slug)}>
          <span class="post-index" aria-hidden="true">{String(index + 1).padStart(2, "0")}</span>
          <span class="post-copy">
            <span class="post-meta">{post.kicker}<time datetime={post.datetime}>{post.date}</time></span>
            <strong>{post.title}</strong>
            <span class="post-lead">{post.lead}</span>
          </span>
          <span class="post-arrow" aria-hidden="true">↗</span>
        </a>
      </li>
    {/each}
  </ol>
</article>
