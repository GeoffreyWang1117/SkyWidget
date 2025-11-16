<script>
  import { formatBytes, formatPercent } from '../utils/format'

  let { memoryInfo } = $props()
</script>

<div class="bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-800/20 rounded-lg p-4 shadow-md">
  <div class="flex items-center justify-between mb-3">
    <h3 class="text-lg font-semibold text-green-900 dark:text-green-100">
      💾 内存
    </h3>
    <span class="text-2xl font-bold text-green-600 dark:text-green-400">
      {formatPercent(memoryInfo.usage_percent)}
    </span>
  </div>

  <div class="space-y-2 text-sm">
    <div class="flex justify-between">
      <span class="text-gray-600 dark:text-gray-400">已用:</span>
      <span class="font-medium text-gray-900 dark:text-gray-100">
        {formatBytes(memoryInfo.used)}
      </span>
    </div>

    <div class="flex justify-between">
      <span class="text-gray-600 dark:text-gray-400">可用:</span>
      <span class="font-medium text-gray-900 dark:text-gray-100">
        {formatBytes(memoryInfo.available)}
      </span>
    </div>

    <div class="flex justify-between">
      <span class="text-gray-600 dark:text-gray-400">总计:</span>
      <span class="font-medium text-gray-900 dark:text-gray-100">
        {formatBytes(memoryInfo.total)}
      </span>
    </div>

    <!-- 内存使用率进度条 -->
    <div class="mt-3">
      <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
        <div
          class="bg-green-600 dark:bg-green-400 h-2 rounded-full transition-all duration-300"
          style="width: {memoryInfo.usage_percent}%"
        ></div>
      </div>
    </div>

    {#if memoryInfo.swap_total > 0}
      <div class="pt-2 border-t border-gray-300 dark:border-gray-600 mt-2">
        <div class="flex justify-between text-xs">
          <span class="text-gray-500 dark:text-gray-400">交换分区:</span>
          <span class="font-medium text-gray-700 dark:text-gray-300">
            {formatBytes(memoryInfo.swap_used)} / {formatBytes(memoryInfo.swap_total)}
          </span>
        </div>
      </div>
    {/if}
  </div>
</div>
