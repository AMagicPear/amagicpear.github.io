<script lang="ts">
  import type { Post } from "@/data/posts";
  import { renderMarkdown } from "@/lib/markdown";

  let {
    post,
    others = [],
    onHome,
    onIndex,
    onOpen,
  }: {
    post: Post;
    others?: Post[];
    onHome: () => void;
    onIndex: () => void;
    onOpen: (slug: string) => void;
  } = $props();

  const html = $derived(renderMarkdown(post.body));

  function open(event: MouseEvent, slug: string) {
    if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    event.preventDefault();
    onOpen(slug);
  }
</script>

<article class="essay">
  <div class="essay-nav">
    <button class="back-link" onclick={onHome}>← 返回主页</button>
    <button class="back-link" onclick={onIndex}>全部随笔</button>
  </div>
  <header class="essay-intro">
    <p class="essay-kicker">{post.kicker}</p>
    <p class="essay-date"><time datetime={post.datetime}>{post.date}</time></p>
    <h1>{post.title}</h1>
    <p class="essay-lead">{post.lead}</p>
  </header>
  <div class="essay-rule"></div>
  <div class="essay-body">{@html html}</div>

  {#if post.source || others.length}
    <footer class="essay-footer">
      {#if post.source}
        <p class="essay-source">
          原载于 <a href={post.source.href} target="_blank" rel="noreferrer">{post.source.label} <span aria-hidden="true">↗</span></a>
        </p>
      {/if}
      {#if others.length}
        <div class="essay-more">
          <p class="essay-footer-label">接着读</p>
          <ul>
            {#each others as item}
              <li>
                <a href={`/essay/${item.slug}`} onclick={(event) => open(event, item.slug)}>
                  <span class="essay-more-title">{item.title}</span>
                  <span class="essay-more-meta">{item.kicker} · {item.date}</span>
                </a>
              </li>
            {/each}
          </ul>
        </div>
      {/if}
    </footer>
  {/if}
</article>
