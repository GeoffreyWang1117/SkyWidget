# SkyWidget 项目初始化脚本
# 使用方法: 在新的 PowerShell 窗口中运行 .\setup.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " SkyWidget 项目初始化" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Node.js
Write-Host "[1/4] 检查 Node.js..." -ForegroundColor Yellow
try {
    $nodeVersion = node --version
    Write-Host "  ✓ Node.js 版本: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Node.js 未找到! 请重新打开终端或检查 PATH 配置" -ForegroundColor Red
    exit 1
}

# 检查 pnpm
Write-Host "[2/4] 检查 pnpm..." -ForegroundColor Yellow
try {
    $pnpmVersion = pnpm --version
    Write-Host "  ✓ pnpm 版本: $pnpmVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ pnpm 未找到! 请重新打开终端或检查安装" -ForegroundColor Red
    exit 1
}

# 检查 Rust
Write-Host "[3/4] 检查 Rust..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version
    Write-Host "  ✓ Rust 版本: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Rust 未找到! 请重新打开终端或检查 PATH 配置" -ForegroundColor Red
    exit 1
}

# 安装依赖
Write-Host "[4/4] 安装项目依赖..." -ForegroundColor Yellow
Write-Host "  正在运行: pnpm install" -ForegroundColor Gray
pnpm install

if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✓ 依赖安装成功!" -ForegroundColor Green
} else {
    Write-Host "  ✗ 依赖安装失败!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host " 初始化完成!" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📦 项目已准备就绪,运行以下命令开始开发:" -ForegroundColor Green
Write-Host ""
Write-Host "  pnpm tauri:dev    # 启动开发服务器" -ForegroundColor White
Write-Host "  pnpm build        # 构建前端" -ForegroundColor White
Write-Host "  pnpm tauri:build  # 构建发布版本" -ForegroundColor White
Write-Host ""
