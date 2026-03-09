use anyhow::Result;
use chrono::{Datelike, Local, Weekday};
use gpui::{Corners, ElementId, Hsla, ParentElement, Render, SharedString, Styled, px};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    hsl,
    progress::Progress,
};

use crate::components::{
    home::home_config::{HomeMenu, read_home_menu},
    interface::PageLayout,
};

#[derive(Default, Debug, Clone)]
pub(crate) struct TodoListHome {
    progress: f32,
    select_menu: usize,
    home_menus: Vec<HomeMenu>,
    _input_content: String,
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
        gpui::div().child(self.header(cx)).child(self.menu(cx))
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
                    .text_color(hsl(0.0, 0.0, 47.0)),
            )
            .child(
                gpui::div()
                    .flex()
                    .justify_between()
                    .children(weekdays.iter().enumerate().map(|(index, day)| {
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
                    })),
            )
            .child(
                Progress::new()
                    .value(self.progress)
                    .bg(hsl(25.0, 100.0, 49.0)),
            )
    }

    fn menu(&mut self, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        let default_button_variant = ButtonCustomVariant::new(cx)
            .color(Hsla::white())
            .hover(hsl(40.0, 5.0, 88.0))
            .active(hsl(189.0, 91.0, 40.0));

        let menu_container = gpui::div().p_2().mt(px(80.0)).w_full().flex().gap_3();

        // 异步加载菜单数据
        cx.spawn(async move |weak_entity, cx| -> Result<()> {
            let home_menus = read_home_menu()?;
            weak_entity.update(cx, |entity: &mut TodoListHome, _cx| {
                entity.home_menus = home_menus;
            })?;
            Ok(())
        })
        .detach();
        let mut childrens = vec![];
        for (index, item) in self.home_menus.iter().enumerate() {
            let btn_id = ElementId::Name(SharedString::new(format!("btn-{:?}", item.id())));
            let mut btn = Button::new(btn_id)
                .label(item.title())
                .w(px(80.0))
                .h(px(40.0));
            if index == self.select_menu {
                let selected_variant = ButtonCustomVariant::new(cx).color(hsl(189.0, 91.0, 40.0));
                btn = btn.custom(selected_variant);
            }else {
                btn = btn.custom(default_button_variant);
            }
            childrens.push(btn);
        }
        menu_container.children(childrens)
    }
}
