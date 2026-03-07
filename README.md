# Todo List

一个基于 Rust 和 GPUI 框架开发的现代化待办事项桌面应用。

## 功能特性

- 📅 **周视图任务管理** - 直观的周视图，快速查看和管理每日任务
- 🎯 **多标签页导航** - 首页、分类、提醒、我的四个主要功能模块
- 🎨 **自定义主题** - 支持亮色/暗色主题切换
- 🖼️ **图标集成** - 使用自定义 SVG 图标，界面美观简洁
- 💾 **资源嵌入** - 使用 rust-embed 嵌入资源，无需外部依赖文件
- 🚀 **跨平台** - 支持 macOS、Windows、Linux

## 技术栈

- **Rust** - 系统编程语言，提供高性能和内存安全
- **GPUI** - 现代化的跨平台 UI 框架
- **GPUI Component** - 丰富的 UI 组件库
- **Chrono** - 日期时间处理
- **Tokio** - 异步运行时

## 安装

### 前置要求

- Rust 1.90 或更高版本
- Cargo（随 Rust 一起安装）

### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/your-username/todo_list.git
cd todo_list

# 构建项目
cargo build

# 运行应用
cargo run
```

## 使用方法

### 启动应用

```bash
cargo run
```

### 主要功能

1. **首页** - 查看周视图任务，今日任务高亮显示
2. **分类** - 按类别管理任务
3. **提醒** - 设置任务提醒
4. **我的** - 个人设置和偏好

## 项目结构

```
todo_list/
├── src/
│   ├── components/       # 组件模块
│   │   ├── home/       # 首页组件
│   │   ├── category/   # 分类组件
│   │   ├── reminder/   # 提醒组件
│   │   ├── profile/    # 我的组件
│   │   └── layout.rs   # 布局组件
│   ├── assets/          # 资源文件
│   │   ├── icon/       # SVG 图标
│   │   └── images/     # 图片资源
│   └── main.rs         # 应用入口
├── docs/               # 文档
├── Cargo.toml          # 项目配置
└── README.md           # 项目说明
```

## 开发

### 添加新功能

1. 在 `src/components/` 下创建新的组件模块
2. 实现相应的页面布局
3. 在 `layout.rs` 中注册新的标签页

### 代码规范

- 遵循 Rust 官方代码风格
- 使用有意义的变量和函数命名
- 添加必要的注释和文档

## 贡献

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目采用 Apache-2.0 许可证 - 详见 [LICENSE](LICENSE) 文件

## 联系方式

- 项目链接: [https://github.com/your-username/todo_list](https://github.com/your-username/todo_list)
- 问题反馈: [Issues](https://github.com/your-username/todo_list/issues)

## 致谢

- [GPUI](https://gpui.rs/) - 跨平台 UI 框架
- [GPUI Component](https://longbridge.github.io/gpui-component/) - UI 组件库
- [Lucide](https://lucide.dev/) - 图标库
