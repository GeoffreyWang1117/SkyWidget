<script>
  import { formatPercent, formatFrequency } from '../utils/format'

  let { cpuInfo } = $props()
</script>

<div class="bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 rounded-lg p-4 shadow-md">
  <div class="flex items-center justify-between mb-3">
    <h3 class="text-lg font-semibold text-blue-900 dark:text-blue-100">
      🖥️ CPU
    </h3>
    <span class="text-2xl font-bold text-blue-600 dark:text-blue-400">
      {formatPercent(cpuInfo.usage)}
    </span>
  </div>

  <div class="space-y-2 text-sm">
    <div class="flex justify-between">
      <span class="text-gray-600 dark:text-gray-400">型号:</span>
      <span class="font-medium text-gray-900 dark:text-gray-100 truncate max-w-[200px]" title={cpuInfo.brand}>
        {cpuInfo.brand}
      </span>
    </div>

    <div class="flex justify-between">
      <span class="text-gray-600 dark:text-gray-400">核心数:</span>
      <span class="font-medium text-gray-900 dark:text-gray-100">
        {cpuInfo.core_count}
      </span>
    </div>

    <div class="flex justify-between">
      <span class="text-gray-600 dark:text-gray-400">频率:</span>
      <span class="font-medium text-gray-900 dark:text-gray-100">
        {formatFrequency(cpuInfo.frequency)}
      </span>
    </div>

    <!-- CPU 使用率进度条 -->
    <div class="mt-3">
      <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
        <div
          class="bg-blue-600 dark:bg-blue-400 h-2 rounded-full transition-all duration-300"
          style="width: {cpuInfo.usage}%"
        ></div>
      </div>
    </div>
  </div>
</div>
