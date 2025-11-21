<script lang="ts">
  /**
   * 设置面板组件
   * 提供用户配置界面
   */

  import { configStore } from '$lib/stores/config.svelte';
  import type { LayoutPreset, ThemeMode, LayoutDensity, RefreshInterval } from '$lib/types/config';

  // 控制面板显示状态
  let { isOpen = $bindable(false) } = $props();

  // 配置项
  const layoutPresets: { value: LayoutPreset; label: string }[] = [
    { value: 'default', label: '默认布局' },
    { value: 'minimal', label: '最小化' },
    { value: 'detailed', label: '详细信息' },
    { value: 'server', label: '服务器监控' },
    { value: 'gaming', label: '游戏模式' },
  ];

  const themeModes: { value: ThemeMode; label: string }[] = [
    { value: 'light', label: '浅色' },
    { value: 'dark', label: '深色' },
    { value: 'auto', label: '跟随系统' },
  ];

  const densities: { value: LayoutDensity; label: string }[] = [
    { value: 'compact', label: '紧凑' },
    { value: 'comfortable', label: '舒适' },
    { value: 'spacious', label: '宽松' },
  ];

  const refreshIntervals: { value: RefreshInterval; label: string }[] = [
    { value: 500, label: '0.5 秒' },
    { value: 1000, label: '1 秒' },
    { value: 2000, label: '2 秒' },
    { value: 5000, label: '5 秒' },
    { value: 10000, label: '10 秒' },
  ];

  // 当前选中的标签页
  let activeTab = $state<'layout' | 'components' | 'theme' | 'performance'>('layout');

  // 处理保存
  async function handleSave() {
    try {
      await configStore.save();
      alert('配置已保存！');
    } catch (error) {
      console.error('保存失败:', error);
      alert('保存失败，请重试');
    }
  }

  // 处理重置
  function handleReset() {
    if (confirm('确定要重置为默认配置吗？所有自定义设置将丢失。')) {
      configStore.reset();
    }
  }

  // 处理导出
  function handleExport() {
    try {
      const json = configStore.exportConfig();
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `skywidget-config-${new Date().toISOString().split('T')[0]}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('导出失败:', error);
      alert('导出失败，请重试');
    }
  }

  // 处理导入
  function handleImport() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;

      try {
        const text = await file.text();
        configStore.importConfig(text);
        alert('配置导入成功！');
      } catch (error) {
        console.error('导入失败:', error);
        alert('导入失败：' + (error as Error).message);
      }
    };
    input.click();
  }
</script>

{#if isOpen}
  <div class="settings-overlay" onclick={() => (isOpen = false)}>
    <div class="settings-panel" onclick={(e) => e.stopPropagation()}>
      <!-- 头部 -->
      <div class="settings-header">
        <h2>设置</h2>
        <button class="close-btn" onclick={() => (isOpen = false)}>✕</button>
      </div>

      <!-- 标签页导航 -->
      <div class="tabs">
        <button
          class="tab"
          class:active={activeTab === 'layout'}
          onclick={() => (activeTab = 'layout')}
        >
          布局
        </button>
        <button
          class="tab"
          class:active={activeTab === 'components'}
          onclick={() => (activeTab = 'components')}
        >
          组件
        </button>
        <button
          class="tab"
          class:active={activeTab === 'theme'}
          onclick={() => (activeTab = 'theme')}
        >
          主题
        </button>
        <button
          class="tab"
          class:active={activeTab === 'performance'}
          onclick={() => (activeTab = 'performance')}
        >
          性能
        </button>
      </div>

      <!-- 内容区域 -->
      <div class="settings-content">
        {#if activeTab === 'layout'}
          <!-- 布局设置 -->
          <div class="settings-section">
            <h3>布局预设</h3>
            <div class="preset-grid">
              {#each layoutPresets as preset}
                <button
                  class="preset-btn"
                  class:active={configStore.config.layout.preset === preset.value}
                  onclick={() => configStore.applyPreset(preset.value)}
                >
                  {preset.label}
                </button>
              {/each}
            </div>
          </div>

          <div class="settings-section">
            <h3>列数</h3>
            <div class="slider-container">
              <input
                type="range"
                min="1"
                max="4"
                bind:value={configStore.config.layout.columns}
                oninput={(e) => configStore.setLayoutColumns(Number(e.currentTarget.value))}
              />
              <span class="slider-value">{configStore.config.layout.columns}</span>
            </div>
          </div>

          <div class="settings-section">
            <h3>布局密度</h3>
            <div class="radio-group">
              {#each densities as density}
                <label class="radio-label">
                  <input
                    type="radio"
                    name="density"
                    value={density.value}
                    checked={configStore.config.layout.density === density.value}
                    onchange={() => configStore.setLayoutDensity(density.value)}
                  />
                  <span>{density.label}</span>
                </label>
              {/each}
            </div>
          </div>
        {/if}

        {#if activeTab === 'components'}
          <!-- 组件可见性设置 -->
          <div class="settings-section">
            <h3>显示组件</h3>
            <div class="component-list">
              {#each configStore.config.components as component}
                <label class="component-item">
                  <input
                    type="checkbox"
                    checked={component.visible}
                    onchange={() => configStore.toggleComponentVisibility(component.id)}
                  />
                  <span class="component-name">{component.type.toUpperCase()}</span>
                  <span class="component-interval">({component.refreshInterval}ms)</span>
                </label>
              {/each}
            </div>
          </div>
        {/if}

        {#if activeTab === 'theme'}
          <!-- 主题设置 -->
          <div class="settings-section">
            <h3>主题模式</h3>
            <div class="radio-group">
              {#each themeModes as mode}
                <label class="radio-label">
                  <input
                    type="radio"
                    name="theme"
                    value={mode.value}
                    checked={configStore.config.theme.mode === mode.value}
                    onchange={() => configStore.setThemeMode(mode.value)}
                  />
                  <span>{mode.label}</span>
                </label>
              {/each}
            </div>
          </div>

          <div class="settings-section">
            <h3>视觉效果</h3>
            <label class="toggle-label">
              <input
                type="checkbox"
                checked={configStore.config.theme.enableAnimations}
                onchange={() => configStore.toggleAnimations()}
              />
              <span>启用动画</span>
            </label>
            <label class="toggle-label">
              <input
                type="checkbox"
                checked={configStore.config.theme.enableShadows}
                onchange={() => configStore.toggleShadows()}
              />
              <span>启用阴影</span>
            </label>
          </div>
        {/if}

        {#if activeTab === 'performance'}
          <!-- 性能设置 -->
          <div class="settings-section">
            <h3>全局刷新间隔</h3>
            <div class="radio-group">
              {#each refreshIntervals as interval}
                <label class="radio-label">
                  <input
                    type="radio"
                    name="refresh"
                    value={interval.value}
                    checked={configStore.config.performance.globalRefreshInterval === interval.value}
                    onchange={() => configStore.setGlobalRefreshInterval(interval.value)}
                  />
                  <span>{interval.label}</span>
                </label>
              {/each}
            </div>
          </div>

          <div class="settings-section">
            <h3>优化选项</h3>
            <label class="toggle-label">
              <input
                type="checkbox"
                checked={configStore.config.performance.enableHardwareAcceleration}
                onchange={() => configStore.toggleHardwareAcceleration()}
              />
              <span>硬件加速</span>
            </label>
            <label class="toggle-label">
              <input
                type="checkbox"
                checked={configStore.config.performance.reduceWhenInactive}
                onchange={() =>
                  configStore.updatePerformance((p) => ({
                    ...p,
                    reduceWhenInactive: !p.reduceWhenInactive,
                  }))}
              />
              <span>后台时降低更新频率</span>
            </label>
          </div>
        {/if}
      </div>

      <!-- 底部操作栏 -->
      <div class="settings-footer">
        <div class="footer-left">
          <button class="btn btn-secondary" onclick={handleImport}>导入配置</button>
          <button class="btn btn-secondary" onclick={handleExport}>导出配置</button>
          <button class="btn btn-danger" onclick={handleReset}>重置</button>
        </div>
        <div class="footer-right">
          {#if configStore.hasUnsavedChanges}
            <span class="unsaved-indicator">● 有未保存的更改</span>
          {/if}
          <button class="btn btn-primary" onclick={handleSave}>保存</button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .settings-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .settings-panel {
    background: var(--bg-primary, white);
    border-radius: 12px;
    width: 90%;
    max-width: 700px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color, #e5e7eb);
  }

  .settings-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-primary, #111827);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: var(--text-secondary, #6b7280);
    padding: 0.25rem;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .close-btn:hover {
    background: var(--bg-hover, #f3f4f6);
  }

  .tabs {
    display: flex;
    border-bottom: 1px solid var(--border-color, #e5e7eb);
    padding: 0 1.5rem;
  }

  .tab {
    padding: 0.75rem 1.5rem;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-weight: 500;
    color: var(--text-secondary, #6b7280);
    transition: all 0.2s;
  }

  .tab:hover {
    color: var(--text-primary, #111827);
  }

  .tab.active {
    color: var(--primary, #3b82f6);
    border-bottom-color: var(--primary, #3b82f6);
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .settings-section {
    margin-bottom: 2rem;
  }

  .settings-section h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary, #111827);
  }

  .preset-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 0.75rem;
  }

  .preset-btn {
    padding: 0.75rem 1rem;
    border: 2px solid var(--border-color, #e5e7eb);
    border-radius: 8px;
    background: var(--bg-secondary, #f9fafb);
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }

  .preset-btn:hover {
    border-color: var(--primary, #3b82f6);
  }

  .preset-btn.active {
    border-color: var(--primary, #3b82f6);
    background: var(--primary-light, #eff6ff);
    color: var(--primary, #3b82f6);
  }

  .slider-container {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .slider-container input[type='range'] {
    flex: 1;
    height: 6px;
    border-radius: 3px;
    background: var(--border-color, #e5e7eb);
    outline: none;
  }

  .slider-value {
    font-weight: 600;
    color: var(--primary, #3b82f6);
    min-width: 2rem;
    text-align: center;
  }

  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .component-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .component-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    border-radius: 8px;
    cursor: pointer;
  }

  .component-item:hover {
    background: var(--bg-hover, #f3f4f6);
  }

  .component-name {
    font-weight: 500;
    flex: 1;
  }

  .component-interval {
    font-size: 0.875rem;
    color: var(--text-secondary, #6b7280);
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 0;
    cursor: pointer;
  }

  .settings-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-top: 1px solid var(--border-color, #e5e7eb);
  }

  .footer-left,
  .footer-right {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary {
    background: var(--primary, #3b82f6);
    color: white;
  }

  .btn-primary:hover {
    background: var(--primary-dark, #2563eb);
  }

  .btn-secondary {
    background: var(--bg-secondary, #f3f4f6);
    color: var(--text-primary, #111827);
  }

  .btn-secondary:hover {
    background: var(--bg-hover, #e5e7eb);
  }

  .btn-danger {
    background: #ef4444;
    color: white;
  }

  .btn-danger:hover {
    background: #dc2626;
  }

  .unsaved-indicator {
    color: #f59e0b;
    font-size: 0.875rem;
    font-weight: 500;
  }
</style>
