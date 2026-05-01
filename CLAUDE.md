# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Personal homepage (Perry Home) - a Svelte 5 + Vite + TypeScript SPA with WebAssembly-powered particle effects.

## Development Commands

```sh
# Install dependencies (requires pnpm)
pnpm install

# Start dev server
pnpm run dev

# Build for production
pnpm run build

# Preview production build
pnpm run preview

# Type check
pnpm run check

# Deploy to GitHub Pages
pnpm run deploy
```

### WebAssembly Module

The particle flow visualization uses a Rust/WASM module located at `src/lib/wasm-perryhome/`. After pulling changes that include Rust code modifications:

```sh
cd src/lib/wasm-perryhome
wasm-pack build --target web
cd ../..
pnpm run dev
```

The compiled WASM outputs to `src/lib/wasm-perryhome/pkg/` and is consumed by `PerryWaves.svelte`.

## Architecture

### Routing
Client-side routing via `page` library. Routes are defined in [App.svelte](src/App.svelte):
- `/` → Home (TopShowcase, EchoNotes, MyWorks)
- `/essay` → Essay page (markdown rendering via `marked`)

### State Management
Svelte stores in [stores.js](src/lib/stores.js):
- `isAtTopShowcase` - controls TopShowcase visibility
- `currentPage` - drives page routing

### i18n
[svelte-i18n](https://github.com/kaisermann/svelte-i18n) with locale files in [src/locales/](src/locales/). Supported locales: `en`, `zh-CN`.

### Key Components
- [PerryWaves.svelte](src/components/PerryWaves.svelte) - WebAssembly-powered particle animation
- [TopShowcase.svelte](src/components/TopShowcase.svelte) - Hero section with typewriter effect
- [EchoNotes.svelte](src/components/EchoNotes.svelte) - Notes/intro section
- [PerryHeader.svelte](src/components/PerryHeader.svelte) - Navigation header

### Data
- [works.ts](src/data/works.ts) - My Works content
- [simplified_nanoflow.json](src/data/simplified_nanoflow.json) - Particle flow configuration

## Tech Stack

- **Framework**: Svelte 5 (runes mode with `$derived`, `$state`)
- **Build**: Vite with rolldown-vite
- **Styling**: SCSS
- **Routing**: page.js
- **i18n**: svelte-i18n
- **WASM**: Rust + wasm-bindgen + wasm-pack
- **Animations**: AOS (Animate On Scroll)
- **Markdown**: marked