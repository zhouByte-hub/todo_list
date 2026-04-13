use std::{collections::HashMap, env, fs, path::PathBuf};

use anyhow::Result;
use chrono::{Local, TimeZone};
use gpui::{
    Corners, Hsla, ParentElement, Render, SharedString, Styled,
    px, relative,
};
use gpui_component::{StyledExt, hsl, scroll::ScrollableElement};
use serde::{Deserialize, Serialize};

use crate::components::interface::PageLayout;

lazy_static::lazy_static! {
    static ref PRIORITY_COLORS: HashMap<&'static str, Hsla> = {
        let mut map = HashMap::new();
        map.insert("high", hsl(349.0, 89.0, 58.0));
        map.insert("medium", hsl(38.0, 92.0, 50.0));
        map.insert("low", hsl(142.0, 71.0, 45.0));
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
}

#[derive(Debug, Clone)]
struct Reminder {
    task: Task,
    time_until: String,
    is_overdue: bool,
    days_remaining: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct ReminderPage {
    reminders: Vec<Reminder>,
    upcoming_count: usize,
    overdue_count: usize,
}

impl Default for ReminderPage {
    fn default() -> Self {
        Self {
            reminders: vec![],
            upcoming_count: 0,
            overdue_count: 0,
        }
    }
}

impl Render for ReminderPage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.page_layout(cx)
    }
}

impl PageLayout for ReminderPage {
    fn page_layout(&mut self, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .h_full()
            .flex()
            .flex_col()
            .bg(hsl(60.0, 22.0, 96.0))
            .child(self.render_header())
            .child(self.render_reminders())
    }
}

impl ReminderPage {
    fn render_header(&mut self) -> impl gpui::IntoElement {
        gpui::div()
            .relative()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .w_full()
            .h(px(120.0))
            .bg(hsl(25.0, 100.0, 49.0))
            .corner_radii(Corners {
                bottom_left: px(25.0),
                bottom_right: px(25.0),
                ..Default::default()
            })
            .child(
                gpui::div()
                    .absolute()
                    .top(px(5.0))
                    .child("任务提醒")
                    .text_color(Hsla::white())
                    .text_size(px(24.0))
                    .font_bold()
                    .p_1(),
            )
            .child(
                gpui::div()
                    .absolute()
                    .top(px(45.0))
                    .child("不要错过重要的截止日期")
                    .text_color(hsl(0.0, 0.0, 90.0))
                    .text_size(px(14.0)),
            )
            .child(
                gpui::div()
                    .absolute()
                    .bottom(px(0.0))
                    .flex()
                    .gap_6()
                    .child(
                        gpui::div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(
                                gpui::div()
                                    .child(format!("{}", self.upcoming_count))
                                    .text_color(Hsla::white())
                                    .text_size(px(20.0))
                                    .font_bold(),
                            )
                            .child(
                                gpui::div()
                                    .child("即将到期")
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
                                    .child(format!("{}", self.overdue_count))
                                    .text_color(Hsla::white())
                                    .text_size(px(20.0))
                                    .font_bold(),
                            )
                            .child(
                                gpui::div()
                                    .child("已逾期")
                                    .text_color(hsl(0.0, 0.0, 90.0))
                                    .text_size(px(12.0)),
                            ),
                    ),
            )
    }

    fn render_reminders(&mut self) -> impl gpui::IntoElement {
        let reminders = self.reminders.clone();

        gpui::div()
            .flex_1()
            .w_full()
            .p_5()
            // .mb_2()
            // .mt_2()
            .overflow_y_scrollbar()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                gpui::div()
                    .w_full()
                    .flex()
                    .mb_3()
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
                    .child(SharedString::new("提醒列表"))
                    .text_color(hsl(0.0, 0.0, 76.0))
                    .child(
                        gpui::div()
                            .w(relative(0.35))
                            .h_0p5()
                            .bg(hsl(0.0, 0.0, 92.0))
                            .ml(px(12.0)),
                    ),
            )
            .children(reminders.iter().map(|reminder| {
                let priority_color = PRIORITY_COLORS
                    .get(reminder.task.priority.as_str())
                    .copied()
                    .unwrap_or_else(|| hsl(0.0, 0.0, 59.0));

                let (bg_color, border_color) = if reminder.is_overdue {
                    (hsl(349.0, 100.0, 97.0), hsl(349.0, 89.0, 58.0))
                } else if reminder.days_remaining <= 1 {
                    (hsl(38.0, 100.0, 97.0), hsl(38.0, 92.0, 50.0))
                } else {
                    (Hsla::white(), hsl(0.0, 0.0, 90.0))
                };

                let status_text = if reminder.is_overdue {
                    "已逾期".to_string()
                } else if reminder.days_remaining == 0 {
                    "今天到期".to_string()
                } else if reminder.days_remaining == 1 {
                    "明天到期".to_string()
                } else {
                    reminder.time_until.clone()
                };

                gpui::div()
                    .w_full()
                    .bg(bg_color)
                    .rounded(px(16.0))
                    .p_5()
                    .mb_3()
                    .border(px(1.0))
                    .border_color(border_color)
                    .shadow_sm()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        gpui::div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                gpui::div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        gpui::div()
                                            .w(px(8.0))
                                            .h(px(8.0))
                                            .rounded(px(4.0))
                                            .bg(priority_color),
                                    )
                                    .child(
                                        gpui::div()
                                            .child(reminder.task.task_name.clone())
                                            .text_size(px(16.0))
                                            .font_bold()
                                            .text_color(hsl(0.0, 0.0, 20.0)),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(6.0))
                                    .bg(if reminder.is_overdue {
                                        hsl(349.0, 89.0, 58.0)
                                    } else if reminder.days_remaining <= 1 {
                                        hsl(38.0, 92.0, 50.0)
                                    } else {
                                        hsl(142.0, 71.0, 45.0)
                                    })
                                    .child(status_text)
                                    .text_color(Hsla::white())
                                    .text_size(px(12.0)),
                            ),
                    )
                    .child(
                        gpui::div()
                            .child(reminder.task.description.clone())
                            .text_size(px(14.0))
                            .text_color(hsl(0.0, 0.0, 60.0)),
                    )
                    .child(
                        gpui::div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                gpui::div()
                                    .child(format!(
                                        "截止: {}",
                                        reminder.task.overdue_time.as_ref().unwrap_or(&"未设置".to_string())
                                    ))
                                    .text_size(px(12.0))
                                    .text_color(hsl(0.0, 0.0, 80.0)),
                            )
                            .child(
                                gpui::div()
                                    .flex()
                                    .gap_1()
                                    .child(
                                        gpui::div()
                                            .px_2()
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(hsl(0.0, 0.0, 95.0))
                                            .child(match reminder.task.priority.as_str() {
                                                "high" => "高优先级",
                                                "medium" => "中优先级",
                                                _ => "低优先级",
                                            })
                                            .text_size(px(10.0))
                                            .text_color(priority_color),
                                    ),
                            ),
                    )
            }))
    }

    #[allow(deprecated)]
    pub fn load_reminders(&mut self, cx: &mut gpui::Context<Self>) {
        cx.spawn(async move |weak_entity, cx| -> Result<()> {
            let task_list_data_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
                .join("data")
                .join("task_list.json");

            if !task_list_data_path.exists() {
                return Ok(());
            }

            let list_json = fs::read_to_string(&task_list_data_path)?;
            let task_list: Vec<Task> = serde_json::from_str(&list_json)?;

            let now = Local::now();
            let mut reminders: Vec<Reminder> = task_list
                .into_iter()
                .filter_map(|task| {
                    task.overdue_time.as_ref()?;
                    let overdue_time = task.overdue_time.clone()?;

                    let overdue_date = Local
                        .datetime_from_str(&overdue_time, "%Y-%m-%d %H:%M:%S")
                        .ok()?;

                    let duration = overdue_date.signed_duration_since(now);
                    let days_remaining = duration.num_days();
                    let is_overdue = days_remaining < 0;

                    let time_until = if is_overdue {
                        format!("逾期 {} 天", days_remaining.abs())
                    } else if days_remaining == 0 {
                        let hours = duration.num_hours();
                        if hours > 0 {
                            format!("{} 小时后", hours)
                        } else {
                            "今天到期".to_string()
                        }
                    } else {
                        format!("{} 天后", days_remaining)
                    };

                    Some(Reminder {
                        task,
                        time_until,
                        is_overdue,
                        days_remaining,
                    })
                })
                .collect();

            reminders.sort_by(|a, b| {
                a.days_remaining.cmp(&b.days_remaining)
            });

            let upcoming_count = reminders.iter().filter(|r| !r.is_overdue).count();
            let overdue_count = reminders.iter().filter(|r| r.is_overdue).count();

            weak_entity.update(cx, |entity, _cx| {
                entity.reminders = reminders;
                entity.upcoming_count = upcoming_count;
                entity.overdue_count = overdue_count;
            })?;

            Ok(())
        })
        .detach();
    }
}
