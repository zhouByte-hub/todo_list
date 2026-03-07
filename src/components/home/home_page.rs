use gpui::{Corners, Hsla, ParentElement, Render, Styled, px};
use gpui_component::{StyledExt, hsl, progress::Progress};
use chrono::{Local, Weekday, Datelike};

use crate::components::interface::PageLayout;

#[derive(Default, Debug, Clone)]
pub(crate) struct TodoListHome {
    progress: f32,
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
    fn page_layout(&mut self, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div().child(self.header(cx))
    }
}

impl TodoListHome {
    fn header(&mut self, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .relative()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .w_full()
            .h(px(150.0))
            .bg(hsl(189.0, 91.0, 40.0))
            .corner_radii(Corners {
                bottom_left: px(25.0),
                bottom_right: px(25.0),
                ..Default::default()
            })
            .child(self.header_title(cx))
            .child(self.header_box(cx))
    }

    fn header_title(&mut self, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .absolute()
            .top(px(20.0))
            .child("Todo List")
            .text_color(Hsla::white())
            .text_size(px(24.0))
            .font_bold()
            .p_1()
    }

    fn header_box(&mut self, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        let today_weekday = Local::now().weekday();
        let weekdays = ["日", "一", "二", "三", "四", "五", "六"];
        
        gpui::div()
            .absolute()
            .w_4_5()
            .h(px(120.0))
            .bg(Hsla::white())
            .rounded(px(10.0))
            .top(px(90.0))
            .shadow_sm()
            .flex()
            .flex_col()
            .p(px(15.0))
            .gap_3()
            .child(
                gpui::div()
                    .flex()
                    .justify_between()
                    .child("周")
                    .child("待办任务")
                    .text_color(hsl(0.0, 0.0, 47.0))
            )
            .child(
                gpui::div()
                    .flex()
                    .justify_between()
                    .children(
                        weekdays.iter().enumerate().map(|(index, day)| {
                            let is_today = match today_weekday {
                                Weekday::Sun => index == 0,
                                Weekday::Mon => index == 1,
                                Weekday::Tue => index == 2,
                                Weekday::Wed => index == 3,
                                Weekday::Thu => index == 4,
                                Weekday::Fri => index == 5,
                                Weekday::Sat => index == 6,
                            };
                            
                            if is_today {
                                gpui::div()
                                    .child(*day)
                                    .text_color(hsl(25.0, 100.0, 49.0))
                                    .font_bold()
                            } else {
                                gpui::div().child(*day)
                            }
                        })
                    ),
            )
            .child(Progress::new().value(self.progress).bg(hsl(25.0, 100.0, 49.0)))
    }
}
