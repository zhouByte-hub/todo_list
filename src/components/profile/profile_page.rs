use gpui::prelude::FluentBuilder;
use gpui::{
    Corners, Hsla, InteractiveElement, ParentElement, Render, StatefulInteractiveElement, Styled,
    px,
};
use gpui_component::{StyledExt, hsl};

use crate::components::interface::PageLayout;

#[derive(Debug, Clone)]
pub(crate) struct ProfilePage {
    user_name: String,
    user_email: String,
    total_tasks: usize,
    completed_tasks: usize,
}

impl Default for ProfilePage {
    fn default() -> Self {
        Self {
            user_name: "用户".to_string(),
            user_email: "user@example.com".to_string(),
            total_tasks: 0,
            completed_tasks: 0,
        }
    }
}

impl Render for ProfilePage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.page_layout(cx)
    }
}

impl PageLayout for ProfilePage {
    fn page_layout(&mut self, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .h_full()
            .flex()
            .flex_col()
            .bg(hsl(210.0, 20.0, 98.0))
            .child(self.render_header())
            .child(self.render_menu_items())
    }
}

impl ProfilePage {
    fn render_header(&mut self) -> impl gpui::IntoElement {
        gpui::div()
            .relative()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .w_full()
            .h(px(160.0))
            .bg(hsl(210.0, 80.0, 55.0))
            .corner_radii(Corners {
                bottom_left: px(20.0),
                bottom_right: px(20.0),
                ..Default::default()
            })
            .child(
                gpui::div()
                    .absolute()
                    .top(px(25.0))
                    .w(px(70.0))
                    .h(px(70.0))
                    .rounded(px(35.0))
                    .bg(Hsla::white())
                    .shadow_md()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        gpui::div()
                            .child("👤")
                            .text_size(px(36.0)),
                    ),
            )
            .child(
                gpui::div()
                    .absolute()
                    .top(px(105.0))
                    .child(self.user_name.clone())
                    .text_color(Hsla::white())
                    .text_size(px(18.0))
                    .font_bold(),
            )
            .child(
                gpui::div()
                    .absolute()
                    .top(px(128.0))
                    .child(self.user_email.clone())
                    .text_color(hsl(0.0, 0.0, 95.0))
                    .text_size(px(13.0)),
            )
    }

    fn render_menu_items(&mut self) -> impl gpui::IntoElement {
        gpui::div()
            .id("profile_page_scrollable_container")
            .flex_1()
            .w_full()
            .px_4()
            .pt_4()
            .pb_20()
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .gap_4()
            .child(self.render_stats_card())
            .child(self.render_menu_section("账户设置".to_string(), vec![
                ("个人资料".to_string(), "修改你的个人信息".to_string()),
                ("账户安全".to_string(), "密码和安全设置".to_string()),
                ("通知设置".to_string(), "管理通知偏好".to_string()),
            ]))
            .child(self.render_menu_section("应用设置".to_string(), vec![
                ("主题设置".to_string(), "选择你喜欢的主题".to_string()),
                ("语言设置".to_string(), "选择应用语言".to_string()),
                ("数据管理".to_string(), "备份和恢复数据".to_string()),
            ]))
            .child(self.render_menu_section("关于".to_string(), vec![
                ("关于我们".to_string(), "了解更多信息".to_string()),
                ("帮助与反馈".to_string(), "获取帮助或提交反馈".to_string()),
                ("版本信息".to_string(), "当前版本 v0.1.0".to_string()),
            ]))
    }

    fn render_stats_card(&mut self) -> impl gpui::IntoElement {
        let completion_rate = if self.total_tasks > 0 {
            (self.completed_tasks as f32 / self.total_tasks as f32) * 100.0
        } else {
            0.0
        };

        gpui::div()
            .w_full()
            .bg(Hsla::white())
            .rounded(px(12.0))
            .p_4()
            .shadow_sm()
            .flex()
            .justify_around()
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(
                        gpui::div()
                            .child(format!("{}", self.total_tasks))
                            .text_size(px(22.0))
                            .font_bold()
                            .text_color(hsl(210.0, 80.0, 55.0)),
                    )
                    .child(
                        gpui::div()
                            .child("总任务")
                            .text_size(px(12.0))
                            .text_color(hsl(0.0, 0.0, 55.0)),
                    ),
            )
            .child(
                gpui::div()
                    .w(px(1.0))
                    .h(px(36.0))
                    .bg(hsl(0.0, 0.0, 92.0)),
            )
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(
                        gpui::div()
                            .child(format!("{}", self.completed_tasks))
                            .text_size(px(22.0))
                            .font_bold()
                            .text_color(hsl(160.0, 60.0, 45.0)),
                    )
                    .child(
                        gpui::div()
                            .child("已完成")
                            .text_size(px(12.0))
                            .text_color(hsl(0.0, 0.0, 55.0)),
                    ),
            )
            .child(
                gpui::div()
                    .w(px(1.0))
                    .h(px(36.0))
                    .bg(hsl(0.0, 0.0, 92.0)),
            )
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(
                        gpui::div()
                            .child(format!("{:.0}%", completion_rate))
                            .text_size(px(22.0))
                            .font_bold()
                            .text_color(hsl(30.0, 80.0, 55.0)),
                    )
                    .child(
                        gpui::div()
                            .child("完成率")
                            .text_size(px(12.0))
                            .text_color(hsl(0.0, 0.0, 55.0)),
                    ),
            )
    }

    fn render_menu_section(&mut self, title: String, items: Vec<(String, String)>) -> impl gpui::IntoElement {
        gpui::div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                gpui::div()
                    .child(title)
                    .text_size(px(13.0))
                    .font_semibold()
                    .text_color(hsl(0.0, 0.0, 45.0))
                    .ml_1(),
            )
            .child(
                gpui::div()
                    .w_full()
                    .bg(Hsla::white())
                    .rounded(px(12.0))
                    .shadow_sm()
                    .flex()
                    .flex_col()
                    .children(items.iter().enumerate().map(|(index, (label, desc))| {
                        let is_last = index == items.len() - 1;
                        gpui::div()
                            .w_full()
                            .px_4()
                            .py_3()
                            .flex()
                            .justify_between()
                            .items_center()
                            .when(!is_last, |this| {
                                this.border_b(px(0.5))
                                    .border_color(hsl(0.0, 0.0, 94.0))
                            })
                            .child(
                                gpui::div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        gpui::div()
                                            .child(label.clone())
                                            .text_size(px(14.0))
                                            .text_color(hsl(0.0, 0.0, 25.0)),
                                    )
                                    .child(
                                        gpui::div()
                                            .child(desc.clone())
                                            .text_size(px(11.0))
                                            .text_color(hsl(0.0, 0.0, 55.0)),
                                    ),
                            )
                            .child(
                                gpui::div()
                                    .child("›")
                                    .text_size(px(18.0))
                                    .text_color(hsl(0.0, 0.0, 75.0)),
                            )
                    })),
            )
    }
}
