use std::{env, fs, path::PathBuf};

use crate::components::{
    home::{header::HomeHeader, menu::HomeMenu},
    interface::PageLayout,
};
use anyhow::{Ok, Result};
use gpui::{AppContext, ParentElement, Render, SharedString, Styled, px, relative};
use gpui_component::hsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub(crate) struct TodoListHome {
    home_header: gpui::Entity<HomeHeader>,
    home_menu: gpui::Entity<HomeMenu>,
    task_list: Vec<Task>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Task {
    id: i64,
    task_name: String,
    priority: u8,
    create_time: String,
    overdue_time: Option<String>,
    description: String,
}

impl TodoListHome {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
        let mut instance = Self {
            home_header: cx.new(|_| HomeHeader::new()),
            home_menu: cx.new(|cx| HomeMenu::new(window, cx)),
            task_list: vec![],
        };
        instance.load_task_list(cx);
        instance
    }
}

impl Render for TodoListHome {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.page_layout(cx)
    }
}

impl PageLayout for TodoListHome {
    fn page_layout(&mut self, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .child(self.home_header.clone())
            .child(self.home_menu.clone())
            .child(
                gpui::div()
                    .w_full()
                    .mt_3()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .child(
                        gpui::div()
                            .w(relative(0.35))
                            .h_0p5()
                            .bg(hsl(0.0, 0.0, 92.0))
                            .mr(px(12.0)),
                    )
                    .child(SharedString::new("任务列表"))
                    .text_color(hsl(0.0, 0.0, 76.0))
                    .child(
                        gpui::div()
                            .w(relative(0.35))
                            .h_0p5()
                            .bg(hsl(0.0, 0.0, 92.0))
                            .ml(px(12.0)),
                    ),
            )
            .child(self.task_list())
    }
}

impl TodoListHome {
    fn task_list(&mut self) -> impl gpui::IntoElement {
        gpui::div().children(self.task_list.iter().map(|task| {
            gpui::div()
                .child(task.task_name.clone())
                .child(task.priority.to_string())
                .child(task.create_time.to_string())
                .child(
                    task.overdue_time
                        .as_ref()
                        .map(|t| t.to_string())
                        .unwrap_or_default(),
                )
                .child(task.description.clone())
        }))
    }

    // 使用 json 文件代替数据库操作
    fn load_task_list(&mut self, cx: &mut gpui::Context<Self>) {
        cx.spawn(async move |weak_entity, cx| -> Result<()> {
            let task_list_data_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
                .join("data")
                .join("task_list.json");

            // 如果文件不存在，创建目录和空文件
            if !task_list_data_path.exists() {
                // 获取父目录并创建
                if let Some(parent_dir) = task_list_data_path.parent() {
                    fs::create_dir_all(parent_dir)?;
                }
                // 创建空的 JSON 数组文件
                fs::write(&task_list_data_path, "[]")?;
                return Ok(());
            }

            // 读取文件内容为字符串
            let list_json = fs::read_to_string(&task_list_data_path)?;
            let task_list: Vec<Task> = serde_json::from_str(&list_json)?;

            // 更新实体状态
            weak_entity.update(cx, |entity, _cx| {
                entity.task_list = task_list;
            })?;

            Ok(())
        })
        .detach();
    }
}
