# SkyWidget - 分布式硬件监控与告警系统

<p align="center">
  <strong>支持局域网 P2P 通信的分布式硬件监控与告警系统</strong>
</p>

## ✨ 核心特性

- 🖥️ **实时硬件监控**: CPU、内存、磁盘、温度、GPU 实时监控 ✨ 增强
- 🎮 **GPU 监控**: NVIDIA GPU 完整监控（AMD/Intel 即将支持）✨ 新增
- 🌡️ **温度传感器**: CPU/GPU/南桥温度监控与预警 ✨ 增强
- ⚠️ **南桥芯片监控**: 防止磁盘掉线和 CMOS 错误的关键监控 ✨ 新增
- 📊 **性能趋势图**: SVG 时序图表可视化
- 🔔 **系统托盘集成**: 后台运行，快速访问
- 🌐 **自动节点发现**: 基于 mDNS 的零配置设备发现
- 📡 **P2P 网络通信**: 同一局域网内节点自动互联
- 🚨 **智能告警系统**: 可自定义的告警规则和阈值
- 📨 **跨设备通知**: 告警信息自动推送到网络内其他节点
- 📜 **告警历史**: 完整的告警记录和确认功能
- ⚙️ **自定义规则**: 可视化创建和管理告警规则
- 💾 **数据导出**: 导出告警历史和指标数据
- 🎨 **现代化 UI**: 响应式设计，支持深色模式
- 🔌 **HTTP REST API**: 开放 API 接口，支持第三方集成

## 🆕 最新功能 (v0.4.0)

### GPU 监控支持
- **NVIDIA GPU 完整支持** - 使用 NVML 库实时监控 NVIDIA 显卡
- **多 GPU 系统** - 支持同时监控多个 GPU
- **详细指标监控**:
  - GPU 使用率 - 实时核心负载
  - 显存使用率 - 显存占用百分比
  - 显存容量 - 已用/总容量显示（GB/MB）
  - GPU 温度 - 实时温度监控
  - 功耗监控 - 实时功耗 (W)
  - 风扇转速 - 风扇速度百分比
  - 核心频率 - GPU 时钟频率 (MHz)
  - 驱动版本 - 当前驱动版本信息
- **颜色预警** - 根据温度/使用率自动变色
- **厂商识别** - NVIDIA(绿)、AMD(红)、Intel(蓝) 图标标识
- **未来支持** - AMD 和 Intel Arc GPU 支持开发中
- **跨平台** - Linux/Windows/macOS（需相应驱动）

### ⚠️ 南桥/PCH 芯片监控（关键！）
- **智能识别** - 自动识别南桥、PCH、FCH 等芯片组温度传感器
- **重点显示** - 南桥温度在温度监控面板独立显示，边框高亮
- **严格阈值** - 南桥温度采用更严格的预警标准：
  - <50°C：正常（绿色）
  - 50-60°C：注意（黄色）- 建议检查散热
  - 60-70°C：偏高（橙色）- 可能影响稳定性
  - ≥70°C：危险（红色）- 可能导致磁盘掉线或 CMOS 错误
- **告警规则** - 自动添加南桥温度告警：
  - 60°C 触发警告级别告警
  - 70°C 触发严重级别告警
- **风险提示** - 实时显示南桥过热可能导致的问题：
  - 磁盘控制器异常导致硬盘掉线
  - CMOS 电路错误导致 BIOS 设置丢失
  - 系统不稳定或黑屏
- **传感器分类** - 所有温度传感器按类型分类标识：
  - 🔥 CPU
  - 🎮 GPU
  - ⚠️ 南桥/PCH（重点标识）
  - 💾 磁盘
  - 📊 其他

---

## 📦 历史版本

### v0.3.0 - 系统托盘、温度监控与图表

### 系统托盘集成
- **后台运行** - 应用可最小化到系统托盘
- **托盘菜单** - 右键菜单：显示/隐藏窗口、退出
- **托盘图标** - 原生系统托盘图标支持
- **点击切换** - 单击托盘图标快速显示/隐藏窗口

### 温度传感器监控
- **CPU 温度** - 实时监控 CPU 温度
- **多传感器支持** - 支持所有系统温度传感器
- **温度平均值** - 计算 CPU 核心平均温度
- **临界温度** - 显示传感器临界值和最高温度
- **颜色预警** - 根据温度高低自动变色（绿/黄/橙/红）
- **跨平台支持** - Linux/Windows/macOS（需驱动支持）

### 性能趋势图表
- **SVG 折线图** - 轻量级 SVG 实时图表
- **三项指标** - CPU、内存、磁盘使用率趋势
- **10 分钟视图** - 显示最近 60 个数据点
- **实时统计** - 显示当前值和平均值
- **自动刷新** - 每 30 秒自动更新
- **零依赖** - 纯 SVG 实现，无需外部图表库

---

## 📦 历史版本

### v0.2.0 - 告警历史与自定义规则

#### 告警历史系统
- **历史记录查看** - 查看所有触发的告警记录
- **告警确认** - 标记告警为已读/已处理
- **历史导出** - 导出告警历史为 JSON 格式
- **过滤功能** - 筛选未确认告警
- **自动清理** - 最多保存 1000 条记录

#### 自定义告警规则
- **可视化规则创建** - 通过 UI 界面创建自定义告警规则
- **多种监控指标** - 支持 CPU、内存、磁盘、温度监控
- **四级严重度** - Info/Warning/Error/Critical
- **规则管理** - 启用/禁用/删除告警规则
- **灵活阈值** - 自定义触发阈值（0-100）

#### 数据持久化
- **时序指标存储** - 24小时硬件指标历史（10秒间隔）
- **告警历史存储** - 最多保存 1000 条告警记录
- **数据导出** - 导出指标和告警数据为 JSON

## 🚀 技术栈

**前端**
- Svelte 5 + TailwindCSS 4 + Vite 6
- TypeScript

**后端**
- Rust + Tauri 2.x
- Tokio 异步运行时
- Warp HTTP 服务器
- mDNS-SD 服务发现

**核心依赖**
- `sysinfo` - 跨平台硬件监控
- `mdns-sd` - 本地网络设备发现
- `warp` - HTTP/WebSocket 服务器
- `tokio` - 异步运行时
- `sled` - 轻量级数据存储
- `notify-rust` - 桌面通知

## 📦 快速开始

### 环境要求

- Rust 1.75+ (通过 rustup 安装)
- Node.js 20 LTS (推荐使用 Scoop 管理)
- pnpm 9+
- Windows: Visual Studio 2022 + WebView2

### 安装依赖

```bash
# 安装前端依赖
pnpm install

# Rust 依赖会在首次运行时自动下载
```

### 开发模式

```bash
# 启动开发服务器 (带热重载)
pnpm tauri:dev

# 查看日志输出 (可选)
RUST_LOG=debug pnpm tauri:dev
```

### 构建发布

```bash
# 构建生产版本
pnpm tauri:build

# 构建后的文件位于:
# - Windows: src-tauri/target/release/SkyWidget.exe
# - Linux: src-tauri/target/release/skywidget
# - macOS: src-tauri/target/release/bundle/macos/
```

## 🎯 功能说明

### 1. 硬件监控

实时监控以下指标（每秒刷新）：
- **CPU**: 使用率、核心数、频率、各核心独立使用率
- **内存**: 总量、已用、可用、使用率
- **磁盘**: 各分区容量、使用量、可用空间
- **温度**: CPU 及其他传感器温度（如果支持）
- **GPU** (NVIDIA): 使用率、显存、温度、功耗、风扇、频率（每 2 秒刷新）

性能趋势图表（10 秒间隔，显示最近 10 分钟）：
- **CPU 使用率曲线** - 蓝色折线图
- **内存使用率曲线** - 绿色折线图
- **磁盘使用率曲线** - 紫色折线图
- 每个图表显示当前值、平均值和时间轴

GPU 监控特性：
- **NVIDIA GPU** - 完整支持（使用 NVML）
- **AMD GPU** - 开发中
- **Intel Arc GPU** - 开发中
- **多 GPU** - 同时监控多个显卡
- **详细指标** - 8 项关键 GPU 指标

### 2. 网络发现

- 基于 **mDNS/DNS-SD** 协议自动发现同一局域网内的 SkyWidget 节点
- 无需手动配置，即插即用
- 每 5 秒刷新节点列表
- 自动清理离线节点（30秒超时）
- 支持 Tailscale/WireGuard 等虚拟局域网

### 3. 告警系统

#### 默认告警规则

| 规则名称 | 触发条件 | 严重级别 | 冷却时间 |
|---------|---------|---------|----------|
| CPU 高负载告警 | CPU > 80% | Warning | 5 分钟 |
| CPU 严重告警 | CPU > 95% | Critical | 5 分钟 |
| 内存高负载告警 | 内存 > 85% | Warning | 5 分钟 |
| 磁盘高负载告警 | 磁盘 > 90% | Warning | 5 分钟 |

#### 告警通知方式

1. **本地桌面通知**: 弹出系统通知（Windows/macOS/Linux）
2. **网络推送**: 自动发送到所有已发现的远程节点
3. **冷却机制**: 防止同一告警短时间内重复触发

### 4. REST API

每个节点都运行一个 HTTP 服务器（端口 3030），提供以下接口：

```bash
# 获取节点硬件信息
curl http://localhost:3030/hardware

# 获取已发现的节点列表
curl http://localhost:3030/nodes

# 发送告警通知
curl -X POST http://localhost:3030/alerts/notify \
  -H "Content-Type: application/json" \
  -d '{
    "source_node_id": "xxx",
    "source_node_name": "Node-A",
    "alert_type": "cpu_high",
    "message": "CPU 使用率过高",
    "severity": "Warning",
    "timestamp": 1234567890
  }'
```

## 🔌 Tailscale/WireGuard 集成

SkyWidget 自动支持虚拟局域网：

1. **Tailscale 用户**: 确保所有设备已连接到同一 Tailnet，SkyWidget 会自动发现
2. **WireGuard 用户**: 配置好 WireGuard 网络后，节点会自动互联
3. **无需额外配置**: mDNS 在虚拟网卡上也能正常工作

## 🎯 使用场景

- 🏠 **家庭网络监控**: 监控家中所有电脑的运行状态
- 🏢 **小型办公室**: 实时掌握办公设备健康状况
- 🎮 **游戏工作室**: 监控多台游戏服务器负载
- 💻 **开发团队**: 监控开发环境和构建服务器
- 🌍 **远程办公**: 通过 Tailscale 监控异地设备

## 🛠️ 开发计划

### ✅ 已完成
- [x] 项目脚手架搭建
- [x] 基础硬件监控 (CPU/内存/磁盘)
- [x] mDNS 节点发现
- [x] HTTP REST API 服务器
- [x] 告警规则引擎
- [x] 跨设备告警通知
- [x] 时序数据存储
- [x] 前端网络拓扑界面
- [x] 前端告警配置界面
- [x] 告警历史查看
- [x] 自定义告警规则
- [x] 数据导出 (JSON)
- [x] 温度传感器监控
- [x] 系统托盘集成
- [x] 性能趋势图表
- [x] NVIDIA GPU 监控

### 🔜 计划中
- [ ] AMD GPU 监控
- [ ] Intel Arc GPU 监控
- [ ] GPU 指标趋势图表
- [ ] 网络流量监控
- [ ] 进程监控与管理
- [ ] WebSocket 实时推送
- [ ] 数据导出 CSV 格式
- [ ] 自定义图表时间范围
- [ ] 移动端 APP
- [ ] Docker 容器监控

## 📁 项目结构

```
tauri-monitor/
├── src/                    # 前端源码 (Svelte)
│   ├── lib/
│   │   ├── components/    # Svelte 组件
│   │   ├── stores/        # 状态管理
│   │   └── utils/         # 工具函数
│   ├── App.svelte         # 根组件
│   ├── main.js            # 入口文件
│   └── app.css            # 全局样式
├── src-tauri/              # 后端源码 (Rust)
│   ├── src/
│   │   └── main.rs        # Tauri 主程序
│   ├── icons/             # 应用图标
│   ├── capabilities/      # 权限配置
│   ├── Cargo.toml         # Rust 依赖
│   └── tauri.conf.json    # Tauri 配置
├── public/                 # 静态资源
├── package.json           # 前端依赖
└── vite.config.js         # Vite 配置
```

## 🌐 网络架构

### 节点发现 (mDNS)
```
节点 A                节点 B                节点 C
  │                     │                     │
  └─────── mDNS ────────┴────── mDNS ─────────┘
         自动发现
```

### 告警通知流程
```
节点 A (检测到告警)
  │
  ├─► 本地桌面通知
  │
  └─► HTTP POST → 节点 B/C (告警通知)
                   │
                   └─► 触发远程节点桌面通知
```

### API 端点

所有节点默认在端口 `3030` 提供 HTTP API：

- `GET /health` - 健康检查
- `GET /node` - 获取节点信息
- `GET /hardware` - 获取硬件监控数据
- `GET /nodes` - 获取已发现的节点列表
- `POST /alerts/notify` - 接收告警通知

## 🔧 系统要求

### 必需依赖

**所有平台**
- Rust 1.75+ (通过 rustup 安装)
- Node.js 20 LTS
- pnpm 9+

**Linux 额外依赖**
```bash
# Ubuntu/Debian
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libgtk-3-dev \
  libgdk-pixbuf-2.0-dev \
  libpango1.0-dev \
  libatk1.0-dev

# Fedora/RHEL
sudo dnf install \
  webkit2gtk4.1-devel \
  openssl-devel \
  gtk3-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel
```

**Windows**
- Visual Studio 2022 Build Tools
- WebView2 Runtime (通常已预装)

**macOS**
- Xcode Command Line Tools
```bash
xcode-select --install
```

## 📝 许可证

MIT License

## 👨‍💻 作者

Geoffrey Wang
