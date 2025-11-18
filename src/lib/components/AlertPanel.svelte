<script>
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let alertRules = $state([])
  let loading = $state(true)
  let error = $state(null)
  let refreshInterval = null

  async function loadAlertRules() {
    try {
      alertRules = await invoke('get_alert_rules')
      error = null
    } catch (e) {
      error = e?.toString() || '获取告警规则失败'
      console.error('Failed to load alert rules:', e)
    } finally {
      loading = false
    }
  }

  async function toggleRule(ruleId, enabled) {
    try {
      await invoke('toggle_alert_rule', { ruleId, enabled })
      await loadAlertRules()
    } catch (e) {
      console.error('Failed to toggle alert rule:', e)
      error = e?.toString() || '切换告警规则失败'
    }
  }

  onMount(() => {
    loadAlertRules()
    // 每 10 秒刷新一次
    refreshInterval = setInterval(loadAlertRules, 10000)
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
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-gray-100">
    🔔 告警规则
  </h2>

  {#if loading}
    <div class="text-center py-4">
      <div class="inline-block animate-spin rounded-full h-8 w-8 border-4 border-blue-600 border-t-transparent"></div>
    </div>
  {:else if error}
    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded p-4">
      <p class="text-red-600 dark:text-red-400">{error}</p>
    </div>
  {:else}
    {#if alertRules.length === 0}
      <div class="text-center py-8 text-gray-500 dark:text-gray-400">
        <p>暂无告警规则</p>
      </div>
    {:else}
      <div class="space-y-3">
        {#each alertRules as rule}
          <div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-2">
                  <span class="text-lg">{getSeverityIcon(rule.severity)}</span>
                  <h3 class="font-semibold text-gray-900 dark:text-gray-100">
                    {rule.name}
                  </h3>
                  <span class={`text-xs px-2 py-1 rounded-full ${getSeverityColor(rule.severity)}`}>
                    {rule.severity}
                  </span>
                </div>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">
                  {rule.description}
                </p>
                <div class="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
                  <span>冷却时间: {rule.cooldown_seconds} 秒</span>
                  {#if rule.last_triggered}
                    <span>上次触发: {new Date(rule.last_triggered * 1000).toLocaleString('zh-CN')}</span>
                  {/if}
                </div>
              </div>
              <div class="ml-4">
                <label class="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={rule.enabled}
                    onchange={(e) => toggleRule(rule.id, e.target.checked)}
                    class="sr-only peer"
                  />
                  <div class="w-11 h-6 bg-gray-300 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-600 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
                </label>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
