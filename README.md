# Perry's Playground

Personal homepage for Perry / AMagicPear. Built with Svelte, Vite, and a Rust/WebAssembly particle field.

## Development

```sh
pnpm install
pnpm run dev
```

Run validation and a production build:

```sh
pnpm run check
pnpm run build
```

## Rebuilding The Particle Module

The interactive particle field lives in `src/lib/wasm-perryhome`. Rebuild it after Rust changes:

```sh
cd src/lib/wasm-perryhome
wasm-pack build --target web
```

## Credits

- [Svelte](https://svelte.dev/)
- [NanoFlow](https://github.com/ZTMYO/NanoFlow)
- [Alibaba Mama FangYuanTi VF](https://www.alibabafonts.com/)
- [WebAssembly](https://webassembly.org/)

## Posts

文章正文放在 `src/pages/*.md`，标题、日期、摘要等元信息写在 `src/data/posts.ts`。新增一篇：

1. 把 markdown 存到 `src/pages/`（Obsidian 的 frontmatter 会在渲染时自动去掉）；
2. 在 `posts` 数组最前面加一条 `slug / title / date / kicker / lead / body`；
3. 配图放到 `public/essay/<slug>/`，正文里用 `/essay/<slug>/xxx.webp` 这样的绝对路径引用。

首页「过程，也值得记录」和 `/essay` 列表都读这个数组，路由是 `/essay` 与 `/essay/<slug>`。

## Project showcase sources

Project descriptions were checked against the public repositories on 2026-11-03.
The showcase is curated in `src/App.svelte`; it does not fetch GitHub at runtime.

- `cover_netease_gpui.webp`: user-provided application screenshot, resized to WebP.
- `cover_pcl_proto.webp`: user-provided launcher screenshot, converted to WebP.
- The pichamber artwork is a workflow illustration, not an application screenshot. Its description follows the [public README](https://github.com/AMagicPear/pichamber).
- Education, internship, patents, skills, awards and current focus follow the résumé `沈立-简历1105.pdf`; a copy is served at `/resume.pdf` and linked from the experience section.
