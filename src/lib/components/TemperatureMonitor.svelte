<script>
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let temperatureInfo = $state(null)
  let supported = $state(false)
  let loading = $state(true)
  let error = $state(null)

  async function loadTemperature() {
    try {
      const isSupported = await invoke('is_temperature_supported')
      supported = isSupported

      if (isSupported) {
        temperatureInfo = await invoke('get_temperature_info')
      }
      error = null
    } catch (e) {
      error = e?.toString() || '获取温度信息失败'
      console.error('Failed to load temperature info:', e)
    } finally {
      loading = false
    }
  }

  onMount(() => {
    loadTemperature()
  })

  function getTempColor(temp) {
    if (temp >= 80) return 'text-red-600 dark:text-red-400'
    if (temp >= 70) return 'text-orange-600 dark:text-orange-400'
    if (temp >= 60) return 'text-yellow-600 dark:text-yellow-400'
    return 'text-green-600 dark:text-green-400'
  }

  function getTempBgColor(temp) {
    if (temp >= 80) return 'bg-red-100 dark:bg-red-900/20'
    if (temp >= 70) return 'bg-orange-100 dark:bg-orange-900/20'
    if (temp >= 60) return 'bg-yellow-100 dark:bg-yellow-900/20'
    return 'bg-green-100 dark:bg-green-900/20'
  }
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <h2 class="text-lg font-semibold mb-4 text-gray-900 dark:text-gray-100 flex items-center gap-2">
    🌡️ 温度监控
  </h2>

  {#if loading}
    <div class="text-center py-4">
      <div
        class="inline-block animate-spin rounded-full h-6 w-6 border-4 border-blue-600 border-t-transparent"
      ></div>
    </div>
  {:else if !supported}
    <div class="text-center py-8 text-gray-500 dark:text-gray-400">
      <p class="mb-2">❌ 当前系统不支持温度监控</p>
      <p class="text-xs">某些平台可能需要额外的驱动程序</p>
    </div>
  {:else if error}
    <div
      class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded p-3"
    >
      <p class="text-sm text-red-600 dark:text-red-400">{error}</p>
    </div>
  {:else if temperatureInfo}
    <!-- CPU 平均温度 -->
    {#if temperatureInfo.cpu_avg_temp !== null}
      <div class="mb-4 p-4 {getTempBgColor(temperatureInfo.cpu_avg_temp)} rounded-lg">
        <div class="flex items-center justify-between">
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">CPU 平均温度</span>
          <span class={`text-2xl font-bold ${getTempColor(temperatureInfo.cpu_avg_temp)}`}>
            {temperatureInfo.cpu_avg_temp.toFixed(1)}°C
          </span>
        </div>
      </div>
    {/if}

    <!-- 传感器列表 -->
    {#if temperatureInfo.sensors.length > 0}
      <div class="space-y-2">
        <h3 class="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-2">
          传感器详情 ({temperatureInfo.sensors.length})
        </h3>
        <div class="max-h-[200px] overflow-y-auto space-y-2">
          {#each temperatureInfo.sensors as sensor}
            <div class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-700 rounded">
              <div class="flex-1 min-w-0">
                <p class="text-xs font-medium text-gray-700 dark:text-gray-300 truncate">
                  {sensor.label}
                </p>
                {#if sensor.critical}
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    临界: {sensor.critical.toFixed(1)}°C
                  </p>
                {/if}
              </div>
              <div class="text-right ml-2">
                <span class={`text-lg font-semibold ${getTempColor(sensor.temperature)}`}>
                  {sensor.temperature.toFixed(1)}°C
                </span>
                {#if sensor.max_temperature}
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    最高: {sensor.max_temperature.toFixed(1)}°C
                  </p>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="text-center py-4 text-gray-500 dark:text-gray-400">
        <p>未检测到温度传感器</p>
      </div>
    {/if}

    <!-- 温度图例 -->
    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
      <div class="flex items-center justify-between text-xs">
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-green-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">&lt;60°C</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-yellow-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">60-70°C</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-orange-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">70-80°C</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-red-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">&gt;=80°C</span>
        </div>
      </div>
    </div>
  {/if}
</div>
