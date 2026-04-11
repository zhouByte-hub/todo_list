use std::{collections::HashMap, env, fs, path::PathBuf};

use crate::components::{
    home::{header::HomeHeader, menu::HomeMenu},
    interface::PageLayout,
};
use anyhow::{Ok, Result};
use gpui::{AppContext, Hsla, ParentElement, Render, SharedString, Styled, px, relative};
use gpui_component::hsl;
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref TASK_ICON_PATH: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert("high".to_string(), "icon/home/high.png".to_string());
        map.insert("medium".to_string(), "icon/home/medium.png".to_string());
        map.insert("low".to_string(), "icon/home/low.png".to_string());
        map
    };
}

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
    priority: String,
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
        let default_icon = String::from("icon/home/high.png");
        let task_childrens = self.task_list.iter().map(|task| {
            let icon_path = TASK_ICON_PATH.get(&task.priority).unwrap_or(&default_icon);
            gpui::div()
                .w(relative(0.95))
                .mb_2()
                .bg(Hsla::white())
                .rounded(px(12.0))
                .p_2()
                .pl_12()
                .relative()
                .child(
                    gpui::img(icon_path.as_str())
                        .w(px(50.0))
                        .h(px(50.0))
                        .absolute()
                        .top(px(-8.0))
                        .left(px(-8.0)),
                )
                .child(
                    gpui::div()
                        .child(
                            gpui::div()
                                .child(task.task_name.clone())
                                .text_size(px(18.0))
                                .text_color(hsl(0.0, 0.0, 20.0)),
                        )
                        .child(
                            gpui::div()
                                .child(task.description.clone())
                                .text_size(px(14.0))
                                .text_color(hsl(0.0, 0.0, 60.0)),
                        ),
                )
                .child(
                    gpui::div().w_full().flex().justify_end().child(
                        gpui::div()
                            .child(
                                task.overdue_time
                                    .clone()
                                    .unwrap_or("未设置逾期时间".to_string()),
                            )
                            .text_color(hsl(0.0, 0.0, 80.0))
                            .text_size(px(12.0)),
                    ),
                )
        });
        gpui::div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .children(task_childrens)
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
