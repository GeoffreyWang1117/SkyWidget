<script>
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let gpus = $state([])
  let loading = $state(true)
  let error = $state(null)
  let supported = $state(false)
  let refreshInterval = null

  async function loadGpuInfo() {
    try {
      const [gpuData, isSupported] = await Promise.all([
        invoke('get_gpu_info'),
        invoke('is_gpu_supported')
      ])

      gpus = gpuData
      supported = isSupported
      error = null
    } catch (e) {
      error = e?.toString() || '获取 GPU 信息失败'
      console.error('Failed to load GPU info:', e)
    } finally {
      loading = false
    }
  }

  onMount(() => {
    loadGpuInfo()
    // 每 2 秒刷新一次
    refreshInterval = setInterval(loadGpuInfo, 2000)
  })

  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })

  function getVendorIcon(vendor) {
    switch (vendor) {
      case 'Nvidia':
        return '🟩'
      case 'Amd':
        return '🟥'
      case 'Intel':
        return '🟦'
      default:
        return '🔲'
    }
  }

  function getVendorColor(vendor) {
    switch (vendor) {
      case 'Nvidia':
        return 'text-green-600 dark:text-green-400'
      case 'Amd':
        return 'text-red-600 dark:text-red-400'
      case 'Intel':
        return 'text-blue-600 dark:text-blue-400'
      default:
        return 'text-gray-600 dark:text-gray-400'
    }
  }

  function getTempColor(temp) {
    if (!temp) return 'text-gray-500 dark:text-gray-400'
    if (temp >= 85) return 'text-red-600 dark:text-red-400'
    if (temp >= 75) return 'text-orange-600 dark:text-orange-400'
    if (temp >= 65) return 'text-yellow-600 dark:text-yellow-400'
    return 'text-green-600 dark:text-green-400'
  }

  function getUsageColor(usage) {
    if (!usage) return 'text-gray-500 dark:text-gray-400'
    if (usage >= 90) return 'text-red-600 dark:text-red-400'
    if (usage >= 70) return 'text-orange-600 dark:text-orange-400'
    if (usage >= 50) return 'text-yellow-600 dark:text-yellow-400'
    return 'text-green-600 dark:text-green-400'
  }

  function formatMemory(mb) {
    if (!mb) return 'N/A'
    if (mb >= 1024) {
      return `${(mb / 1024).toFixed(1)} GB`
    }
    return `${mb} MB`
  }
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-gray-100 flex items-center gap-2">
    🎮 GPU 监控
  </h2>

  {#if loading}
    <div class="text-center py-4">
      <div class="inline-block animate-spin rounded-full h-8 w-8 border-4 border-blue-600 border-t-transparent"></div>
    </div>
  {:else if error}
    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded p-4">
      <p class="text-red-600 dark:text-red-400 text-sm">{error}</p>
    </div>
  {:else if !supported || (gpus.length === 1 && gpus[0].vendor === 'Unknown')}
    <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded p-4">
      <p class="text-yellow-700 dark:text-yellow-400 text-sm mb-2">⚠️ 未检测到独立显卡</p>
      <p class="text-xs text-yellow-600 dark:text-yellow-500">
        当前仅支持 NVIDIA GPU。AMD 和 Intel Arc 支持即将推出。
      </p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each gpus as gpu}
        <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
          <!-- GPU 名称和厂商 -->
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <span class="text-2xl">{getVendorIcon(gpu.vendor)}</span>
              <div>
                <h3 class="font-semibold {getVendorColor(gpu.vendor)}">{gpu.name}</h3>
                {#if gpu.driver_version}
                  <p class="text-xs text-gray-500 dark:text-gray-400">驱动: {gpu.driver_version}</p>
                {/if}
              </div>
            </div>
            {#if gpu.temperature !== null}
              <div class="text-right">
                <div class="text-2xl font-bold {getTempColor(gpu.temperature)}">
                  {gpu.temperature.toFixed(1)}°C
                </div>
              </div>
            {/if}
          </div>

          <!-- GPU 指标网格 -->
          <div class="grid grid-cols-2 gap-3 text-sm">
            <!-- GPU 使用率 -->
            {#if gpu.utilization !== null}
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded p-3">
                <div class="text-gray-600 dark:text-gray-400 text-xs mb-1">GPU 使用率</div>
                <div class="flex items-baseline gap-1">
                  <span class="text-lg font-bold {getUsageColor(gpu.utilization)}">
                    {gpu.utilization.toFixed(1)}
                  </span>
                  <span class="text-gray-500 dark:text-gray-400">%</span>
                </div>
              </div>
            {/if}

            <!-- 显存使用率 -->
            {#if gpu.memory_utilization !== null}
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded p-3">
                <div class="text-gray-600 dark:text-gray-400 text-xs mb-1">显存使用率</div>
                <div class="flex items-baseline gap-1">
                  <span class="text-lg font-bold {getUsageColor(gpu.memory_utilization)}">
                    {gpu.memory_utilization.toFixed(1)}
                  </span>
                  <span class="text-gray-500 dark:text-gray-400">%</span>
                </div>
              </div>
            {/if}

            <!-- 显存信息 -->
            {#if gpu.memory_total !== null && gpu.memory_used !== null}
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded p-3">
                <div class="text-gray-600 dark:text-gray-400 text-xs mb-1">显存</div>
                <div class="text-sm text-gray-700 dark:text-gray-300">
                  {formatMemory(gpu.memory_used)} / {formatMemory(gpu.memory_total)}
                </div>
              </div>
            {/if}

            <!-- 功耗 -->
            {#if gpu.power_usage !== null}
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded p-3">
                <div class="text-gray-600 dark:text-gray-400 text-xs mb-1">功耗</div>
                <div class="flex items-baseline gap-1">
                  <span class="text-lg font-bold text-purple-600 dark:text-purple-400">
                    {gpu.power_usage.toFixed(1)}
                  </span>
                  <span class="text-gray-500 dark:text-gray-400">W</span>
                </div>
              </div>
            {/if}

            <!-- 风扇转速 -->
            {#if gpu.fan_speed !== null}
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded p-3">
                <div class="text-gray-600 dark:text-gray-400 text-xs mb-1">风扇转速</div>
                <div class="flex items-baseline gap-1">
                  <span class="text-lg font-bold text-blue-600 dark:text-blue-400">
                    {gpu.fan_speed.toFixed(0)}
                  </span>
                  <span class="text-gray-500 dark:text-gray-400">%</span>
                </div>
              </div>
            {/if}

            <!-- 时钟频率 -->
            {#if gpu.clock_speed !== null}
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded p-3">
                <div class="text-gray-600 dark:text-gray-400 text-xs mb-1">核心频率</div>
                <div class="flex items-baseline gap-1">
                  <span class="text-lg font-bold text-indigo-600 dark:text-indigo-400">
                    {gpu.clock_speed}
                  </span>
                  <span class="text-gray-500 dark:text-gray-400">MHz</span>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>

    <!-- GPU 温度图例 -->
    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
      <p class="text-xs text-gray-500 dark:text-gray-400 mb-2">温度图例:</p>
      <div class="flex flex-wrap gap-3 text-xs">
        <span class="text-green-600 dark:text-green-400">● &lt;65°C 正常</span>
        <span class="text-yellow-600 dark:text-yellow-400">● 65-75°C 适中</span>
        <span class="text-orange-600 dark:text-orange-400">● 75-85°C 偏高</span>
        <span class="text-red-600 dark:text-red-400">● ≥85°C 过热</span>
      </div>
    </div>
  {/if}
</div>
