<script>
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let localNode = $state(null)
  let discoveredNodes = $state([])
  let loading = $state(true)
  let error = $state(null)
  let refreshInterval = null

  async function loadNetworkInfo() {
    try {
      localNode = await invoke('get_local_node_info')
      discoveredNodes = await invoke('get_discovered_nodes')
      error = null
    } catch (e) {
      error = e?.toString() || '获取网络信息失败'
      console.error('Failed to load network info:', e)
    } finally {
      loading = false
    }
  }

  onMount(() => {
    loadNetworkInfo()
    // 每 5 秒刷新一次
    refreshInterval = setInterval(loadNetworkInfo, 5000)
  })

  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })

  function getStatusColor(status) {
    switch (status) {
      case 'Online':
        return 'bg-green-500'
      case 'Offline':
        return 'bg-gray-500'
      case 'Alerting':
        return 'bg-red-500'
      default:
        return 'bg-gray-500'
    }
  }
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-gray-100">
    🌐 网络拓扑
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
    <!-- 本地节点信息 -->
    {#if localNode}
      <div class="mb-6 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
        <h3 class="text-sm font-semibold text-blue-900 dark:text-blue-300 mb-2">
          🖥️ 本地节点（当前设备）
        </h3>
        <div class="grid grid-cols-2 gap-2 text-sm">
          <div>
            <span class="text-gray-600 dark:text-gray-400">节点名称:</span>
            <span class="ml-2 font-medium text-gray-900 dark:text-gray-100">{localNode.name}</span>
          </div>
          <div>
            <span class="text-gray-600 dark:text-gray-400">IP 地址:</span>
            <span class="ml-2 font-mono text-gray-900 dark:text-gray-100">{localNode.ip_address}</span>
          </div>
          <div>
            <span class="text-gray-600 dark:text-gray-400">API 端口:</span>
            <span class="ml-2 font-mono text-gray-900 dark:text-gray-100">{localNode.api_port}</span>
          </div>
          <div>
            <span class="text-gray-600 dark:text-gray-400">系统:</span>
            <span class="ml-2 text-gray-900 dark:text-gray-100">{localNode.os_info}</span>
          </div>
        </div>
      </div>
    {/if}

    <!-- 已发现的远程节点 -->
    <div>
      <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
        📡 已发现的远程节点 ({discoveredNodes.length})
      </h3>

      {#if discoveredNodes.length === 0}
        <div class="text-center py-8 text-gray-500 dark:text-gray-400">
          <p>暂未发现其他节点</p>
          <p class="text-xs mt-2">在同一局域网中运行 SkyWidget 即可自动发现</p>
        </div>
      {:else}
        <div class="space-y-3">
          {#each discoveredNodes as node}
            <div class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center gap-2">
                  <div class={`w-2 h-2 rounded-full ${getStatusColor(node.status)}`}></div>
                  <span class="font-medium text-gray-900 dark:text-gray-100">{node.name}</span>
                </div>
                <span class="text-xs px-2 py-1 bg-gray-200 dark:bg-gray-600 rounded text-gray-700 dark:text-gray-300">
                  v{node.version}
                </span>
              </div>
              <div class="grid grid-cols-2 gap-2 text-xs text-gray-600 dark:text-gray-400">
                <div>
                  <span>IP:</span>
                  <span class="ml-1 font-mono">{node.ip_address}</span>
                </div>
                <div>
                  <span>端口:</span>
                  <span class="ml-1 font-mono">{node.api_port}</span>
                </div>
                <div class="col-span-2">
                  <span>系统:</span>
                  <span class="ml-1">{node.os_info}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
