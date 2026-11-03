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

## Project showcase sources

Project descriptions were checked against the public repositories on 2026-11-03.
The showcase is curated in `src/App.svelte`; it does not fetch GitHub at runtime.

- `cover_netease_gpui.webp`: user-provided application screenshot, resized to WebP.
- `cover_pcl_proto.webp`: user-provided launcher screenshot, converted to WebP.
- The pichamber artwork is a workflow illustration, not an application screenshot. Its description follows the [public README](https://github.com/AMagicPear/pichamber).
