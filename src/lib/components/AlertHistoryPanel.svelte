<script>
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let alertHistory = $state([])
  let loading = $state(true)
  let error = $state(null)
  let refreshInterval = null
  let showAcknowledgedOnly = $state(false)

  async function loadAlertHistory() {
    try {
      alertHistory = await invoke('get_alert_history')
      error = null
    } catch (e) {
      error = e?.toString() || '获取告警历史失败'
      console.error('Failed to load alert history:', e)
    } finally {
      loading = false
    }
  }

  async function acknowledgeAlert(recordId) {
    try {
      await invoke('acknowledge_alert', { recordId })
      await loadAlertHistory()
    } catch (e) {
      console.error('Failed to acknowledge alert:', e)
      error = e?.toString() || '确认告警失败'
    }
  }

  async function clearHistory() {
    if (!confirm('确定要清空所有告警历史吗？此操作不可撤销。')) return

    try {
      await invoke('clear_alert_history')
      await loadAlertHistory()
    } catch (e) {
      console.error('Failed to clear alert history:', e)
      error = e?.toString() || '清空告警历史失败'
    }
  }

  async function exportHistory() {
    try {
      const json = await invoke('export_alert_history')
      const blob = new Blob([json], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `alert-history-${new Date().toISOString()}.json`
      a.click()
      URL.revokeObjectURL(url)
    } catch (e) {
      console.error('Failed to export alert history:', e)
      error = e?.toString() || '导出告警历史失败'
    }
  }

  onMount(() => {
    loadAlertHistory()
    refreshInterval = setInterval(loadAlertHistory, 15000) // 每 15 秒刷新
  })

  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })

  function getSeverityColor(severity) {
    switch (severity) {
      case 'Info':
        return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300'
      case 'Warning':
        return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-300'
      case 'Error':
        return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300'
      case 'Critical':
        return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300'
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300'
    }
  }

  function getSeverityIcon(severity) {
    switch (severity) {
      case 'Info':
        return 'ℹ️'
      case 'Warning':
        return '⚠️'
      case 'Error':
        return '❌'
      case 'Critical':
        return '🚨'
      default:
        return '📋'
    }
  }

  $: filteredHistory = showAcknowledgedOnly
    ? alertHistory.filter((r) => !r.acknowledged)
    : alertHistory
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <div class="flex items-center justify-between mb-4">
    <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">📜 告警历史</h2>
    <div class="flex items-center gap-2">
      <button
        onclick={() => (showAcknowledgedOnly = !showAcknowledgedOnly)}
        class="text-xs px-3 py-1 rounded {showAcknowledgedOnly
          ? 'bg-blue-600 text-white'
          : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'}"
      >
        {showAcknowledgedOnly ? '显示全部' : '仅未确认'}
      </button>
      <button
        onclick={exportHistory}
        class="text-xs px-3 py-1 bg-green-600 hover:bg-green-700 text-white rounded"
      >
        导出
      </button>
      <button
        onclick={clearHistory}
        class="text-xs px-3 py-1 bg-red-600 hover:bg-red-700 text-white rounded"
      >
        清空
      </button>
    </div>
  </div>

  {#if loading}
    <div class="text-center py-4">
      <div
        class="inline-block animate-spin rounded-full h-8 w-8 border-4 border-blue-600 border-t-transparent"
      ></div>
    </div>
  {:else if error}
    <div
      class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded p-4"
    >
      <p class="text-red-600 dark:text-red-400">{error}</p>
    </div>
  {:else}
    {#if filteredHistory.length === 0}
      <div class="text-center py-8 text-gray-500 dark:text-gray-400">
        <p>暂无告警记录</p>
      </div>
    {:else}
      <div class="space-y-2 max-h-[400px] overflow-y-auto">
        {#each filteredHistory as record}
          <div
            class="p-3 rounded-lg border {record.acknowledged
              ? 'bg-gray-50 dark:bg-gray-700/50 border-gray-200 dark:border-gray-600'
              : 'bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-500'}"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-1">
                  <span class="text-lg">{getSeverityIcon(record.severity)}</span>
                  <span class="font-semibold text-gray-900 dark:text-gray-100">
                    {record.rule_name}
                  </span>
                  <span class={`text-xs px-2 py-1 rounded-full ${getSeverityColor(record.severity)}`}>
                    {record.severity}
                  </span>
                  {#if record.acknowledged}
                    <span class="text-xs px-2 py-1 bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300 rounded-full">
                      ✓ 已确认
                    </span>
                  {/if}
                </div>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-1">{record.message}</p>
                <p class="text-xs text-gray-500 dark:text-gray-500">
                  {new Date(record.timestamp).toLocaleString('zh-CN')}
                </p>
              </div>
              {#if !record.acknowledged}
                <button
                  onclick={() => acknowledgeAlert(record.id)}
                  class="ml-2 text-xs px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded"
                >
                  确认
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
