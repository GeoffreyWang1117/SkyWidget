<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { getAllHardwareInfo } from './lib/utils/tauri'
  import type { AllHardwareInfo } from './lib/utils/tauri'
  import { configStore } from './lib/stores/config.svelte'
  import DynamicLayout from './lib/components/DynamicLayout.svelte'
  import SettingsPanel from './lib/components/SettingsPanel.svelte'

  let hardwareInfo = $state<AllHardwareInfo | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)
  let refreshInterval: number | null = null
  let lastUpdate = $state<string>('')
  let settingsOpen = $state(false)

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
    // 使用配置中的刷新间隔
    const interval = configStore.config.performance.globalRefreshInterval
    refreshInterval = window.setInterval(() => {
      loadHardwareInfo()
    }, interval)
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

<main class="min-h-screen bg-gray-50 dark:bg-gray-900 p-4" data-theme={configStore.currentTheme()}>
  <div class="max-w-6xl mx-auto">
    <!-- 标题栏和设置按钮 -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex-1 text-center">
        <h1 class="text-4xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-2">
          SkyWidget
        </h1>
        <p class="text-gray-600 dark:text-gray-400 text-sm">
          分布式硬件监控与告警系统
        </p>
        {#if lastUpdate}
          <p class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            最后更新: {lastUpdate}
          </p>
        {/if}
      </div>
      <button
        class="settings-button"
        onclick={() => (settingsOpen = true)}
        title="设置"
      >
        ⚙️
      </button>
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
      <!-- 动态布局容器 -->
      <DynamicLayout />

      <!-- 底部信息 -->
      <div class="mt-6 text-center text-xs text-gray-500 dark:text-gray-500">
        <p>✨ Tauri + Svelte 5 + Rust + P2P</p>
        <p class="mt-1">
          🔄 硬件监控: {configStore.config.performance.globalRefreshInterval}ms | 网络发现: 5秒 | 告警检测: 10秒
        </p>
        <p class="mt-1">🌐 mDNS 自动发现 | 📡 HTTP API (端口 3030)</p>
      </div>
    {/if}
  </div>
</main>

<!-- 设置面板 -->
<SettingsPanel bind:isOpen={settingsOpen} />

<style>
  :global(body) {
    background-color: white;
    color: #111827;
    margin: 0;
    padding: 0;
  }

  :global([data-theme='dark'] body) {
    background-color: #111827;
    color: #f9fafb;
  }

  .settings-button {
    position: fixed;
    top: 1rem;
    right: 1rem;
    width: 3rem;
    height: 3rem;
    border-radius: 50%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
    color: white;
    font-size: 1.5rem;
    cursor: pointer;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .settings-button:hover {
    transform: scale(1.1) rotate(90deg);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
  }

  .settings-button:active {
    transform: scale(0.95) rotate(90deg);
  }
</style>
