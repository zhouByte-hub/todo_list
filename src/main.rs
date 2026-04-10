use std::{fs, io::Write, path::PathBuf};

use anyhow::{Error, Ok, Result};
use gpui::{AppContext, Application, Bounds, Pixels, Size, WindowBounds};
use gpui_component::{Root, Theme};

use crate::{components::layout::TodoLayout, todo_icon_assets::TodoIconAssets};
mod components;
mod todo_icon_assets;

/**
   - 实体创建时机 ：在 new() 或 default() 方法中创建实体，而不是在 render() 中
   - 状态管理 ：缓存页面实体，避免重复创建导致状态丢失
   - 异步操作 ：在初始化时执行，避免在 render 中重复触发
   - 订阅管理 ：在构造函数中建立订阅，确保只订阅一次
*/
fn main() {
    Application::new().with_assets(TodoIconAssets).run(|app| {
        app.set_global(Theme::default());
        gpui_component::init(app);

        let mut window_options = gpui::WindowOptions::default();
        // 设置窗口大小
        let bounds = Bounds::centered(
            None,
            Size::new(Pixels::from(450.0), Pixels::from(800.0)),
            app,
        );
        window_options.window_bounds = Some(WindowBounds::Windowed(bounds));

        // 设置窗口为弹出窗口
        window_options.kind = gpui::WindowKind::PopUp;
        window_options.focus = true;

        // 修改 titleBar 样式
        let mut title_bar_options = gpui::TitlebarOptions::default();
        title_bar_options.appears_transparent = true;
        window_options.titlebar = Some(title_bar_options);

        app.spawn(async move |app| -> Result<()> {
            let root_window = app.open_window(window_options, |window, cx| {
                // 创建布局实体，并初始化首页
                let layout = cx.new(|cx| TodoLayout::new(window, cx));
                // 创建根组件实体，作为窗口的第一级子元素，启用 GPUI Component 功能
                cx.new(|cx| Root::new(layout, window, cx))
            });
            if let Err(e) = root_window {
                record_error(&e)?;
            }
            Ok(())
        })
        .detach();
    });
}

fn record_error(e: &Error) -> Result<()> {
    let mut launch_error_path = PathBuf::from("./logs");
    if !launch_error_path.exists() {
        fs::create_dir(&launch_error_path)?;
    }
    launch_error_path.push("/Launch.log");
    if launch_error_path.exists() {
        fs::remove_file(&launch_error_path)?;
    }
    let mut error_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(launch_error_path)?;
    Ok(error_file.write_all(e.to_string().as_bytes())?)
}
