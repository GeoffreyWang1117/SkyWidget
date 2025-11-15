import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

export default {
  // 使用 Vite 预处理器
  preprocess: vitePreprocess(),

  // Svelte 编译选项
  compilerOptions: {
    // 使用 Svelte 5 的 runes 模式
    runes: true
  }
}
