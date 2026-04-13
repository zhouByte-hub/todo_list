use crate::components::{
    category::category_page::CategoryPage, home::home_page::TodoListHome,
    modal::AddTaskModal,
    profile::profile_page::ProfilePage, reminder::reminder_page::ReminderPage,
};
use gpui::{
    AnyView, AppContext, Corners, Div, Edges, ElementId, Hsla, InteractiveElement, ParentElement,
    Render, SharedString, Stateful, StatefulInteractiveElement, Styled, img, px,
};
use gpui_component::{Icon, Sizable, StyledExt, hsl};

pub(crate) struct TodoLayout {
    selected_menu: usize,
    pages: [Option<AnyView>; 4],
    add_task_modal: gpui::Entity<AddTaskModal>,
    home_page: Option<gpui::Entity<TodoListHome>>,
}

impl TodoLayout {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
        let home_page = cx.new(|cx| TodoListHome::new(window, cx));
        let pages: [Option<AnyView>; 4] = [Some(home_page.clone().into()), None, None, None];
        
        let add_task_modal = cx.new(|cx| {
            let mut modal = AddTaskModal::new(window, cx);
            let home_page_clone = home_page.clone();
            modal.set_on_save_success(move |cx| {
                home_page_clone.update(cx, |home, cx| {
                    home.load_task_list(cx);
                });
            });
            modal
        });
        
        Self {
            selected_menu: 0,
            pages,
            add_task_modal,
            home_page: Some(home_page),
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
        if let Some(current_page) = &self.pages[self.selected_menu] {
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
                .child(self.add_task_modal.clone())
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
        div
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
            .child(self.render_add_button(cx))
            .child(self.set_menu("提醒", 2, String::from("icon/tabber/bell-ring.svg"), cx))
            .child(self.set_menu("我的", 3, String::from("icon/tabber/user.svg"), cx))
    }

    fn render_add_button(&self, cx: &gpui::Context<Self>) -> Stateful<Div> {
        gpui::div()
            .id("add_task_button")
            .bottom(px(25.0))
            .bg(hsl(191.0, 98.0, 42.0))
            .border(px(6.5))
            .border_color(Hsla::white())
            .rounded(px(50.0))
            .size(px(65.0))
            .flex()
            .justify_center()
            .items_center()
            .cursor_pointer()
            .hover(|style| style.bg(hsl(191.0, 98.0, 38.0)))
            .active(|style| style.bg(hsl(191.0, 98.0, 35.0)))
            .child(
                Icon::default()
                    .path("icon/tabber/plus.svg")
                    .text_color(Hsla::white())
                    .large(),
            )
            .on_click(cx.listener(|this, _, window, cx| {
                this.add_task_modal.update(cx, |modal, cx| {
                    modal.show(window, cx);
                });
            }))
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
            .on_click(cx.listener(move |this, _event, window, cx| {
                if this.selected_menu != menu_index {
                    this.selected_menu = menu_index;

                    if this.pages[menu_index].is_none() {
                        this.pages[menu_index] = match menu_index {
                            0 => Some(cx.new(|cx| TodoListHome::new(window, cx)).into()),
                            1 => {
                                let category_page = cx.new(|cx| {
                                    let mut page = CategoryPage::default();
                                    page.load_tasks(cx);
                                    page
                                });
                                Some(category_page.into())
                            }
                            2 => {
                                let reminder_page = cx.new(|cx| {
                                    let mut page = ReminderPage::default();
                                    page.load_reminders(cx);
                                    page
                                });
                                Some(reminder_page.into())
                            }
                            3 => Some(cx.new(|_| ProfilePage::default()).into()),
                            _ => None,
                        };
                    }

                    cx.notify();
                }
            }))
    }
}
