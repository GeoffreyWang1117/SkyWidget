# 🚀 SkyWidget 项目初始化指南

## ✅ 已完成的配置

### 1. 开发环境
- ✅ Git 仓库初始化并关联到 GitHub
- ✅ Rust 1.91.1 + Cargo (环境变量已配置)
- ✅ Node.js 24.11.1 LTS (通过 Scoop 管理)
- ✅ pnpm 包管理器
- ✅ Visual Studio 2022 (MSVC 编译器)
- ✅ WebView2 运行时

### 2. 项目结构
```
tauri-monitor/
├── src/                    # Svelte 5 前端
│   ├── lib/
│   │   ├── components/    # 组件目录
│   │   ├── stores/        # 状态管理
│   │   └── utils/         # 工具函数
│   ├── App.svelte         # 根组件
│   ├── main.js            # 入口
│   └── app.css            # TailwindCSS
├── src-tauri/              # Rust 后端
│   ├── src/main.rs        # Tauri 主程序
│   ├── Cargo.toml         # Rust 依赖 (包含 sysinfo)
│   └── tauri.conf.json    # Tauri 配置
├── package.json           # 前端依赖
├── vite.config.js         # Vite 配置
├── tailwind.config.js     # TailwindCSS 配置
├── setup.ps1              # 自动化安装脚本
└── README.md              # 项目说明
```

### 3. 已配置的功能
- ✅ Tauri 2.x 框架
- ✅ Svelte 5 (Runes 模式)
- ✅ TailwindCSS 4
- ✅ Vite 6 构建工具
- ✅ sysinfo 硬件监控库
- ✅ VSCode 工作区配置
- ✅ Prettier 代码格式化
- ✅ Git 配置 (.gitignore)

---

## 📋 下一步操作

### ⚠️ 重要：首次使用必须重启终端！

由于环境变量已更新 (Rust, Node.js, pnpm 的 PATH)，您需要：

**关闭当前所有终端窗口，然后打开新的 PowerShell/CMD**

---

### 方式 1: 使用自动化脚本（推荐）

在**新的 PowerShell 窗口**中运行：

```powershell
cd C:\Users\wzhh9\Projects\tauri-monitor
.\setup.ps1
```

这个脚本会：
1. 检查所有环境变量
2. 自动安装前端依赖 (pnpm install)
3. 显示使用说明

---

### 方式 2: 手动安装

如果自动脚本失败，手动执行：

```bash
# 1. 进入项目目录
cd C:\Users\wzhh9\Projects\tauri-monitor

# 2. 安装前端依赖
pnpm install

# 3. 启动开发服务器
pnpm tauri:dev
```

---

## 🛠️ 常用命令

```bash
# 开发模式 (带热重载)
pnpm tauri:dev

# 仅运行前端开发服务器
pnpm dev

# 构建前端
pnpm build

# 构建发布版本 (生成安装包)
pnpm tauri:build

# 格式化代码
pnpm format
```

---

## 🎯 已实现的功能

### Rust 后端 (src-tauri/src/main.rs)
- ✅ `greet(name)` - 简单的问候命令
- ✅ `get_system_info()` - 获取系统信息
  - 系统名称、内核版本
  - CPU 核心数
  - 内存使用情况

### 前端 (src/App.svelte)
- ✅ 基础 UI 框架
- ✅ TailwindCSS 样式系统
- ✅ 深色/浅色主题支持
- ✅ Tauri API 集成示例

---

## 📦 依赖说明

### 前端依赖 (package.json)
- `@tauri-apps/api` - Tauri JavaScript API
- `svelte` ^5.19.0 - Svelte 5 框架
- `vite` ^6.0.11 - 构建工具
- `tailwindcss` ^4.0.0 - CSS 框架

### Rust 依赖 (Cargo.toml)
- `tauri` 2.2 - Tauri 核心
- `sysinfo` 0.32 - 跨平台硬件监控
- `serde` - JSON 序列化

---

## ⚠️ 可能遇到的问题

### 1. 命令找不到 (rustc / node / pnpm)
**原因**: 环境变量未生效
**解决**: 关闭所有终端，重新打开

### 2. pnpm install 失败
**原因**: 网络问题或 Node.js 未正确安装
**解决**:
```bash
# 检查 Node.js
node --version

# 检查 pnpm
pnpm --version

# 清除缓存重试
pnpm store prune
pnpm install
```

### 3. Rust 编译错误
**原因**: MSVC 工具链未找到
**解决**:
- 确保 Visual Studio 2022 已安装 "C++ 构建工具"
- 运行 `rustup default stable-msvc`

### 4. WebView2 错误
**原因**: WebView2 未安装
**解决**: Tauri 会自动下载，或手动下载：
https://developer.microsoft.com/microsoft-edge/webview2/

---

## 🎨 推荐的 VSCode 插件

打开项目后，VSCode 会提示安装推荐插件：
- **Svelte for VS Code** - Svelte 语法支持
- **rust-analyzer** - Rust 智能提示
- **Tauri** - Tauri 开发工具
- **Tailwind CSS IntelliSense** - TailwindCSS 提示
- **Prettier** - 代码格式化

---

## 📖 下一步开发计划

按照开发计划书，接下来应该：

### 第 1 周 (Day 3-4): 核心监控模块
1. 实现 CPU 监控 (使用 sysinfo)
2. 实现内存监控
3. 实现磁盘监控
4. 编写 Tauri Command API
5. 前端调用测试

### 参考文件
- 计划书: `/path/to/your/development_plan.md`
- Tauri 文档: https://v2.tauri.app
- Svelte 5 文档: https://svelte.dev/docs/svelte/overview
- sysinfo 文档: https://docs.rs/sysinfo/

---

## 🔗 快速链接

- GitHub 仓库: https://github.com/GeoffreyWang1117/SkyWidget
- Tauri 官方文档: https://v2.tauri.app
- Svelte 5 文档: https://svelte.dev
- sysinfo crate: https://github.com/GuillaumeGomez/sysinfo

---

## 💬 获取帮助

如果遇到问题：
1. 查看 `README.md` 的常见问题
2. 检查 Tauri 官方文档
3. 在项目 Issues 提问
4. 联系开发团队

---

**祝开发顺利！🎉**
