<script>
  import { formatBytes, formatPercent } from '../utils/format'

  let { diskInfo } = $props()
</script>

<div class="bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/20 dark:to-purple-800/20 rounded-lg p-4 shadow-md">
  <div class="flex items-center justify-between mb-3">
    <h3 class="text-lg font-semibold text-purple-900 dark:text-purple-100">
      💿 磁盘
    </h3>
    <span class="text-sm font-medium text-purple-600 dark:text-purple-400">
      {diskInfo.disk_count} 个设备
    </span>
  </div>

  <div class="space-y-3 text-sm max-h-60 overflow-y-auto">
    {#each diskInfo.disks as disk}
      <div class="bg-white/50 dark:bg-gray-800/50 rounded-md p-3">
        <div class="flex justify-between items-center mb-2">
          <span class="font-semibold text-purple-900 dark:text-purple-100 truncate max-w-[150px]" title={disk.mount_point}>
            {disk.mount_point || disk.name}
          </span>
          <span class="text-purple-600 dark:text-purple-400 font-bold">
            {formatPercent(disk.usage_percent)}
          </span>
        </div>

        <div class="space-y-1 text-xs">
          <div class="flex justify-between">
            <span class="text-gray-600 dark:text-gray-400">已用:</span>
            <span class="font-medium text-gray-900 dark:text-gray-100">
              {formatBytes(disk.used_space)}
            </span>
          </div>

          <div class="flex justify-between">
            <span class="text-gray-600 dark:text-gray-400">总计:</span>
            <span class="font-medium text-gray-900 dark:text-gray-100">
              {formatBytes(disk.total_space)}
            </span>
          </div>

          <!-- 磁盘使用率进度条 -->
          <div class="mt-2">
            <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1.5">
              <div
                class="bg-purple-600 dark:bg-purple-400 h-1.5 rounded-full transition-all duration-300"
                style="width: {disk.usage_percent}%"
              ></div>
            </div>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>
