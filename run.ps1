# ppGame PowerShell 启动脚本
# 设置当前目录为脚本所在目录
Set-Location -Path $PSScriptRoot
Write-Host "未找到 ppGame.exe 文件，正在自动构建项目..." -ForegroundColor Yellow
Write-Host "使用 cargo build --release 命令构建..." -ForegroundColor Cyan
# 执行构建命令
$buildResult = cargo build --release


# 检查可执行文件是否存在
$exePath = ".\target\release\ppGame.exe"
if (-not (Test-Path $exePath)) {
   # 检查构建结果
    if ($LASTEXITCODE -ne 0) {
        Write-Host "构建失败!" -ForegroundColor Red
        Pause
        exit 1
    } else {
        Write-Host "构建成功!" -ForegroundColor Green
    }
    
    
}

# 启动游戏
Write-Host "正在启动 ppGame..." -ForegroundColor Green
Start-Process -FilePath $exePath -Wait

# 检查游戏退出状态
if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "游戏异常退出，错误代码: $LASTEXITCODE" -ForegroundColor Red
    Pause
}