<script>
  import { marked } from "marked";
  import doc from "./2025-07-15.md?raw";
  $: docHTML = marked.parse(doc);
</script>

<article>
  <pre>此处为测试占位文章，后续将从服务器获取列表</pre>
  <h1>有了树莓派以后，我是如何改造家里的网络的？</h1>
  {@html docHTML}
</article>

<style>
  article {
    text-align: justify;
    max-width: 800px;
    margin: 100px auto;
    padding: 0 20px;
    line-height: 1.6;
  }

  :global(pre) {
    text-align: left;
    overflow-x: auto;
    background-color: #f5f5f5;
    padding: 16px;
    border-radius: 6px;
    margin: 20px 0;
    font-size: 0.9em;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    color: #333;
  }

  :global(code) {
    background-color: #f0f0f0;
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.9em;
    color: #333;
  }

  :global(pre code) {
    background-color: transparent;
    padding: 0;
    border-radius: 0;
  }

  h1 {
    text-align: center;
    margin-bottom: 30px;
    font-size: 2em;
    color: #333;
  }

  article :global(img) {
    max-width: 100%;
    height: auto;
    margin: 20px 0;
    border-radius: 6px;
  }

  @media (prefers-color-scheme: dark) {
    :global(pre) {
      background-color: #2d2d2d;
      color: #e0e0e0;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    }

    :global(code) {
      background-color: #404040;
      color: #e0e0e0;
    }

    :global(pre code) {
      background-color: transparent;
    }

    h1 {
      color: #e0e0e0;
    }

    article {
      color: #e0e0e0;
    }
  }
</style>
