<script lang="ts">
  /**
   * 动态布局容器
   * 根据用户配置动态渲染组件，支持拖拽排序
   */

  import { configStore } from '$lib/stores/config.svelte';
  import type { ComponentType, ComponentConfig } from '$lib/types/config';
  import { dndzone, TRIGGERS, SOURCES } from 'svelte-dnd-action';

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

  // 本地拖拽状态
  let items = $state<ComponentConfig[]>([]);
  let dragDisabled = $derived(!configStore.isEditMode);

  // 同步可见组件到本地状态
  $effect(() => {
    items = [...configStore.visibleComponents];
  });

  // 拖拽相关配置
  const flipDurationMs = 200;

  // 处理拖拽排序
  function handleDndConsider(e: CustomEvent<any>) {
    items = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent<any>) {
    items = e.detail.items;
    // 保存新顺序到配置
    configStore.reorderVisibleComponents(items);
  }

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
    const editClass = configStore.isEditMode ? 'edit-mode' : '';
    return `layout-container density-${density} ${editClass}`.trim();
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

<div
  class={containerClass()}
  style={gridStyle()}
  use:dndzone={{ items, flipDurationMs, dragDisabled, type: 'components' }}
  onconsider={handleDndConsider}
  onfinalize={handleDndFinalize}
>
  {#each items as component (component.id)}
    {@const Component = componentMap[component.type]}
    {#if Component}
      <div
        class="component-wrapper"
        class:collapsed={component.collapsed}
        class:draggable={configStore.isEditMode}
        style={getComponentStyle(component)}
      >
        {#if configStore.isEditMode}
          <div class="drag-handle" title="拖动排序">
            <span class="drag-icon">⋮⋮</span>
            <span class="component-label">{component.type.toUpperCase()}</span>
          </div>
        {/if}
        <div class="component-content" class:with-handle={configStore.isEditMode}>
          <Component />
        </div>
      </div>
    {:else}
      <!-- 组件未实现或不可用 -->
      <div
        class="component-placeholder"
        class:draggable={configStore.isEditMode}
        style={getComponentStyle(component)}
      >
        {#if configStore.isEditMode}
          <div class="drag-handle" title="拖动排序">
            <span class="drag-icon">⋮⋮</span>
            <span class="component-label">{component.type.toUpperCase()}</span>
          </div>
        {/if}
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
    position: relative;
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

  .component-wrapper,
  .component-placeholder {
    background: var(--bg-card, white);
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease;
    overflow: hidden;
    position: relative;
  }

  .component-wrapper {
    padding: 1rem;
  }

  .component-wrapper:hover {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .component-wrapper.collapsed {
    max-height: 60px;
    overflow: hidden;
  }

  /* 编辑模式样式 */
  .edit-mode .component-wrapper.draggable,
  .edit-mode .component-placeholder.draggable {
    cursor: grab;
    border: 2px dashed #cbd5e1;
  }

  .edit-mode .component-wrapper.draggable:active,
  .edit-mode .component-placeholder.draggable:active {
    cursor: grabbing;
    opacity: 0.5;
  }

  .edit-mode .component-wrapper.draggable:hover,
  .edit-mode .component-placeholder.draggable:hover {
    border-color: #3b82f6;
    box-shadow: 0 6px 12px rgba(59, 130, 246, 0.2);
  }

  /* 拖动手柄 */
  .drag-handle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    margin: -1rem -1rem 0.5rem -1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    font-weight: 600;
    font-size: 0.875rem;
    cursor: grab;
    user-select: none;
  }

  .drag-handle:active {
    cursor: grabbing;
  }

  .drag-icon {
    font-size: 1.25rem;
    line-height: 1;
    letter-spacing: -2px;
  }

  .component-label {
    flex: 1;
  }

  .component-content {
    position: relative;
  }

  .component-content.with-handle {
    padding-top: 0;
  }

  .component-placeholder {
    padding: 2rem;
    display: flex;
    flex-direction: column;
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

  /* 拖拽占位符样式 */
  :global(.dnd-action-dragged-el) {
    opacity: 0.5;
    transform: rotate(2deg);
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

  :global([data-theme='dark']) .edit-mode .component-wrapper.draggable,
  :global([data-theme='dark']) .edit-mode .component-placeholder.draggable {
    border-color: #475569;
  }

  :global([data-theme='dark']) .edit-mode .component-wrapper.draggable:hover,
  :global([data-theme='dark']) .edit-mode .component-placeholder.draggable:hover {
    border-color: #60a5fa;
  }

  :global([data-theme='dark']) .placeholder-text {
    color: #f9fafb;
  }

  :global([data-theme='dark']) .placeholder-hint {
    color: #9ca3af;
  }
</style>
