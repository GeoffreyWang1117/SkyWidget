# 应用图标

## 📝 说明

Tauri 需要以下格式的图标文件：

- `32x32.png` - Windows 任务栏图标
- `128x128.png` - Windows 应用图标
- `128x128@2x.png` - 高分辨率图标
- `icon.icns` - macOS 图标
- `icon.ico` - Windows ICO 图标

## 🎨 生成图标

### 方式 1: 使用 Tauri 官方工具

```bash
# 准备一个 1024x1024 的 PNG 图标源文件 (app-icon.png)
pnpm tauri icon path/to/app-icon.png
```

### 方式 2: 在线工具

访问 https://icon.kitchen/ 生成所有需要的图标格式

### 方式 3: 使用占位符

**首次开发可以先使用 Tauri 默认图标**

Tauri 在首次运行 `pnpm tauri:dev` 时，如果检测到缺少图标，会：
1. 使用默认 Tauri 图标
2. 或提示您生成图标

---

## ⚡ 快速开始（使用默认图标）

如果暂时没有设计好的图标，可以：

1. 删除 `tauri.conf.json` 中的 icon 配置
2. 或暂时注释掉 icon 相关配置
3. Tauri 会使用内置默认图标

---

## 📐 推荐尺寸

- 源文件: 1024x1024 PNG (无透明度)
- 颜色: 适合深色和浅色背景
- 风格: 简洁、可识别

---

**提示**: 开发阶段可以先跳过图标，专注于功能开发。后续再添加精美图标。
