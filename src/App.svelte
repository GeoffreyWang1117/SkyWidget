<script>
  import { onMount, onDestroy } from 'svelte'
  import { getAllHardwareInfo } from './lib/utils/tauri'
  import type { AllHardwareInfo } from './lib/utils/tauri'
  import CpuMonitor from './lib/components/CpuMonitor.svelte'
  import MemoryMonitor from './lib/components/MemoryMonitor.svelte'
  import DiskMonitor from './lib/components/DiskMonitor.svelte'

  let hardwareInfo = $state<AllHardwareInfo | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)
  let refreshInterval: number | null = null
  let lastUpdate = $state<string>('')

  // 加载硬件信息
  async function loadHardwareInfo() {
    try {
      hardwareInfo = await getAllHardwareInfo()
      error = null

      // 更新最后刷新时间
      const now = new Date()
      lastUpdate = now.toLocaleTimeString('zh-CN')
    } catch (e) {
      error = e instanceof Error ? e.message : '获取硬件信息失败'
      console.error('Failed to load hardware info:', e)
    } finally {
      loading = false
    }
  }

  // 启动自动刷新
  function startAutoRefresh() {
    // 每秒刷新一次
    refreshInterval = window.setInterval(() => {
      loadHardwareInfo()
    }, 1000)
  }

  // 停止自动刷新
  function stopAutoRefresh() {
    if (refreshInterval !== null) {
      clearInterval(refreshInterval)
      refreshInterval = null
    }
  }

  onMount(() => {
    loadHardwareInfo()
    startAutoRefresh()
  })

  onDestroy(() => {
    stopAutoRefresh()
  })
</script>

<main class="min-h-screen bg-gray-50 dark:bg-gray-900 p-4">
  <div class="max-w-6xl mx-auto">
    <!-- 标题栏 -->
    <div class="text-center mb-6">
      <h1 class="text-4xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-2">
        SkyWidget
      </h1>
      <p class="text-gray-600 dark:text-gray-400 text-sm">
        跨平台硬件监控桌面应用
      </p>
      {#if lastUpdate}
        <p class="text-xs text-gray-500 dark:text-gray-500 mt-1">
          最后更新: {lastUpdate}
        </p>
      {/if}
    </div>

    {#if loading}
      <!-- 加载状态 -->
      <div class="flex items-center justify-center h-64">
        <div class="text-center">
          <div class="inline-block animate-spin rounded-full h-12 w-12 border-4 border-blue-600 border-t-transparent"></div>
          <p class="mt-4 text-gray-600 dark:text-gray-400">加载硬件信息中...</p>
        </div>
      </div>
    {:else if error}
      <!-- 错误状态 -->
      <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-6 text-center">
        <p class="text-red-600 dark:text-red-400 font-semibold mb-2">⚠️ 错误</p>
        <p class="text-red-500 dark:text-red-300">{error}</p>
        <button
          onclick={() => loadHardwareInfo()}
          class="mt-4 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-md transition-colors"
        >
          重试
        </button>
      </div>
    {:else if hardwareInfo}
      <!-- 硬件监控面板 -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <!-- CPU 监控 -->
        <CpuMonitor cpuInfo={hardwareInfo.cpu} />

        <!-- 内存监控 -->
        <MemoryMonitor memoryInfo={hardwareInfo.memory} />

        <!-- 磁盘监控 -->
        <DiskMonitor diskInfo={hardwareInfo.disk} />
      </div>

      <!-- 底部信息 -->
      <div class="mt-6 text-center text-xs text-gray-500 dark:text-gray-500">
        <p>✨ Tauri + Svelte 5 + Rust</p>
        <p class="mt-1">🔄 自动刷新间隔: 1 秒</p>
      </div>
    {/if}
  </div>
</main>

<style>
  :global(body) {
    @apply bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100;
    margin: 0;
    padding: 0;
  }
</style>
