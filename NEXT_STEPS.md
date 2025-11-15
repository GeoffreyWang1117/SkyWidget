# ⚡ 立即开始使用 SkyWidget

## 🎉 项目初始化完成！

所有配置文件和项目结构已创建完成。现在只需3步即可启动开发：

---

## 📝 必需步骤

### 步骤 1: 关闭当前终端

**⚠️ 重要**: 由于环境变量已更新(PATH)，必须关闭所有终端窗口并重新打开。

---

### 步骤 2: 打开新终端并进入项目目录

```powershell
cd C:\Users\wzhh9\Projects\tauri-monitor
```

---

### 步骤 3: 运行初始化脚本

```powershell
.\setup.ps1
```

**这个脚本会自动：**
- ✅ 检查所有环境（Rust, Node.js, pnpm）
- ✅ 安装前端依赖 (pnpm install)
- ✅ 显示可用命令

---

## 🚀 启动开发服务器

初始化完成后，运行：

```bash
pnpm tauri:dev
```

**首次启动会：**
1. 下载 Rust 依赖 (~5-10分钟，仅首次)
2. 编译 Tauri 后端
3. 启动前端开发服务器
4. 打开应用程序窗口

---

## 📦 已配置的内容

### ✅ 环境
- Scoop 包管理器 (0.5.3)
- Node.js 24.11.1 LTS
- pnpm (最新版)
- Rust 1.91.1
- Visual Studio 2022 (MSVC)
- WebView2

### ✅ 项目
- Tauri 2.x 框架
- Svelte 5 (Runes 模式)
- TailwindCSS 4
- Vite 6
- sysinfo 硬件监控库
- VSCode 工作区配置
- Git 仓库 (关联到 GitHub)

### ✅ 代码
- 基础 Rust 后端 (`greet`, `get_system_info` 命令)
- Svelte 前端框架
- TailwindCSS 样式系统
- 深色/浅色主题

---

## 🔧 可用命令

```bash
pnpm tauri:dev     # 启动开发模式(热重载)
pnpm build         # 构建前端
pnpm tauri:build   # 构建发布版本
pnpm format        # 格式化代码
```

---

## 📖 相关文档

- **详细指南**: `SETUP_GUIDE.md`
- **项目说明**: `README.md`
- **Tauri 文档**: https://v2.tauri.app
- **Svelte 5 文档**: https://svelte.dev

---

## ⚠️ 常见问题

### Q: 运行脚本提示"无法识别"
**A**: 确保已关闭并重新打开终端

### Q: pnpm 找不到
**A**: 检查 PATH 是否包含 `C:\Users\wzhh9\scoop\persist\nodejs-lts\bin`

### Q: 首次编译很慢
**A**: 正常现象，Rust 首次编译需要下载并编译大量依赖

### Q: WebView2 错误
**A**: Tauri 会自动下载，等待完成即可

---

## 🎯 下一步开发计划

根据计划书，接下来应实现：

1. **基础监控模块** (第1周 Day 3-4)
   - CPU 使用率监控
   - 内存使用监控
   - 磁盘使用监控
   - 编写 Tauri 命令 API

2. **UI 组件** (第1周 Day 5-7)
   - 磁贴组件设计
   - 实时数据刷新
   - 窗口置顶功能

参考文件位置：
- Rust 后端: `src-tauri/src/main.rs`
- 前端组件: `src/App.svelte`
- 组件目录: `src/lib/components/`

---

**准备好了吗？关闭终端，重新开始吧！🚀**
