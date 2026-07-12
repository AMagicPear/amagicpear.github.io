# Perry's Playground

## Overview

Personal homepage built with Svelte 5, Vite, TypeScript, SCSS, and a Rust/WebAssembly particle field. The public site is a single-page portfolio with an internal `/essay` view.

## Commands

```sh
pnpm install
pnpm run dev
pnpm run check
pnpm run build
pnpm run preview
pnpm run deploy
```

## Structure

- `src/App.svelte`: portfolio layout, lightweight client-side path handling, and content.
- `src/pages/Essay.svelte`: article view rendered from `src/pages/2025-07-15.md`.
- `src/components/PerryWaves.svelte`: pointer-reactive SVG particle surface.
- `src/data/simplified_nanoflow.json`: particle layout and physics configuration.
- `src/lib/wasm-perryhome/`: Rust source for the particle engine.

## WebAssembly

Rebuild the generated browser package after changing Rust code:

```sh
cd src/lib/wasm-perryhome
wasm-pack build --target web
```

The generated `pkg/` output is consumed directly by `PerryWaves.svelte` and must remain in the repository.
