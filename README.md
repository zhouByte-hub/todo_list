# Todo List

一个基于 Rust 和 GPUI 框架开发的现代化待办事项桌面应用。

## 效果预览

### 首页

![首页效果图](docs/home_page.png)

## 功能特性

- � **任务管理** - 创建、查看和管理待办任务
- 🎯 **优先级分类** - 支持高、中、低三种优先级，不同优先级显示不同图标
- 🔍 **任务搜索** - 支持按任务名称和描述搜索
- 🏷️ **分类筛选** - 按优先级筛选任务列表
- 🎨 **现代化 UI** - 基于 GPUI Component 的美观界面
- 📱 **底部导航** - 首页、分类、提醒、我的四个功能模块
- ➕ **快速添加** - 底部中央醒目的添加按钮
- � **本地存储** - 使用 JSON 文件存储任务数据
- �️ **资源嵌入** - 使用 rust-embed 嵌入资源，无需外部依赖文件
- 🚀 **跨平台** - 支持 macOS、Windows、Linux

## 技术栈

- **Rust 2024 Edition** - 最新版 Rust，提供高性能和内存安全
- **GPUI 0.2** - 现代化的跨平台 UI 框架
- **GPUI Component 0.5** - 丰富的 UI 组件库
- **Chrono** - 日期时间处理
- **Serde** - 序列化/反序列化
- **Rust-Embed** - 资源文件嵌入
- **Lazy Static** - 全局静态变量

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

1. **首页** - 查看所有任务列表，支持搜索和筛选
2. **分类** - 按类别管理任务（开发中）
3. **提醒** - 设置任务提醒（开发中）
4. **我的** - 个人设置和偏好（开发中）

### 任务数据

任务数据存储在 `data/task_list.json` 文件中，格式如下：

```json
[
  {
    "id": 1,
    "task_name": "示例任务",
    "priority": "high",
    "create_time": "2024-01-01",
    "overdue_time": "2024-01-15",
    "description": "这是一个示例任务"
  }
]
```

## 项目结构

```
todo_list/
├── src/
│   ├── components/           # 组件模块
│   │   ├── home/             # 首页组件
│   │   │   ├── header.rs     # 页面头部
│   │   │   ├── home_page.rs  # 首页主页面
│   │   │   └── menu.rs       # 分类菜单
│   │   ├── category/         # 分类组件
│   │   ├── reminder/         # 提醒组件
│   │   ├── profile/          # 我的组件
│   │   ├── interface.rs      # 页面布局 trait
│   │   └── layout.rs         # 主布局组件
│   ├── assets/               # 资源文件
│   │   ├── icon/
│   │   │   ├── home/         # 任务优先级图标
│   │   │   └── tabber/       # 底部导航图标
│   │   └── images/           # 图片资源
│   ├── main.rs               # 应用入口
│   └── todo_icon_assets.rs   # 资源加载
├── config/                   # 配置文件
│   └── home_menu.json        # 首页菜单配置
├── data/                     # 数据文件
│   └── task_list.json        # 任务列表数据
├── docs/                     # 文档
├── Cargo.toml                # 项目配置
└── README.md                 # 项目说明
```

## 开发

### 窗口配置

应用窗口默认配置：
- 尺寸：450 x 800 像素
- 居中显示
- 透明标题栏

### 添加新功能

1. 在 `src/components/` 下创建新的组件模块
2. 实现 `PageLayout` trait
3. 在 `layout.rs` 中注册新的标签页

