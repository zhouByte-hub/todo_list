use std::{collections::HashMap, env, fs, path::PathBuf};

use anyhow::Result;
use gpui::{
    Corners, Hsla, ParentElement, Render, Styled,
    px, relative,
};
use gpui_component::{StyledExt, hsl, progress::Progress, scroll::ScrollableElement};
use serde::{Deserialize, Serialize};

use crate::components::interface::PageLayout;

lazy_static::lazy_static! {
    static ref CATEGORY_COLORS: HashMap<&'static str, Hsla> = {
        let mut map = HashMap::new();
        map.insert("work", hsl(217.0, 91.0, 60.0));
        map.insert("study", hsl(142.0, 71.0, 45.0));
        map.insert("life", hsl(262.0, 83.0, 58.0));
        map.insert("health", hsl(349.0, 89.0, 58.0));
        map.insert("other", hsl(0.0, 0.0, 59.0));
        map
    };
    
    static ref CATEGORY_ICONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("work", "💼");
        map.insert("study", "📚");
        map.insert("life", "🏠");
        map.insert("health", "💪");
        map.insert("other", "📌");
        map
    };
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Task {
    id: i64,
    task_name: String,
    priority: String,
    create_time: String,
    overdue_time: Option<String>,
    description: String,
    category: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Category {
    id: String,
    name: String,
    icon: String,
    color: String,
}

#[derive(Debug, Clone)]
pub(crate) struct CategoryPage {
    categories: Vec<Category>,
    tasks: Vec<Task>
}


impl Default for CategoryPage {
    fn default() -> Self {
        Self {
            categories: vec![
                Category {
                    id: "work".to_string(),
                    name: "工作".to_string(),
                    icon: "💼".to_string(),
                    color: "blue".to_string(),
                },
                Category {
                    id: "study".to_string(),
                    name: "学习".to_string(),
                    icon: "📚".to_string(),
                    color: "green".to_string(),
                },
                Category {
                    id: "life".to_string(),
                    name: "生活".to_string(),
                    icon: "🏠".to_string(),
                    color: "purple".to_string(),
                },
                Category {
                    id: "health".to_string(),
                    name: "健康".to_string(),
                    icon: "💪".to_string(),
                    color: "red".to_string(),
                },
                Category {
                    id: "other".to_string(),
                    name: "其他".to_string(),
                    icon: "📌".to_string(),
                    color: "gray".to_string(),
                },
            ],
            tasks: vec![]
        }
    }
}

impl Render for CategoryPage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.page_layout(cx)
    }
}

impl PageLayout for CategoryPage {
    fn page_layout(&mut self, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .h_full()
            .flex()
            .flex_col()
            .bg(hsl(60.0, 22.0, 96.0))
            .child(self.render_header())
            .child(self.render_categories())
    }
}

impl CategoryPage {
    fn render_header(&mut self) -> impl gpui::IntoElement {
        gpui::div()
            .relative()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .w_full()
            .h(px(120.0))
            .bg(hsl(262.0, 83.0, 58.0))
            .corner_radii(Corners {
                bottom_left: px(25.0),
                bottom_right: px(25.0),
                ..Default::default()
            })
            .child(
                gpui::div()
                    .absolute()
                    .top(px(5.0))
                    .child("任务分类")
                    .text_color(Hsla::white())
                    .text_size(px(24.0))
                    .font_bold()
                    .p_1(),
            )
            .child(
                gpui::div()
                    .absolute()
                    .top(px(45.0))
                    .child("按类别管理你的任务")
                    .text_color(hsl(0.0, 0.0, 90.0))
                    .text_size(px(14.0)),
            )
            .child(
                gpui::div()
                    .absolute()
                    .bottom(px(0.0))
                    .flex()
                    .gap_4()
                    .child(
                        gpui::div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(
                                gpui::div()
                                    .child(format!("{}", self.categories.len()))
                                    .text_color(Hsla::white())
                                    .text_size(px(20.0))
                                    .font_bold(),
                            )
                            .child(
                                gpui::div()
                                    .child("分类数")
                                    .text_color(hsl(0.0, 0.0, 90.0))
                                    .text_size(px(12.0)),
                            ),
                    )
                    .child(
                        gpui::div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(
                                gpui::div()
                                    .child(format!("{}", self.tasks.len()))
                                    .text_color(Hsla::white())
                                    .text_size(px(20.0))
                                    .font_bold(),
                            )
                            .child(
                                gpui::div()
                                    .child("总任务")
                                    .text_color(hsl(0.0, 0.0, 90.0))
                                    .text_size(px(12.0)),
                            ),
                    ),
            )
    }

    fn render_categories(&mut self) -> impl gpui::IntoElement {
        let categories = self.categories.clone();
        let tasks = self.tasks.clone();
        
        gpui::div()
            .flex_1()
            .w_full()
            .p_5()
            .mt_4()
            .overflow_y_scrollbar()
            .flex()
            .flex_col()
            .gap_4()
            .children(categories.iter().map(|category| {
                let category_tasks: Vec<&Task> = tasks
                    .iter()
                    .filter(|t| {
                        t.category.as_ref().unwrap_or(&"other".to_string()) == &category.id
                    })
                    .collect();
                let total = category_tasks.len();
                let completed = category_tasks.iter().filter(|t| t.overdue_time.is_none()).count();
                let progress = if total > 0 {
                    completed as f32 / total as f32
                } else {
                    0.0
                };

                let color = CATEGORY_COLORS
                    .get(category.id.as_str())
                    .copied()
                    .unwrap_or_else(|| hsl(0.0, 0.0, 59.0));

                gpui::div()
                    .w_full()
                    .bg(Hsla::white())
                    .rounded(px(16.0))
                    .p_5()
                    .mb_3()
                    .shadow_sm()
                    .flex()
                    .items_center()
                    .gap_5()
                    .child(
                        gpui::div()
                            .w(px(56.0))
                            .h(px(56.0))
                            .rounded(px(14.0))
                            .bg(color)
                            .flex()
                            .justify_center()
                            .items_center()
                            .child(
                                gpui::div()
                                    .child(category.icon.clone())
                                    .text_size(px(26.0)),
                            ),
                    )
                    .child(
                        gpui::div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                gpui::div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        gpui::div()
                                            .child(category.name.clone())
                                            .text_size(px(16.0))
                                            .font_bold()
                                            .text_color(hsl(0.0, 0.0, 20.0)),
                                    )
                                    .child(
                                        gpui::div()
                                            .child(format!("{} 项任务", total))
                                            .text_size(px(12.0))
                                            .text_color(hsl(0.0, 0.0, 60.0)),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Progress::new()
                                            .value(progress)
                                            .bg(color)
                                            .w(relative(0.6)),
                                    )
                                    .child(
                                        gpui::div()
                                            .child(format!("{:.0}%", progress * 100.0))
                                            .text_size(px(12.0))
                                            .text_color(hsl(0.0, 0.0, 60.0)),
                                    ),
                            ),
                    )
            }))
    }

    pub fn load_tasks(&mut self, cx: &mut gpui::Context<Self>) {
        cx.spawn(async move |weak_entity, cx| -> Result<()> {
            let task_list_data_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
                .join("data")
                .join("task_list.json");

            if !task_list_data_path.exists() {
                return Ok(());
            }

            let list_json = fs::read_to_string(&task_list_data_path)?;
            let mut task_list: Vec<Task> = serde_json::from_str(&list_json)?;

            for task in &mut task_list {
                if task.category.is_none() {
                    task.category = Some("other".to_string());
                }
            }
            weak_entity.update(cx, |entity, _cx| {
                entity.tasks = task_list;
            })?;

            Ok(())
        })
        .detach();
    }
}
