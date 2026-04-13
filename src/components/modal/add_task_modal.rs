use std::{env, fs, path::PathBuf};

use anyhow::Result;
use chrono::Local;
use gpui::{
    AppContext, Hsla, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, StatefulInteractiveElement, Styled, px,
};
use gpui_component::{StyledExt, hsl, input::{InputState, Input}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub task_name: String,
    pub priority: String,
    pub create_time: String,
    pub overdue_time: Option<String>,
    pub description: String,
}

pub(crate) struct AddTaskModal {
    task_name_input: gpui::Entity<InputState>,
    priority: String,
    overdue_time_input: gpui::Entity<InputState>,
    description_input: gpui::Entity<InputState>,
    is_visible: bool,
    on_save_success: Option<Box<dyn Fn(&mut gpui::Context<Self>) + 'static>>,
}

impl AddTaskModal {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
        let task_name_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("请输入任务名称...")
        });

        let overdue_time_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("格式: YYYY-MM-DD HH:MM:SS")
        });

        let description_input = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .placeholder("请输入任务描述...")
        });

        Self {
            task_name_input,
            priority: "medium".to_string(),
            overdue_time_input,
            description_input,
            is_visible: false,
            on_save_success: None,
        }
    }

    pub fn set_on_save_success<F: Fn(&mut gpui::Context<Self>) + 'static>(&mut self, callback: F) {
        self.on_save_success = Some(Box::new(callback));
    }

    pub fn show(&mut self, window: &mut gpui::Window, cx: &mut gpui::Context<Self>) {
        self.is_visible = true;
        self.priority = "medium".to_string();
        
        self.task_name_input.update(cx, |state, cx| {
            state.replace("", window, cx);
        });
        self.overdue_time_input.update(cx, |state, cx| {
            state.replace("", window, cx);
        });
        self.description_input.update(cx, |state, cx| {
            state.replace("", window, cx);
        });
        
        cx.notify();
    }

    pub fn hide(&mut self, cx: &mut gpui::Context<Self>) {
        self.is_visible = false;
        cx.notify();
    }

    fn get_task(&self, cx: &gpui::Context<Self>) -> Option<Task> {
        let task_name = self.task_name_input.read(cx).value().trim().to_string();
        if task_name.is_empty() {
            return None;
        }

        let now = Local::now();
        let create_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let overdue_time_str = self.overdue_time_input.read(cx).value().trim().to_string();
        let overdue_time = if overdue_time_str.is_empty() {
            None
        } else {
            Some(overdue_time_str)
        };

        let description = self.description_input.read(cx).value().trim().to_string();

        Some(Task {
            id: now.timestamp_millis(),
            task_name,
            priority: self.priority.clone(),
            create_time,
            overdue_time,
            description,
        })
    }
}

impl Render for AddTaskModal {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        if !self.is_visible {
            return gpui::div().into_any_element();
        }

        gpui::div()
            .absolute()
            .inset_0()
            .child(
                gpui::div()
                    .id("modal_backdrop")
                    .absolute()
                    .inset_0()
                    .bg(hsl(0.0, 0.0, 0.0))
                    .opacity(0.5)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.hide(cx);
                    })),
            )
            .child(
                gpui::div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        gpui::div()
                            .id("modal_content")
                            .w(px(450.0))
                            .bg(Hsla::white())
                            .rounded(px(16.0))
                            .shadow_xl()
                            .flex()
                            .flex_col()
                            .overflow_hidden()
                            .child(self.render_header(cx))
                            .child(self.render_form(cx))
                            .child(self.render_footer(cx)),
                    ),
            )
            .into_any_element()
    }
}

impl AddTaskModal {
    fn render_header(&mut self, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .w_full()
            .h(px(50.0))
            .bg(hsl(210.0, 80.0, 55.0))
            .flex()
            .justify_between()
            .items_center()
            .px_4()
            .child(
                gpui::div()
                    .child("添加新任务")
                    .text_color(Hsla::white())
                    .text_size(px(18.0))
                    .font_bold(),
            )
            .child(
                gpui::div()
                    .id("close_modal_btn")
                    .w(px(30.0))
                    .h(px(30.0))
                    .rounded(px(15.0))
                    .flex()
                    .justify_center()
                    .items_center()
                    .cursor_pointer()
                    .child(
                        gpui::div()
                            .child("✕")
                            .text_color(Hsla::white())
                            .text_size(px(16.0)),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.hide(cx);
                    })),
            )
    }

    fn render_form(&mut self, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        let priority = self.priority.clone();

        gpui::div()
            .w_full()
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .child(self.render_input_field("任务名称", self.task_name_input.clone()))
            .child(self.render_priority_select(&priority, cx))
            .child(self.render_input_field("预计完成时间", self.overdue_time_input.clone()))
            .child(self.render_textarea_field("任务描述", self.description_input.clone()))
    }

    fn render_input_field(
        &mut self,
        label: &str,
        input_state: gpui::Entity<InputState>,
    ) -> impl gpui::IntoElement {
        let label_text = label.to_string();

        gpui::div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                gpui::div()
                    .child(label_text)
                    .text_size(px(14.0))
                    .text_color(hsl(0.0, 0.0, 40.0))
                    .font_semibold(),
            )
            .child(Input::new(&input_state).w_full())
    }

    fn render_priority_select(
        &mut self,
        current_priority: &str,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let priorities = vec![
            ("high", "高优先级", hsl(349.0, 89.0, 58.0)),
            ("medium", "中优先级", hsl(38.0, 92.0, 50.0)),
            ("low", "低优先级", hsl(142.0, 71.0, 45.0)),
        ];

        gpui::div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                gpui::div()
                    .child("任务优先级")
                    .text_size(px(14.0))
                    .text_color(hsl(0.0, 0.0, 40.0))
                    .font_semibold(),
            )
            .child(
                gpui::div()
                    .w_full()
                    .flex()
                    .gap_2()
                    .children(priorities.into_iter().map(|(key, label, color)| {
                        let is_selected = key == current_priority;
                        let key_owned = key.to_string();
                        let label_owned = label.to_string();

                        gpui::div()
                            .id(SharedString::from(format!("priority_{}", key)))
                            .flex_1()
                            .h(px(36.0))
                            .rounded(px(8.0))
                            .border(px(1.5))
                            .border_color(if is_selected { color } else { hsl(0.0, 0.0, 90.0) })
                            .bg(if is_selected {
                                hsl(color.h, color.s, 0.95)
                            } else {
                                Hsla::white()
                            })
                            .flex()
                            .justify_center()
                            .items_center()
                            .cursor_pointer()
                            .child(
                                gpui::div()
                                    .child(label_owned)
                                    .text_size(px(13.0))
                                    .text_color(if is_selected { color } else { hsl(0.0, 0.0, 50.0) })
                                    .font_semibold(),
                            )
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.priority = key_owned.clone();
                                cx.notify();
                            }))
                    })),
            )
    }

    fn render_textarea_field(
        &mut self,
        label: &str,
        input_state: gpui::Entity<InputState>,
    ) -> impl gpui::IntoElement {
        let label_text = label.to_string();

        gpui::div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                gpui::div()
                    .child(label_text)
                    .text_size(px(14.0))
                    .text_color(hsl(0.0, 0.0, 40.0))
                    .font_semibold(),
            )
            .child(Input::new(&input_state).w_full().h(px(80.0)))
    }

    fn render_footer(&mut self, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .w_full()
            .h(px(60.0))
            .px_4()
            .flex()
            .justify_end()
            .items_center()
            .gap_3()
            .border_t(px(1.0))
            .border_color(hsl(0.0, 0.0, 94.0))
            .child(
                gpui::div()
                    .id("cancel_btn")
                    .px_5()
                    .h(px(36.0))
                    .rounded(px(8.0))
                    .border(px(1.0))
                    .border_color(hsl(0.0, 0.0, 88.0))
                    .flex()
                    .justify_center()
                    .items_center()
                    .cursor_pointer()
                    .child(
                        gpui::div()
                            .child("取消")
                            .text_size(px(14.0))
                            .text_color(hsl(0.0, 0.0, 50.0)),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.hide(cx);
                    })),
            )
            .child(
                gpui::div()
                    .id("save_btn")
                    .px_5()
                    .h(px(36.0))
                    .rounded(px(8.0))
                    .bg(hsl(210.0, 80.0, 55.0))
                    .flex()
                    .justify_center()
                    .items_center()
                    .cursor_pointer()
                    .child(
                        gpui::div()
                            .child("保存")
                            .text_size(px(14.0))
                            .text_color(Hsla::white())
                            .font_semibold(),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(task) = this.get_task(cx) {
                            if let Err(e) = save_task_to_file(&task) {
                                eprintln!("Failed to save task: {}", e);
                            } else if let Some(callback) = &this.on_save_success {
                                callback(cx);
                            }
                        }
                        this.hide(cx);
                    })),
            )
    }
}

pub fn save_task_to_file(task: &Task) -> Result<()> {
    let task_list_data_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("data")
        .join("task_list.json");

    let mut tasks: Vec<Task> = if task_list_data_path.exists() {
        let json = fs::read_to_string(&task_list_data_path)?;
        serde_json::from_str(&json)?
    } else {
        vec![]
    };

    tasks.push(task.clone());

    let json = serde_json::to_string_pretty(&tasks)?;
    fs::write(&task_list_data_path, json)?;

    Ok(())
}
