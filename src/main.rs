use std::path::PathBuf;

use anyhow::{Error, Ok, Result};
use gpui::{AppContext, Application, Bounds, Pixels, Size, WindowBounds};
use gpui_component::{Root, Theme};
use tokio::io::AsyncWriteExt;

use crate::{
    components::{home::home_page::TodoListHome, layout::TodoLayout},
    todo_icon_assets::TodoIconAssets,
};
mod components;
mod todo_icon_assets;

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
                // window.set_window_title("Todo List");
                let home_page = cx.new(|_| TodoListHome::default());
                let layout = cx.new(|_| TodoLayout::default(Some(home_page.into())));
                cx.new(|cx| Root::new(layout, window, cx))
            });
            if let Err(e) = root_window {
                record_error(&e).await?;
            }
            Ok(())
        })
        .detach();
    });
}

async fn record_error(e: &Error) -> Result<()> {
    let mut launch_error_path = PathBuf::from("./logs");
    if !launch_error_path.exists() {
        tokio::fs::create_dir(&launch_error_path).await?;
    }
    launch_error_path.push("/Launch.log");
    if launch_error_path.exists() {
        tokio::fs::remove_file(&launch_error_path).await?;
    }
    let mut error_file = tokio::fs::OpenOptions::new()
        .write(true)
        .open(launch_error_path)
        .await?;
    Ok(error_file.write_all(e.to_string().as_bytes()).await?)
}
