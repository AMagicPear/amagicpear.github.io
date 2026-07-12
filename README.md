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
