/**
 * 用户配置类型定义
 * 用于 SkyWidget 监控面板的自定义配置
 */

/**
 * 组件类型
 */
export type ComponentType =
  | 'cpu'           // CPU 监控
  | 'memory'        // 内存监控
  | 'disk'          // 磁盘监控
  | 'temperature'   // 温度监控
  | 'gpu'           // GPU 监控
  | 'fan'           // 风扇监控
  | 'power'         // 电源/电压监控
  | 'network'       // 网络监控（未来）
  | 'alerts'        // 告警面板
  | 'chart';        // 性能图表

/**
 * 主题类型
 */
export type ThemeMode = 'light' | 'dark' | 'auto';

/**
 * 布局预设类型
 */
export type LayoutPreset =
  | 'default'       // 默认布局（所有组件）
  | 'minimal'       // 最小化（仅 CPU/内存/磁盘）
  | 'detailed'      // 详细（所有组件 + 图表）
  | 'server'        // 服务器（磁盘/温度/告警）
  | 'gaming'        // 游戏（CPU/GPU/温度/图表）
  | 'custom';       // 自定义布局

/**
 * 数据更新频率（毫秒）
 */
export type RefreshInterval = 500 | 1000 | 2000 | 5000 | 10000;

/**
 * 布局密度
 */
export type LayoutDensity = 'compact' | 'comfortable' | 'spacious';

/**
 * 网格位置（用于高级布局）
 */
export interface GridPosition {
  /** 起始列（从 1 开始） */
  columnStart: number;
  /** 结束列（从 1 开始，不包含） */
  columnEnd: number;
  /** 起始行（从 1 开始） */
  rowStart: number;
  /** 结束行（从 1 开始，不包含） */
  rowEnd: number;
}

/**
 * 组件特定设置
 */
export interface ComponentSettings {
  /** CPU 监控设置 */
  cpu?: {
    showCoreDetails: boolean;
    showFrequency: boolean;
    showTemperature: boolean;
  };

  /** 内存监控设置 */
  memory?: {
    showSwap: boolean;
    showTemperature: boolean;
    showErrors: boolean;
  };

  /** 磁盘监控设置 */
  disk?: {
    showAllDisks: boolean;
    showTemperature: boolean;
    showSmartInfo: boolean;
  };

  /** 温度监控设置 */
  temperature?: {
    showAllSensors: boolean;
    temperatureUnit: 'celsius' | 'fahrenheit';
  };

  /** GPU 监控设置 */
  gpu?: {
    showMemoryUsage: boolean;
    showTemperature: boolean;
    showPowerDraw: boolean;
  };

  /** 风扇监控设置 */
  fan?: {
    showAllFans: boolean;
    showRpmGraph: boolean;
  };

  /** 电源监控设置 */
  power?: {
    showAllVoltages: boolean;
    highlightAbnormal: boolean;
  };

  /** 告警面板设置 */
  alerts?: {
    maxDisplayCount: number;
    autoHideResolved: boolean;
    showTimestamp: boolean;
  };

  /** 图表设置 */
  chart?: {
    historyDuration: number; // 秒
    showLegend: boolean;
    metrics: string[]; // 要显示的指标列表
  };
}

/**
 * 单个组件配置
 */
export interface ComponentConfig {
  /** 组件唯一 ID */
  id: string;

  /** 组件类型 */
  type: ComponentType;

  /** 是否可见 */
  visible: boolean;

  /** 显示顺序（越小越靠前） */
  order: number;

  /** 刷新间隔（毫秒） */
  refreshInterval: RefreshInterval;

  /** 组件特定设置 */
  settings: ComponentSettings;

  /** 是否折叠 */
  collapsed: boolean;

  /** 网格位置（高级布局使用） */
  gridPosition?: GridPosition;
}

/**
 * 布局配置
 */
export interface LayoutConfig {
  /** 布局预设 */
  preset: LayoutPreset;

  /** 列数（1-4） */
  columns: number;

  /** 布局密度 */
  density: LayoutDensity;

  /** 是否启用网格布局（高级） */
  enableGridLayout: boolean;

  /** 网格行数（高级布局使用） */
  gridRows?: number;

  /** 网格列数（高级布局使用） */
  gridColumns?: number;
}

/**
 * 主题配置
 */
export interface ThemeConfig {
  /** 主题模式 */
  mode: ThemeMode;

  /** 自定义颜色（可选） */
  customColors?: {
    primary?: string;
    secondary?: string;
    success?: string;
    warning?: string;
    error?: string;
    background?: string;
    surface?: string;
    text?: string;
  };

  /** 是否启用动画 */
  enableAnimations: boolean;

  /** 是否启用阴影 */
  enableShadows: boolean;
}

/**
 * 性能配置
 */
export interface PerformanceConfig {
  /** 全局刷新间隔（毫秒） */
  globalRefreshInterval: RefreshInterval;

  /** 是否启用硬件加速 */
  enableHardwareAcceleration: boolean;

  /** 图表历史数据点数 */
  chartHistorySize: number;

  /** 是否在后台时降低更新频率 */
  reduceWhenInactive: boolean;
}

/**
 * 完整用户配置
 */
export interface UserConfig {
  /** 配置版本号 */
  version: string;

  /** 布局配置 */
  layout: LayoutConfig;

  /** 组件配置列表 */
  components: ComponentConfig[];

  /** 主题配置 */
  theme: ThemeConfig;

  /** 性能配置 */
  performance: PerformanceConfig;

  /** 配置最后更新时间 */
  lastUpdated?: string;
}

/**
 * 默认配置
 */
export const DEFAULT_CONFIG: UserConfig = {
  version: '1.0.0',

  layout: {
    preset: 'default',
    columns: 2,
    density: 'comfortable',
    enableGridLayout: false,
  },

  components: [
    {
      id: 'cpu-monitor',
      type: 'cpu',
      visible: true,
      order: 1,
      refreshInterval: 1000,
      collapsed: false,
      settings: {
        cpu: {
          showCoreDetails: true,
          showFrequency: true,
          showTemperature: true,
        },
      },
    },
    {
      id: 'memory-monitor',
      type: 'memory',
      visible: true,
      order: 2,
      refreshInterval: 1000,
      collapsed: false,
      settings: {
        memory: {
          showSwap: true,
          showTemperature: true,
          showErrors: true,
        },
      },
    },
    {
      id: 'disk-monitor',
      type: 'disk',
      visible: true,
      order: 3,
      refreshInterval: 5000,
      collapsed: false,
      settings: {
        disk: {
          showAllDisks: true,
          showTemperature: true,
          showSmartInfo: true,
        },
      },
    },
    {
      id: 'temperature-monitor',
      type: 'temperature',
      visible: true,
      order: 4,
      refreshInterval: 2000,
      collapsed: false,
      settings: {
        temperature: {
          showAllSensors: true,
          temperatureUnit: 'celsius',
        },
      },
    },
    {
      id: 'gpu-monitor',
      type: 'gpu',
      visible: true,
      order: 5,
      refreshInterval: 1000,
      collapsed: false,
      settings: {
        gpu: {
          showMemoryUsage: true,
          showTemperature: true,
          showPowerDraw: true,
        },
      },
    },
    {
      id: 'fan-monitor',
      type: 'fan',
      visible: true,
      order: 6,
      refreshInterval: 2000,
      collapsed: false,
      settings: {
        fan: {
          showAllFans: true,
          showRpmGraph: false,
        },
      },
    },
    {
      id: 'power-monitor',
      type: 'power',
      visible: true,
      order: 7,
      refreshInterval: 2000,
      collapsed: false,
      settings: {
        power: {
          showAllVoltages: true,
          highlightAbnormal: true,
        },
      },
    },
    {
      id: 'alerts-panel',
      type: 'alerts',
      visible: true,
      order: 8,
      refreshInterval: 1000,
      collapsed: false,
      settings: {
        alerts: {
          maxDisplayCount: 10,
          autoHideResolved: false,
          showTimestamp: true,
        },
      },
    },
    {
      id: 'performance-chart',
      type: 'chart',
      visible: true,
      order: 9,
      refreshInterval: 1000,
      collapsed: false,
      settings: {
        chart: {
          historyDuration: 60,
          showLegend: true,
          metrics: ['cpu_usage', 'memory_usage'],
        },
      },
    },
  ],

  theme: {
    mode: 'dark',
    enableAnimations: true,
    enableShadows: true,
  },

  performance: {
    globalRefreshInterval: 1000,
    enableHardwareAcceleration: true,
    chartHistorySize: 60,
    reduceWhenInactive: true,
  },
};

/**
 * 布局预设配置
 */
export const LAYOUT_PRESETS: Record<LayoutPreset, Partial<UserConfig>> = {
  default: {
    layout: {
      preset: 'default',
      columns: 2,
      density: 'comfortable',
      enableGridLayout: false,
    },
    components: DEFAULT_CONFIG.components.map(c => ({ ...c, visible: true })),
  },

  minimal: {
    layout: {
      preset: 'minimal',
      columns: 3,
      density: 'compact',
      enableGridLayout: false,
    },
    components: DEFAULT_CONFIG.components.map(c => ({
      ...c,
      visible: ['cpu', 'memory', 'disk'].includes(c.type),
    })),
  },

  detailed: {
    layout: {
      preset: 'detailed',
      columns: 2,
      density: 'spacious',
      enableGridLayout: false,
    },
    components: DEFAULT_CONFIG.components.map(c => ({
      ...c,
      visible: true,
      collapsed: false,
    })),
  },

  server: {
    layout: {
      preset: 'server',
      columns: 2,
      density: 'comfortable',
      enableGridLayout: false,
    },
    components: DEFAULT_CONFIG.components.map(c => ({
      ...c,
      visible: ['disk', 'temperature', 'memory', 'alerts', 'power'].includes(c.type),
    })),
  },

  gaming: {
    layout: {
      preset: 'gaming',
      columns: 2,
      density: 'comfortable',
      enableGridLayout: false,
    },
    components: DEFAULT_CONFIG.components.map(c => ({
      ...c,
      visible: ['cpu', 'gpu', 'temperature', 'chart', 'memory'].includes(c.type),
    })),
  },

  custom: {
    layout: {
      preset: 'custom',
      columns: 2,
      density: 'comfortable',
      enableGridLayout: false,
    },
  },
};
