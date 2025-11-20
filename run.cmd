@echo off
title pp_game
cd /d "%~dp0"

if not exist ".\target\release\pp_game.exe" (
    echo 未找到 pp_game.exe 文件，正在自动构建项目...
    echo 使用 cargo build --release 命令构建...
    cargo build --release
    
    if %errorlevel% neq 0 (
        echo 构建失败!
        pause
        exit /b 1
    ) else (
        echo 构建成功!
    )
)

echo 正在启动 pp_game...
.\target\release\pp_game.exe

if %errorlevel% neq 0 (
    echo.
    echo 游戏异常退出，错误代码: %errorlevel%
    pause
)