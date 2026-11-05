import { marked } from "marked";

marked.setOptions({ gfm: true, breaks: false });

/** 去掉 Obsidian 风格的前置元数据。 */
function stripFrontmatter(source: string) {
  return source.replace(/^---\r?\n[\s\S]*?\r?\n---\r?\n/, "");
}

/** 正文图片统一加上懒加载，避免长文一进页面就拉全部图。 */
function lazyImages(html: string) {
  return html.replace(/<img\b([^>]*)>/g, (tag, attrs: string) => {
    if (/loading=/.test(attrs)) return tag;
    return `<img loading="lazy" decoding="async"${attrs}>`;
  });
}

export function renderMarkdown(source: string) {
  return lazyImages(marked.parse(stripFrontmatter(source)) as string);
}
