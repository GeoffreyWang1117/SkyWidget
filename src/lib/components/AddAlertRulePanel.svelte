<script>
  import { invoke } from '@tauri-apps/api/core'

  let showDialog = $state(false)
  let name = $state('')
  let description = $state('')
  let conditionType = $state('cpu_usage')
  let threshold = $state(80)
  let severity = $state('Warning')
  let error = $state(null)
  let submitting = $state(false)

  const conditionTypes = [
    { value: 'cpu_usage', label: 'CPU 使用率 (%)' },
    { value: 'memory_usage', label: '内存使用率 (%)' },
    { value: 'disk_usage', label: '磁盘使用率 (%)' },
    { value: 'cpu_temperature', label: 'CPU 温度 (°C)' },
  ]

  const severityLevels = [
    { value: 'Info', label: 'ℹ️ 信息', color: 'blue' },
    { value: 'Warning', label: '⚠️ 警告', color: 'yellow' },
    { value: 'Error', label: '❌ 错误', color: 'orange' },
    { value: 'Critical', label: '🚨 严重', color: 'red' },
  ]

  async function handleSubmit() {
    if (!name || !description) {
      error = '请填写所有必填字段'
      return
    }

    if (threshold <= 0 || threshold > 100) {
      error = '阈值必须在 0-100 之间'
      return
    }

    submitting = true
    error = null

    try {
      await invoke('add_alert_rule', {
        name,
        description,
        conditionType,
        threshold,
        severity,
      })

      // 重置表单
      name = ''
      description = ''
      conditionType = 'cpu_usage'
      threshold = 80
      severity = 'Warning'
      showDialog = false

      // 触发刷新（通过事件或回调）
      window.location.reload() // 简单方案
    } catch (e) {
      error = e?.toString() || '添加告警规则失败'
      console.error('Failed to add alert rule:', e)
    } finally {
      submitting = false
    }
  }

  function resetForm() {
    name = ''
    description = ''
    conditionType = 'cpu_usage'
    threshold = 80
    severity = 'Warning'
    error = null
  }
</script>

<div>
  <button
    onclick={() => {
      showDialog = true
      resetForm()
    }}
    class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md font-medium shadow-sm transition-colors"
  >
    + 添加自定义规则
  </button>

  {#if showDialog}
    <div
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onclick={(e) => {
        if (e.target === e.currentTarget) showDialog = false
      }}
    >
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
        <h3 class="text-xl font-bold mb-4 text-gray-900 dark:text-gray-100">
          添加自定义告警规则
        </h3>

        {#if error}
          <div class="mb-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded">
            <p class="text-sm text-red-600 dark:text-red-400">{error}</p>
          </div>
        {/if}

        <form
          onsubmit={(e) => {
            e.preventDefault()
            handleSubmit()
          }}
          class="space-y-4"
        >
          <!-- 规则名称 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              规则名称 *
            </label>
            <input
              type="text"
              bind:value={name}
              placeholder="例如: 高 CPU 使用率告警"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
              required
            />
          </div>

          <!-- 描述 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              描述 *
            </label>
            <textarea
              bind:value={description}
              placeholder="例如: CPU 使用率超过阈值时触发"
              rows="2"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
              required
            ></textarea>
          </div>

          <!-- 监控指标 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              监控指标
            </label>
            <select
              bind:value={conditionType}
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
            >
              {#each conditionTypes as type}
                <option value={type.value}>{type.label}</option>
              {/each}
            </select>
          </div>

          <!-- 阈值 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              阈值
            </label>
            <div class="flex items-center gap-2">
              <input
                type="number"
                bind:value={threshold}
                min="0"
                max="100"
                step="1"
                class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
                required
              />
              <span class="text-sm text-gray-600 dark:text-gray-400">
                {conditionType === 'cpu_temperature' ? '°C' : '%'}
              </span>
            </div>
            <input
              type="range"
              bind:value={threshold}
              min="0"
              max="100"
              step="1"
              class="w-full mt-2"
            />
          </div>

          <!-- 严重级别 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              严重级别
            </label>
            <div class="grid grid-cols-2 gap-2">
              {#each severityLevels as level}
                <button
                  type="button"
                  onclick={() => (severity = level.value)}
                  class="px-3 py-2 rounded-md border {severity === level.value
                    ? `bg-${level.color}-100 border-${level.color}-500 text-${level.color}-800 dark:bg-${level.color}-900/30 dark:text-${level.color}-300`
                    : 'bg-gray-100 border-gray-300 text-gray-700 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-300'}"
                >
                  {level.label}
                </button>
              {/each}
            </div>
          </div>

          <!-- 按钮 -->
          <div class="flex items-center gap-2 pt-2">
            <button
              type="submit"
              disabled={submitting}
              class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md font-medium disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {submitting ? '添加中...' : '添加规则'}
            </button>
            <button
              type="button"
              onclick={() => (showDialog = false)}
              class="flex-1 px-4 py-2 bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-md font-medium"
            >
              取消
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
