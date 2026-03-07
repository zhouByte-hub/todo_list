use crate::components::{
    category::category_page::CategoryPage, home::home_page::TodoListHome,
    profile::profile_page::ProfilePage, reminder::reminder_page::ReminderPage,
};
use gpui::{
    AnyView, AppContext, Corners, Div, Edges, ElementId, Hsla, InteractiveElement, ParentElement,
    Render, SharedString, Stateful, StatefulInteractiveElement, Styled, img, px,
};
use gpui_component::{Icon, Sizable, StyledExt, hsl};

#[derive(Debug)]
pub(crate) struct TodoLayout {
    selected_menu: usize, // 0: 首页，1：分类，2：提醒，3：我的
    current_page: Option<AnyView>,
}

impl TodoLayout {
    pub fn default(current_page: Option<AnyView>) -> Self {
        Self {
            selected_menu: 0,
            current_page,
        }
    }
}

impl Render for TodoLayout {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let div: Div;
        if let Some(current_page) = &self.current_page {
            div = gpui::div()
                .bg(hsl(60.0, 22.0, 96.0))
                .h_full()
                .relative()
                .flex()
                .flex_col()
                .child(
                    gpui::div()
                        .flex_1()
                        .overflow_hidden()
                        .child(current_page.clone()),
                )
                .child(self.tabber(cx))
        } else {
            let error_image = img("images/app_error.png").w_full().h(px(250.0));
            div = gpui::div()
                .flex()
                .gap_1()
                .flex_col()
                .justify_center()
                .items_center()
                .h_full()
                .child(error_image)
                .child(SharedString::new("程序出现错误了哦！！！"))
                .text_size(px(24.0))
                .text_color(hsl(0.0, 0.0, 58.0));
        }
        return div;
    }
}

impl TodoLayout {
    fn tabber(&mut self, cx: &gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .w_full()
            .paddings(Edges {
                top: px(0.0),
                bottom: px(0.0),
                left: px(30.0),
                right: px(30.0),
            })
            .h(px(50.0))
            .absolute()
            .bottom_0()
            .left_0()
            .flex()
            .flex_row()
            .justify_between()
            .bg(Hsla::white())
            .corner_radii(Corners {
                top_left: px(10.0),
                top_right: px(10.0),
                ..Default::default()
            })
            .shadow_sm()
            .child(self.set_menu("首页", 0, String::from("icon/tabber/house.svg"), cx))
            .child(self.set_menu(
                "分类",
                1,
                String::from("icon/tabber/layout-dashboard.svg"),
                cx,
            ))
            .child(
                gpui::div()
                    .bottom(px(25.0))
                    .bg(hsl(191.0, 98.0, 42.0))
                    .border(px(6.5))
                    .border_color(Hsla::white())
                    .rounded(px(50.0))
                    .size(px(65.0))
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        Icon::default()
                            .path("icon/tabber/plus.svg")
                            .text_color(Hsla::white())
                            .large(),
                    ),
            )
            .child(self.set_menu("提醒", 2, String::from("icon/tabber/bell-ring.svg"), cx))
            .child(self.set_menu("我的", 3, String::from("icon/tabber/user.svg"), cx))
    }

    fn set_selected_menu_color(&self, menu_index: usize, current_menu_index: usize) -> Hsla {
        if menu_index == current_menu_index {
            hsl(191.0, 98.0, 42.0)
        } else {
            hsl(0.0, 0.0, 59.0)
        }
    }

    fn set_menu(
        &self,
        label: &str,
        menu_index: usize,
        icon: String,
        cx: &gpui::Context<Self>,
    ) -> Stateful<Div> {
        let element_id = ElementId::Name(SharedString::from(format!("menu-{}", menu_index)));
        gpui::div()
            .id(element_id)
            .child(Icon::default().path(icon).large())
            .child(
                gpui::div()
                    .child(SharedString::new(label))
                    .text_size(px(12.0)),
            )
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .bg(gpui::transparent_black())
            .hover(|style| style.bg(gpui::transparent_black()))
            .active(|style| style.bg(gpui::transparent_black()))
            .cursor_pointer()
            .text_color(self.set_selected_menu_color(menu_index, self.selected_menu))
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.selected_menu = menu_index;
                this.current_page = match menu_index {
                    0 => Some(cx.new(|_| TodoListHome::default()).into()),
                    1 => Some(cx.new(|_| CategoryPage::default()).into()),
                    2 => Some(cx.new(|_| ReminderPage::default()).into()),
                    3 => Some(cx.new(|_| ProfilePage::default()).into()),
                    _ => None,
                };
                cx.notify();
            }))
    }
}
