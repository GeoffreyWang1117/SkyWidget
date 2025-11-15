<script>
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let greeting = $state('')

  async function loadGreeting() {
    greeting = await invoke('greet', { name: 'SkyWidget' })
  }

  onMount(() => {
    loadGreeting()
  })
</script>

<main class="container mx-auto p-4">
  <div class="text-center">
    <h1 class="text-4xl font-bold mb-4 text-blue-600">
      SkyWidget
    </h1>
    <p class="text-xl mb-8">
      跨平台硬件监控桌面磁贴应用
    </p>

    {#if greeting}
      <div class="bg-gray-100 dark:bg-gray-800 p-6 rounded-lg shadow-lg">
        <p class="text-lg">{greeting}</p>
      </div>
    {/if}

    <div class="mt-8 text-sm text-gray-600 dark:text-gray-400">
      <p>✅ Tauri + Svelte 5 初始化成功</p>
      <p>✅ TailwindCSS 配置完成</p>
      <p>🚀 准备开始开发...</p>
    </div>
  </div>
</main>

<style>
  :global(body) {
    @apply bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100;
  }
</style>
