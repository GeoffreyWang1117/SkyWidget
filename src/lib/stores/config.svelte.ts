/**
 * 配置管理 Store (Svelte 5 Runes)
 * 使用 Tauri Store 插件进行持久化存储
 */

import { Store } from '@tauri-apps/plugin-store';
import type {
  UserConfig,
  ComponentConfig,
  LayoutPreset,
  ThemeMode,
  RefreshInterval,
  LayoutDensity,
  ComponentType,
} from '$lib/types/config';
import { DEFAULT_CONFIG, LAYOUT_PRESETS } from '$lib/types/config';

// Tauri Store 实例
let tauriStore: Store | null = null;

// 初始化 Tauri Store
async function initTauriStore(): Promise<Store> {
  if (!tauriStore) {
    tauriStore = new Store('skywidget-config.json');
  }
  return tauriStore;
}

/**
 * 配置状态类
 * 使用 Svelte 5 runes 实现响应式
 */
class ConfigState {
  // 当前配置（响应式）
  config = $state<UserConfig>(structuredClone(DEFAULT_CONFIG));

  // 是否正在加载
  loading = $state(false);

  // 是否有未保存的更改
  hasUnsavedChanges = $state(false);

  // 派生状态：可见组件列表
  visibleComponents = $derived(
    this.config.components
      .filter((c) => c.visible)
      .sort((a, b) => a.order - b.order)
  );

  // 派生状态：当前主题模式
  currentTheme = $derived(() => {
    const mode = this.config.theme.mode;
    if (mode === 'auto') {
      // 检测系统主题
      if (typeof window !== 'undefined') {
        return window.matchMedia('(prefers-color-scheme: dark)').matches
          ? 'dark'
          : 'light';
      }
      return 'dark';
    }
    return mode;
  });

  constructor() {
    // 初始化时加载配置
    this.load();
  }

  /**
   * 从持久化存储加载配置
   */
  async load(): Promise<void> {
    this.loading = true;
    try {
      const store = await initTauriStore();
      const savedConfig = await store.get<UserConfig>('config');

      if (savedConfig) {
        // 合并默认配置和保存的配置（防止新增字段丢失）
        this.config = this.mergeConfig(DEFAULT_CONFIG, savedConfig);
      } else {
        // 首次运行，使用默认配置
        this.config = structuredClone(DEFAULT_CONFIG);
        await this.save();
      }

      this.hasUnsavedChanges = false;
    } catch (error) {
      console.error('加载配置失败:', error);
      this.config = structuredClone(DEFAULT_CONFIG);
    } finally {
      this.loading = false;
    }
  }

  /**
   * 保存配置到持久化存储
   */
  async save(): Promise<void> {
    try {
      const store = await initTauriStore();

      // 更新最后修改时间
      this.config.lastUpdated = new Date().toISOString();

      await store.set('config', this.config);
      await store.save();

      this.hasUnsavedChanges = false;
      console.log('配置已保存');
    } catch (error) {
      console.error('保存配置失败:', error);
      throw error;
    }
  }

  /**
   * 合并配置（深度合并）
   */
  private mergeConfig(defaultConfig: UserConfig, savedConfig: Partial<UserConfig>): UserConfig {
    return {
      version: savedConfig.version ?? defaultConfig.version,
      layout: { ...defaultConfig.layout, ...savedConfig.layout },
      components: this.mergeComponents(defaultConfig.components, savedConfig.components ?? []),
      theme: { ...defaultConfig.theme, ...savedConfig.theme },
      performance: { ...defaultConfig.performance, ...savedConfig.performance },
      lastUpdated: savedConfig.lastUpdated,
    };
  }

  /**
   * 合并组件配置
   */
  private mergeComponents(
    defaultComponents: ComponentConfig[],
    savedComponents: ComponentConfig[]
  ): ComponentConfig[] {
    const componentMap = new Map(savedComponents.map((c) => [c.id, c]));

    return defaultComponents.map((defaultComp) => {
      const savedComp = componentMap.get(defaultComp.id);
      if (savedComp) {
        return {
          ...defaultComp,
          ...savedComp,
          settings: { ...defaultComp.settings, ...savedComp.settings },
        };
      }
      return defaultComp;
    });
  }

  /**
   * 更新配置
   */
  update(updater: (config: UserConfig) => UserConfig): void {
    this.config = updater(this.config);
    this.hasUnsavedChanges = true;
  }

  /**
   * 更新布局配置
   */
  updateLayout(updater: (layout: typeof this.config.layout) => typeof this.config.layout): void {
    this.update((config) => ({
      ...config,
      layout: updater(config.layout),
    }));
  }

  /**
   * 更新主题配置
   */
  updateTheme(updater: (theme: typeof this.config.theme) => typeof this.config.theme): void {
    this.update((config) => ({
      ...config,
      theme: updater(config.theme),
    }));
  }

  /**
   * 更新性能配置
   */
  updatePerformance(
    updater: (performance: typeof this.config.performance) => typeof this.config.performance
  ): void {
    this.update((config) => ({
      ...config,
      performance: updater(config.performance),
    }));
  }

  /**
   * 切换组件可见性
   */
  toggleComponentVisibility(componentId: string): void {
    this.update((config) => ({
      ...config,
      components: config.components.map((c) =>
        c.id === componentId ? { ...c, visible: !c.visible } : c
      ),
    }));
  }

  /**
   * 更新组件配置
   */
  updateComponent(
    componentId: string,
    updater: (component: ComponentConfig) => ComponentConfig
  ): void {
    this.update((config) => ({
      ...config,
      components: config.components.map((c) => (c.id === componentId ? updater(c) : c)),
    }));
  }

  /**
   * 设置主题模式
   */
  setThemeMode(mode: ThemeMode): void {
    this.updateTheme((theme) => ({ ...theme, mode }));
  }

  /**
   * 设置布局列数
   */
  setLayoutColumns(columns: number): void {
    this.updateLayout((layout) => ({ ...layout, columns: Math.max(1, Math.min(4, columns)) }));
  }

  /**
   * 设置布局密度
   */
  setLayoutDensity(density: LayoutDensity): void {
    this.updateLayout((layout) => ({ ...layout, density }));
  }

  /**
   * 设置全局刷新间隔
   */
  setGlobalRefreshInterval(interval: RefreshInterval): void {
    this.updatePerformance((perf) => ({ ...perf, globalRefreshInterval: interval }));
  }

  /**
   * 应用布局预设
   */
  applyPreset(preset: LayoutPreset): void {
    const presetConfig = LAYOUT_PRESETS[preset];
    if (!presetConfig) {
      console.error(`未知的布局预设: ${preset}`);
      return;
    }

    this.update((config) => ({
      ...config,
      layout: {
        ...config.layout,
        ...(presetConfig.layout ?? {}),
        preset,
      },
      components: presetConfig.components
        ? this.mergeComponents(config.components, presetConfig.components as ComponentConfig[])
        : config.components,
    }));

    console.log(`已应用布局预设: ${preset}`);
  }

  /**
   * 重置为默认配置
   */
  reset(): void {
    this.config = structuredClone(DEFAULT_CONFIG);
    this.hasUnsavedChanges = true;
  }

  /**
   * 导出配置为 JSON
   */
  exportConfig(): string {
    return JSON.stringify(this.config, null, 2);
  }

  /**
   * 从 JSON 导入配置
   */
  importConfig(jsonString: string): void {
    try {
      const importedConfig = JSON.parse(jsonString) as UserConfig;

      // 验证配置版本
      if (!importedConfig.version) {
        throw new Error('无效的配置文件：缺少版本号');
      }

      // 合并配置
      this.config = this.mergeConfig(DEFAULT_CONFIG, importedConfig);
      this.hasUnsavedChanges = true;

      console.log('配置导入成功');
    } catch (error) {
      console.error('配置导入失败:', error);
      throw error;
    }
  }

  /**
   * 获取组件配置
   */
  getComponent(componentId: string): ComponentConfig | undefined {
    return this.config.components.find((c) => c.id === componentId);
  }

  /**
   * 获取组件列表（按类型）
   */
  getComponentsByType(type: ComponentType): ComponentConfig[] {
    return this.config.components.filter((c) => c.type === type);
  }

  /**
   * 移动组件顺序
   */
  moveComponent(componentId: string, direction: 'up' | 'down'): void {
    const components = [...this.config.components];
    const index = components.findIndex((c) => c.id === componentId);

    if (index === -1) return;

    const targetIndex = direction === 'up' ? index - 1 : index + 1;

    if (targetIndex < 0 || targetIndex >= components.length) return;

    // 交换顺序
    const temp = components[index].order;
    components[index].order = components[targetIndex].order;
    components[targetIndex].order = temp;

    this.update((config) => ({ ...config, components }));
  }

  /**
   * 切换动画
   */
  toggleAnimations(): void {
    this.updateTheme((theme) => ({
      ...theme,
      enableAnimations: !theme.enableAnimations,
    }));
  }

  /**
   * 切换阴影
   */
  toggleShadows(): void {
    this.updateTheme((theme) => ({
      ...theme,
      enableShadows: !theme.enableShadows,
    }));
  }

  /**
   * 切换硬件加速
   */
  toggleHardwareAcceleration(): void {
    this.updatePerformance((perf) => ({
      ...perf,
      enableHardwareAcceleration: !perf.enableHardwareAcceleration,
    }));
  }
}

// 导出单例实例
export const configStore = new ConfigState();
