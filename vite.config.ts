import { copyFileSync, mkdirSync } from 'node:fs'
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    {
      name: 'github-pages-spa-fallback',
      apply: 'build',
      closeBundle() {
        copyFileSync('dist/index.html', 'dist/404.html')
        mkdirSync('dist/essay', { recursive: true })
        copyFileSync('dist/index.html', 'dist/essay/index.html')
      },
    },
  ],
  resolve: {
    alias: {
      '@': '/src',
    },
  },
})
