import { copyFileSync, mkdirSync, readFileSync } from 'node:fs'
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

/** 从文章元信息里读出 slug，给每篇文章生成一个静态入口，避免直接访问时落到 404 页。 */
function postSlugs() {
  const source = readFileSync('src/data/posts.ts', 'utf8')
  return [...source.matchAll(/slug:\s*"([^"]+)"/g)].map((match) => match[1])
}

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
        for (const slug of postSlugs()) {
          mkdirSync(`dist/essay/${slug}`, { recursive: true })
          copyFileSync('dist/index.html', `dist/essay/${slug}/index.html`)
        }
      },
    },
  ],
  resolve: {
    alias: {
      '@': '/src',
    },
  },
})
