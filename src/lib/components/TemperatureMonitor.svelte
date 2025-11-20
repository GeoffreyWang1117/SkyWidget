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

  // 南桥温度颜色（阈值更低，因为南桥过热更危险）
  function getChipsetTempColor(temp) {
    if (temp >= 70) return 'text-red-600 dark:text-red-400'
    if (temp >= 60) return 'text-orange-600 dark:text-orange-400'
    if (temp >= 50) return 'text-yellow-600 dark:text-yellow-400'
    return 'text-green-600 dark:text-green-400'
  }

  function getChipsetBgColor(temp) {
    if (temp >= 70) return 'bg-red-100 dark:bg-red-900/20 border-red-300 dark:border-red-700'
    if (temp >= 60) return 'bg-orange-100 dark:bg-orange-900/20 border-orange-300 dark:border-orange-700'
    if (temp >= 50) return 'bg-yellow-100 dark:bg-yellow-900/20 border-yellow-300 dark:border-yellow-700'
    return 'bg-green-100 dark:bg-green-900/20 border-green-300 dark:border-green-700'
  }

  function getSensorIcon(sensorType) {
    switch (sensorType) {
      case 'Cpu': return '🔥'
      case 'Gpu': return '🎮'
      case 'Chipset': return '⚠️'
      case 'Disk': return '💾'
      default: return '📊'
    }
  }

  function getSensorTypeName(sensorType) {
    switch (sensorType) {
      case 'Cpu': return 'CPU'
      case 'Gpu': return 'GPU'
      case 'Chipset': return '南桥/PCH'
      case 'Disk': return '磁盘'
      default: return '其他'
    }
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
    <div class="space-y-3 mb-4">
      <!-- CPU 平均温度 -->
      {#if temperatureInfo.cpu_avg_temp !== null}
        <div class="p-4 {getTempBgColor(temperatureInfo.cpu_avg_temp)} rounded-lg">
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">🔥 CPU 平均温度</span>
            <span class={`text-2xl font-bold ${getTempColor(temperatureInfo.cpu_avg_temp)}`}>
              {temperatureInfo.cpu_avg_temp.toFixed(1)}°C
            </span>
          </div>
        </div>
      {/if}

      <!-- 南桥/PCH 温度（重点显示） -->
      {#if temperatureInfo.chipset_temp !== null}
        <div class="p-4 border-2 {getChipsetBgColor(temperatureInfo.chipset_temp)} rounded-lg">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-bold text-gray-800 dark:text-gray-200">⚠️ 南桥/PCH 温度</span>
            <span class={`text-2xl font-bold ${getChipsetTempColor(temperatureInfo.chipset_temp)}`}>
              {temperatureInfo.chipset_temp.toFixed(1)}°C
            </span>
          </div>
          {#if temperatureInfo.chipset_temp >= 60}
            <div class="text-xs text-red-700 dark:text-red-300 mt-2 flex items-start gap-1">
              <span>⚠️</span>
              <span>警告：南桥温度偏高！可能导致磁盘掉线或 CMOS 错误</span>
            </div>
          {:else if temperatureInfo.chipset_temp >= 50}
            <div class="text-xs text-yellow-700 dark:text-yellow-300 mt-2 flex items-start gap-1">
              <span>⚡</span>
              <span>提示：建议检查机箱散热和南桥风道</span>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <!-- 传感器列表 -->
    {#if temperatureInfo.sensors.length > 0}
      <div class="space-y-2">
        <h3 class="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-2">
          传感器详情 ({temperatureInfo.sensors.length})
        </h3>
        <div class="max-h-[200px] overflow-y-auto space-y-2">
          {#each temperatureInfo.sensors as sensor}
            <div class="flex items-center justify-between p-2 {sensor.sensor_type === 'Chipset' ? 'bg-yellow-50 dark:bg-yellow-900/10 border border-yellow-200 dark:border-yellow-800' : 'bg-gray-50 dark:bg-gray-700'} rounded">
              <div class="flex-1 min-w-0 flex items-start gap-2">
                <span class="text-sm">{getSensorIcon(sensor.sensor_type)}</span>
                <div class="flex-1 min-w-0">
                  <p class="text-xs font-medium text-gray-700 dark:text-gray-300 truncate">
                    {sensor.label}
                  </p>
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    {getSensorTypeName(sensor.sensor_type)}
                    {#if sensor.critical}
                      • 临界: {sensor.critical.toFixed(1)}°C
                    {/if}
                  </p>
                </div>
              </div>
              <div class="text-right ml-2">
                <span class={`text-lg font-semibold ${sensor.sensor_type === 'Chipset' ? getChipsetTempColor(sensor.temperature) : getTempColor(sensor.temperature)}`}>
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
    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700 space-y-2">
      <div class="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-2">CPU/GPU 温度阈值</div>
      <div class="flex items-center justify-between text-xs flex-wrap gap-2">
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
          <span class="text-gray-600 dark:text-gray-400">≥80°C</span>
        </div>
      </div>

      <div class="text-xs font-semibold text-gray-600 dark:text-gray-400 mt-3 mb-2">⚠️ 南桥/PCH 阈值（更严格）</div>
      <div class="flex items-center justify-between text-xs flex-wrap gap-2">
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-green-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">&lt;50°C 正常</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-yellow-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">50-60°C 注意</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-orange-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">60-70°C 偏高</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 bg-red-500 rounded"></div>
          <span class="text-gray-600 dark:text-gray-400">≥70°C 危险</span>
        </div>
      </div>

      <div class="mt-3 p-2 bg-yellow-50 dark:bg-yellow-900/10 border border-yellow-200 dark:border-yellow-800 rounded text-xs text-yellow-800 dark:text-yellow-300">
        💡 提示：南桥芯片过热可能导致磁盘掉线、CMOS 错误或系统不稳定。建议温度保持在 60°C 以下。
      </div>
    </div>
  {/if}
</div>
