<script lang="ts">
  /**
   * 动态布局容器
   * 根据用户配置动态渲染组件
   */

  import { configStore } from '$lib/stores/config.svelte';
  import type { ComponentType, ComponentConfig } from '$lib/types/config';

  // 导入所有可用组件
  import CpuMonitor from './CpuMonitor.svelte';
  import MemoryMonitor from './MemoryMonitor.svelte';
  import DiskMonitor from './DiskMonitor.svelte';
  import TemperatureMonitor from './TemperatureMonitor.svelte';
  import GpuMonitor from './GpuMonitor.svelte';
  import FanMonitor from './FanMonitor.svelte';
  import AlertPanel from './AlertPanel.svelte';
  import MetricsChart from './MetricsChart.svelte';
  import NetworkPanel from './NetworkPanel.svelte';

  // 组件映射表
  const componentMap: Record<ComponentType, any> = {
    cpu: CpuMonitor,
    memory: MemoryMonitor,
    disk: DiskMonitor,
    temperature: TemperatureMonitor,
    gpu: GpuMonitor,
    fan: FanMonitor,
    power: null, // 待实现
    network: NetworkPanel,
    alerts: AlertPanel,
    chart: MetricsChart,
  };

  // 获取可见组件
  const visibleComponents = $derived(configStore.visibleComponents);

  // 计算网格样式
  const gridStyle = $derived(() => {
    const { columns, density } = configStore.config.layout;
    const gap = density === 'compact' ? '0.5rem' : density === 'comfortable' ? '1rem' : '1.5rem';
    return `
      display: grid;
      grid-template-columns: repeat(${columns}, 1fr);
      gap: ${gap};
      width: 100%;
    `;
  });

  // 计算容器类名
  const containerClass = $derived(() => {
    const { density } = configStore.config.layout;
    return `layout-container density-${density}`;
  });

  // 获取组件包装器样式
  function getComponentStyle(component: ComponentConfig): string {
    if (configStore.config.layout.enableGridLayout && component.gridPosition) {
      const { columnStart, columnEnd, rowStart, rowEnd } = component.gridPosition;
      return `
        grid-column: ${columnStart} / ${columnEnd};
        grid-row: ${rowStart} / ${rowEnd};
      `;
    }
    return '';
  }
</script>

<div class={containerClass()} style={gridStyle()}>
  {#each visibleComponents as component (component.id)}
    {@const Component = componentMap[component.type]}
    {#if Component}
      <div
        class="component-wrapper"
        class:collapsed={component.collapsed}
        style={getComponentStyle(component)}
      >
        <Component />
      </div>
    {:else}
      <!-- 组件未实现或不可用 -->
      <div class="component-placeholder" style={getComponentStyle(component)}>
        <div class="placeholder-content">
          <p class="placeholder-icon">⚠️</p>
          <p class="placeholder-text">{component.type.toUpperCase()}</p>
          <p class="placeholder-hint">组件待实现</p>
        </div>
      </div>
    {/if}
  {/each}
</div>

<style>
  .layout-container {
    padding: 1rem;
    min-height: 100vh;
    background: var(--bg-app, #f9fafb);
  }

  .density-compact {
    padding: 0.5rem;
  }

  .density-comfortable {
    padding: 1rem;
  }

  .density-spacious {
    padding: 1.5rem;
  }

  .component-wrapper {
    background: var(--bg-card, white);
    border-radius: 8px;
    padding: 1rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease;
    overflow: hidden;
  }

  .component-wrapper:hover {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .component-wrapper.collapsed {
    max-height: 60px;
    overflow: hidden;
  }

  .component-placeholder {
    background: var(--bg-card, white);
    border-radius: 8px;
    padding: 2rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
  }

  .placeholder-content {
    text-align: center;
  }

  .placeholder-icon {
    font-size: 3rem;
    margin: 0 0 0.5rem 0;
  }

  .placeholder-text {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary, #111827);
    margin: 0 0 0.25rem 0;
  }

  .placeholder-hint {
    font-size: 0.875rem;
    color: var(--text-secondary, #6b7280);
    margin: 0;
  }

  /* 响应式布局 */
  @media (max-width: 768px) {
    .layout-container {
      grid-template-columns: 1fr !important;
    }
  }

  /* 暗色主题支持 */
  :global([data-theme='dark']) .layout-container {
    background: #111827;
  }

  :global([data-theme='dark']) .component-wrapper,
  :global([data-theme='dark']) .component-placeholder {
    background: #1f2937;
  }

  :global([data-theme='dark']) .placeholder-text {
    color: #f9fafb;
  }

  :global([data-theme='dark']) .placeholder-hint {
    color: #9ca3af;
  }
</style>
