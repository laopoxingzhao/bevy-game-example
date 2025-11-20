# pp_game - Rust + Bevy 乒乓球游戏

这是一个使用 Rust 编程语言和 Bevy 游戏引擎构建的简单乒乓球游戏。

## 功能特点

- 经典乒乓球游戏玩法
- 单人模式（玩家对抗AI）
- 简洁的菜单界面
- 物理碰撞检测
- 游戏状态管理（菜单、游戏中、游戏结束）

## 技术栈

- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Bevy](https://bevyengine.org/) - 数据驱动的游戏引擎，版本 0.17.2
- Bevy ECS 架构
- Bevy UI 系统

## 快速开始

### 环境要求

- Rust 工具链（建议使用最新版本）
- Cargo 包管理器

### 安装与运行

1. 克隆项目：
   ```bash
   git clone <repository-url>
   cd pp_game
   ```

2. 运行游戏：
   ```bash
   cargo run
   ```

3. 构建发布版本：
   ```bash
   cargo build --release
   ```

## 游戏操作

- 使用方向键 ↑ 和 ↓ 控制左侧挡板上下移动
- 点击菜单中的 "Play" 按钮开始游戏
- 将球弹回对方半场得分，防止球从自己半场出界

## 项目结构

```
src/
├── components.rs     # 游戏组件定义
├── main.rs           # 程序入口
├── stares.rs         # 游戏状态枚举
├── plugins/          # 自定义插件
│   ├── game_plugin.rs
│   └── mod.rs
├── systems/          # 游戏系统逻辑
│   ├── ball.rs
│   ├── gutters.rs
│   ├── paddle.rs
│   ├── setup.rs
│   └── mod.rs
└── ui/               # 用户界面
    ├── menu_ui.rs
    └── mod.rs
```

## 架构说明

本项目采用 Bevy 的 ECS（实体-组件-系统）架构模式：

- **组件 (Components)**: 定义在 `components.rs` 中，包括位置、速度、球、挡板等
- **系统 (Systems)**: 分布在 `systems/` 目录下，处理游戏逻辑如球的移动、碰撞检测等
- **插件 (Plugins)**: 在 `plugins/` 目录中，组织和封装游戏功能
- **状态管理**: 使用 Bevy 的状态机制管理游戏状态（菜单、游戏中、游戏结束）

## 开发计划

- [x] 基础游戏机制实现
- [x] 菜单界面
- [x] AI 对手
- [ ] 计分系统
- [ ] 多球模式
- [ ] 音效支持
- [ ] 更丰富的视觉效果

## 许可证

本项目基于 MIT 许可证开源，详见 [LICENSE](LICENSE) 文件。