<script>
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let cpuData = $state([])
  let memoryData = $state([])
  let diskData = $state([])
  let loading = $state(true)
  let error = $state(null)
  let refreshInterval = null

  const chartWidth = 400
  const chartHeight = 120
  const padding = { top: 10, right: 10, bottom: 20, left: 40 }
  const maxDataPoints = 60 // 显示最近60个数据点（10分钟）

  async function loadMetricsHistory() {
    try {
      const [cpu, memory, disk] = await Promise.all([
        invoke('get_metrics_history', { metricName: 'cpu_usage', maxPoints: maxDataPoints }),
        invoke('get_metrics_history', { metricName: 'memory_usage_percent', maxPoints: maxDataPoints }),
        invoke('get_metrics_history', { metricName: 'disk_usage_percent', maxPoints: maxDataPoints })
      ])

      cpuData = cpu
      memoryData = memory
      diskData = disk
      error = null
    } catch (e) {
      error = e?.toString() || '获取指标历史失败'
      console.error('Failed to load metrics history:', e)
    } finally {
      loading = false
    }
  }

  onMount(() => {
    loadMetricsHistory()
    // 每30秒刷新一次
    refreshInterval = setInterval(loadMetricsHistory, 30000)
  })

  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })

  function generatePath(data) {
    if (!data || data.length === 0) return ''

    const innerWidth = chartWidth - padding.left - padding.right
    const innerHeight = chartHeight - padding.top - padding.bottom

    // 找到最大值用于缩放
    const maxValue = Math.max(...data.map(d => d.value), 100)
    const minValue = 0

    const xStep = innerWidth / Math.max(data.length - 1, 1)

    const points = data.map((point, i) => {
      const x = padding.left + i * xStep
      const y = padding.top + innerHeight - ((point.value - minValue) / (maxValue - minValue)) * innerHeight
      return `${x},${y}`
    })

    return `M ${points.join(' L ')}`
  }

  function generateGridLines() {
    const lines = []
    const innerHeight = chartHeight - padding.top - padding.bottom

    // 5条水平网格线 (0%, 25%, 50%, 75%, 100%)
    for (let i = 0; i <= 4; i++) {
      const y = padding.top + (innerHeight * i) / 4
      lines.push({
        y,
        label: 100 - (i * 25)
      })
    }

    return lines
  }

  function formatTime(timestamp) {
    const date = new Date(timestamp)
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  }

  function getLatestValue(data) {
    if (!data || data.length === 0) return 'N/A'
    return data[data.length - 1].value.toFixed(1) + '%'
  }

  function getAverageValue(data) {
    if (!data || data.length === 0) return 'N/A'
    const sum = data.reduce((acc, point) => acc + point.value, 0)
    return (sum / data.length).toFixed(1) + '%'
  }

  const gridLines = generateGridLines()
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-gray-100 flex items-center gap-2">
    📊 性能趋势图
  </h2>

  {#if loading}
    <div class="text-center py-8">
      <div class="inline-block animate-spin rounded-full h-8 w-8 border-4 border-blue-600 border-t-transparent"></div>
    </div>
  {:else if error}
    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded p-4">
      <p class="text-red-600 dark:text-red-400">{error}</p>
    </div>
  {:else}
    <div class="space-y-6">
      <!-- CPU 使用率图表 -->
      <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
        <div class="flex items-center justify-between mb-2">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">CPU 使用率</h3>
          <div class="flex gap-4 text-xs text-gray-500 dark:text-gray-400">
            <span>当前: <strong class="text-blue-600 dark:text-blue-400">{getLatestValue(cpuData)}</strong></span>
            <span>平均: <strong>{getAverageValue(cpuData)}</strong></span>
          </div>
        </div>
        <svg width={chartWidth} height={chartHeight} class="w-full">
          <!-- 网格线 -->
          {#each gridLines as line}
            <line
              x1={padding.left}
              y1={line.y}
              x2={chartWidth - padding.right}
              y2={line.y}
              stroke="currentColor"
              class="text-gray-300 dark:text-gray-600"
              stroke-width="0.5"
              stroke-dasharray="4,4"
            />
            <text
              x={padding.left - 5}
              y={line.y + 4}
              text-anchor="end"
              class="text-gray-500 dark:text-gray-400"
              font-size="10"
            >
              {line.label}%
            </text>
          {/each}

          <!-- 数据线 -->
          {#if cpuData.length > 0}
            <path
              d={generatePath(cpuData)}
              fill="none"
              stroke="rgb(37, 99, 235)"
              stroke-width="2"
              class="transition-all"
            />
            <!-- 填充区域 -->
            <path
              d={generatePath(cpuData) + ` L ${chartWidth - padding.right},${chartHeight - padding.bottom} L ${padding.left},${chartHeight - padding.bottom} Z`}
              fill="rgb(37, 99, 235)"
              fill-opacity="0.1"
            />
          {/if}

          <!-- 时间轴标签 -->
          {#if cpuData.length > 0}
            <text
              x={padding.left}
              y={chartHeight - 5}
              class="text-gray-500 dark:text-gray-400"
              font-size="9"
            >
              {formatTime(cpuData[0].timestamp)}
            </text>
            <text
              x={chartWidth - padding.right}
              y={chartHeight - 5}
              text-anchor="end"
              class="text-gray-500 dark:text-gray-400"
              font-size="9"
            >
              {formatTime(cpuData[cpuData.length - 1].timestamp)}
            </text>
          {/if}
        </svg>
      </div>

      <!-- 内存使用率图表 -->
      <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
        <div class="flex items-center justify-between mb-2">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">内存使用率</h3>
          <div class="flex gap-4 text-xs text-gray-500 dark:text-gray-400">
            <span>当前: <strong class="text-green-600 dark:text-green-400">{getLatestValue(memoryData)}</strong></span>
            <span>平均: <strong>{getAverageValue(memoryData)}</strong></span>
          </div>
        </div>
        <svg width={chartWidth} height={chartHeight} class="w-full">
          <!-- 网格线 -->
          {#each gridLines as line}
            <line
              x1={padding.left}
              y1={line.y}
              x2={chartWidth - padding.right}
              y2={line.y}
              stroke="currentColor"
              class="text-gray-300 dark:text-gray-600"
              stroke-width="0.5"
              stroke-dasharray="4,4"
            />
            <text
              x={padding.left - 5}
              y={line.y + 4}
              text-anchor="end"
              class="text-gray-500 dark:text-gray-400"
              font-size="10"
            >
              {line.label}%
            </text>
          {/each}

          <!-- 数据线 -->
          {#if memoryData.length > 0}
            <path
              d={generatePath(memoryData)}
              fill="none"
              stroke="rgb(34, 197, 94)"
              stroke-width="2"
              class="transition-all"
            />
            <!-- 填充区域 -->
            <path
              d={generatePath(memoryData) + ` L ${chartWidth - padding.right},${chartHeight - padding.bottom} L ${padding.left},${chartHeight - padding.bottom} Z`}
              fill="rgb(34, 197, 94)"
              fill-opacity="0.1"
            />
          {/if}

          <!-- 时间轴标签 -->
          {#if memoryData.length > 0}
            <text
              x={padding.left}
              y={chartHeight - 5}
              class="text-gray-500 dark:text-gray-400"
              font-size="9"
            >
              {formatTime(memoryData[0].timestamp)}
            </text>
            <text
              x={chartWidth - padding.right}
              y={chartHeight - 5}
              text-anchor="end"
              class="text-gray-500 dark:text-gray-400"
              font-size="9"
            >
              {formatTime(memoryData[memoryData.length - 1].timestamp)}
            </text>
          {/if}
        </svg>
      </div>

      <!-- 磁盘使用率图表 -->
      <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
        <div class="flex items-center justify-between mb-2">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">磁盘使用率</h3>
          <div class="flex gap-4 text-xs text-gray-500 dark:text-gray-400">
            <span>当前: <strong class="text-purple-600 dark:text-purple-400">{getLatestValue(diskData)}</strong></span>
            <span>平均: <strong>{getAverageValue(diskData)}</strong></span>
          </div>
        </div>
        <svg width={chartWidth} height={chartHeight} class="w-full">
          <!-- 网格线 -->
          {#each gridLines as line}
            <line
              x1={padding.left}
              y1={line.y}
              x2={chartWidth - padding.right}
              y2={line.y}
              stroke="currentColor"
              class="text-gray-300 dark:text-gray-600"
              stroke-width="0.5"
              stroke-dasharray="4,4"
            />
            <text
              x={padding.left - 5}
              y={line.y + 4}
              text-anchor="end"
              class="text-gray-500 dark:text-gray-400"
              font-size="10"
            >
              {line.label}%
            </text>
          {/each}

          <!-- 数据线 -->
          {#if diskData.length > 0}
            <path
              d={generatePath(diskData)}
              fill="none"
              stroke="rgb(168, 85, 247)"
              stroke-width="2"
              class="transition-all"
            />
            <!-- 填充区域 -->
            <path
              d={generatePath(diskData) + ` L ${chartWidth - padding.right},${chartHeight - padding.bottom} L ${padding.left},${chartHeight - padding.bottom} Z`}
              fill="rgb(168, 85, 247)"
              fill-opacity="0.1"
            />
          {/if}

          <!-- 时间轴标签 -->
          {#if diskData.length > 0}
            <text
              x={padding.left}
              y={chartHeight - 5}
              class="text-gray-500 dark:text-gray-400"
              font-size="9"
            >
              {formatTime(diskData[0].timestamp)}
            </text>
            <text
              x={chartWidth - padding.right}
              y={chartHeight - 5}
              text-anchor="end"
              class="text-gray-500 dark:text-gray-400"
              font-size="9"
            >
              {formatTime(diskData[diskData.length - 1].timestamp)}
            </text>
          {/if}
        </svg>
      </div>
    </div>

    <!-- 图表说明 -->
    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
      <p class="text-xs text-gray-500 dark:text-gray-400 text-center">
        显示最近 10 分钟数据 | 每 30 秒自动刷新
      </p>
    </div>
  {/if}
</div>
