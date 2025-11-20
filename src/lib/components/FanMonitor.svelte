<script>
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let fansInfo = $state(null)
  let supported = $state(false)
  let loading = $state(true)
  let error = $state(null)
  let refreshInterval = null

  async function loadFanInfo() {
    try {
      const [fanData, isSupported, hasFailures] = await Promise.all([
        invoke('get_fan_info'),
        invoke('is_fan_supported'),
        invoke('has_fan_failures')
      ])

      fansInfo = fanData
      supported = isSupported
      error = null

      // 如果有风扇故障，在控制台输出警告
      if (hasFailures) {
        console.warn('⚠️ 检测到风扇故障！')
      }
    } catch (e) {
      error = e?.toString() || '获取风扇信息失败'
      console.error('Failed to load fan info:', e)
    } finally {
      loading = false
    }
  }

  onMount(() => {
    loadFanInfo()
    // 每 2 秒刷新一次
    refreshInterval = setInterval(loadFanInfo, 2000)
  })

  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })

  function getFanTypeIcon(fanType) {
    switch (fanType) {
      case 'CpuFan':
        return '🔥'
      case 'GpuFan':
        return '🎮'
      case 'CaseFan':
        return '💨'
      case 'ChipsetFan':
        return '⚠️'
      case 'PowerSupply':
        return '⚡'
      default:
        return '🌀'
    }
  }

  function getFanTypeName(fanType) {
    switch (fanType) {
      case 'CpuFan':
        return 'CPU 风扇'
      case 'GpuFan':
        return 'GPU 风扇'
      case 'CaseFan':
        return '机箱风扇'
      case 'ChipsetFan':
        return '南桥风扇'
      case 'PowerSupply':
        return '电源风扇'
      default:
        return '其他风扇'
    }
  }

  function getStatusColor(status) {
    switch (status) {
      case 'Running':
        return 'text-green-600 dark:text-green-400'
      case 'Stopped':
        return 'text-red-600 dark:text-red-400'
      case 'SlowSpeed':
        return 'text-orange-600 dark:text-orange-400'
      default:
        return 'text-gray-500 dark:text-gray-400'
    }
  }

  function getStatusBgColor(status) {
    switch (status) {
      case 'Running':
        return 'bg-green-50 dark:bg-green-900/10 border-green-200 dark:border-green-800'
      case 'Stopped':
        return 'bg-red-50 dark:bg-red-900/20 border-red-300 dark:border-red-700'
      case 'SlowSpeed':
        return 'bg-orange-50 dark:bg-orange-900/20 border-orange-300 dark:border-orange-700'
      default:
        return 'bg-gray-50 dark:bg-gray-700 border-gray-200 dark:border-gray-600'
    }
  }

  function getStatusText(status) {
    switch (status) {
      case 'Running':
        return '✓ 正常运行'
      case 'Stopped':
        return '✗ 已停转'
      case 'SlowSpeed':
        return '⚠ 转速过低'
      default:
        return '? 未知'
    }
  }

  function getRpmColor(rpm) {
    if (!rpm) return 'text-gray-500 dark:text-gray-400'
    if (rpm === 0) return 'text-red-600 dark:text-red-400'
    if (rpm < 500) return 'text-orange-600 dark:text-orange-400'
    if (rpm < 1000) return 'text-yellow-600 dark:text-yellow-400'
    return 'text-green-600 dark:text-green-400'
  }
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <h2 class="text-lg font-semibold mb-4 text-gray-900 dark:text-gray-100 flex items-center gap-2">
    🌀 风扇监控
  </h2>

  {#if loading}
    <div class="text-center py-4">
      <div
        class="inline-block animate-spin rounded-full h-6 w-6 border-4 border-blue-600 border-t-transparent"
      ></div>
    </div>
  {:else if !supported}
    <div class="text-center py-8 text-gray-500 dark:text-gray-400">
      <p class="mb-2">❌ 当前系统不支持风扇监控</p>
      <p class="text-xs">
        某些平台可能需要额外的驱动程序或管理员权限
      </p>
    </div>
  {:else if error}
    <div
      class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded p-3"
    >
      <p class="text-sm text-red-600 dark:text-red-400">{error}</p>
    </div>
  {:else if fansInfo}
    <!-- 统计概览 -->
    {#if fansInfo.total_count > 0}
      <div class="grid grid-cols-3 gap-3 mb-4">
        <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-3 text-center">
          <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
            {fansInfo.total_count}
          </div>
          <div class="text-xs text-gray-600 dark:text-gray-400 mt-1">总风扇数</div>
        </div>

        <div
          class="bg-{fansInfo.stopped_count > 0 ? 'red' : 'green'}-50 dark:bg-{fansInfo.stopped_count > 0 ? 'red' : 'green'}-900/20 rounded-lg p-3 text-center"
        >
          <div class="text-2xl font-bold text-{fansInfo.stopped_count > 0 ? 'red' : 'green'}-600 dark:text-{fansInfo.stopped_count > 0 ? 'red' : 'green'}-400">
            {fansInfo.stopped_count}
          </div>
          <div class="text-xs text-gray-600 dark:text-gray-400 mt-1">停转风扇</div>
        </div>

        <div
          class="bg-{fansInfo.slow_speed_count > 0 ? 'orange' : 'green'}-50 dark:bg-{fansInfo.slow_speed_count > 0 ? 'orange' : 'green'}-900/20 rounded-lg p-3 text-center"
        >
          <div class="text-2xl font-bold text-{fansInfo.slow_speed_count > 0 ? 'orange' : 'green'}-600 dark:text-{fansInfo.slow_speed_count > 0 ? 'orange' : 'green'}-400">
            {fansInfo.slow_speed_count}
          </div>
          <div class="text-xs text-gray-600 dark:text-gray-400 mt-1">低速风扇</div>
        </div>
      </div>

      <!-- 故障警告 -->
      {#if fansInfo.stopped_count > 0 || fansInfo.slow_speed_count > 0}
        <div
          class="mb-4 p-3 bg-red-50 dark:bg-red-900/20 border-2 border-red-300 dark:border-red-700 rounded-lg"
        >
          <div class="flex items-start gap-2">
            <span class="text-xl">⚠️</span>
            <div class="flex-1">
              <p class="text-sm font-bold text-red-700 dark:text-red-300">
                检测到风扇异常！
              </p>
              <p class="text-xs text-red-600 dark:text-red-400 mt-1">
                风扇故障可能导致硬件过热、性能下降甚至损坏。请立即检查风扇连接和状态。
              </p>
            </div>
          </div>
        </div>
      {/if}
    {/if}

    <!-- 风扇列表 -->
    {#if fansInfo.fans.length > 0}
      <div class="space-y-2">
        <h3 class="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-2">
          风扇详情 ({fansInfo.fans.length})
        </h3>
        <div class="max-h-[300px] overflow-y-auto space-y-2">
          {#each fansInfo.fans as fan}
            <div class="border-2 {getStatusBgColor(fan.status)} rounded-lg p-3">
              <div class="flex items-start justify-between gap-3">
                <!-- 风扇信息 -->
                <div class="flex-1 min-w-0 flex items-start gap-2">
                  <span class="text-xl">{getFanTypeIcon(fan.fan_type)}</span>
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
                      {fan.label}
                    </p>
                    <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                      {getFanTypeName(fan.fan_type)}
                    </p>
                    <div class="mt-1">
                      <span class={`text-xs font-semibold ${getStatusColor(fan.status)}`}>
                        {getStatusText(fan.status)}
                      </span>
                    </div>
                  </div>
                </div>

                <!-- RPM 显示 -->
                <div class="text-right">
                  {#if fan.rpm !== null}
                    <div class={`text-2xl font-bold ${getRpmColor(fan.rpm)}`}>
                      {fan.rpm.toFixed(0)}
                    </div>
                    <div class="text-xs text-gray-500 dark:text-gray-400">RPM</div>
                  {:else}
                    <div class="text-sm text-gray-500 dark:text-gray-400">N/A</div>
                  {/if}
                </div>
              </div>

              <!-- 转速范围（如果可用） -->
              {#if fan.min_rpm !== null || fan.max_rpm !== null}
                <div class="mt-2 pt-2 border-t border-gray-200 dark:border-gray-600">
                  <div class="flex justify-between text-xs text-gray-600 dark:text-gray-400">
                    {#if fan.min_rpm !== null}
                      <span>最小: {fan.min_rpm.toFixed(0)} RPM</span>
                    {/if}
                    {#if fan.max_rpm !== null}
                      <span>最大: {fan.max_rpm.toFixed(0)} RPM</span>
                    {/if}
                  </div>
                </div>
              {/if}

              <!-- PWM 百分比（如果可用） -->
              {#if fan.pwm_percent !== null}
                <div class="mt-2">
                  <div class="flex items-center gap-2">
                    <div class="flex-1 bg-gray-200 dark:bg-gray-600 rounded-full h-2">
                      <div
                        class="bg-blue-600 dark:bg-blue-400 h-2 rounded-full transition-all"
                        style="width: {fan.pwm_percent}%"
                      ></div>
                    </div>
                    <span class="text-xs text-gray-600 dark:text-gray-400 w-12 text-right">
                      {fan.pwm_percent.toFixed(0)}%
                    </span>
                  </div>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="text-center py-4 text-gray-500 dark:text-gray-400">
        <p>未检测到风扇</p>
      </div>
    {/if}

    <!-- 状态图例 -->
    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
      <p class="text-xs text-gray-500 dark:text-gray-400 mb-2">状态图例:</p>
      <div class="flex flex-wrap gap-3 text-xs">
        <span class="text-green-600 dark:text-green-400">● 正常运行 (≥500 RPM)</span>
        <span class="text-orange-600 dark:text-orange-400">● 转速过低 (&lt;500 RPM)</span>
        <span class="text-red-600 dark:text-red-400">● 已停转 (0 RPM)</span>
      </div>

      <div class="mt-3 p-2 bg-yellow-50 dark:bg-yellow-900/10 border border-yellow-200 dark:border-yellow-800 rounded text-xs text-yellow-800 dark:text-yellow-300">
        💡 提示：风扇停转是最常见但最难察觉的硬件故障。停转的风扇会导致连锁反应：过热→性能下降→硬件损坏。建议定期检查风扇状态。
      </div>
    </div>
  {/if}
</div>
