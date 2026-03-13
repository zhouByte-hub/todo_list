use gpui::{AppContext, ParentElement, Render};

use crate::components::{
    home::{header::HomeHeader, menu::HomeMenu},
    interface::PageLayout,
};

#[derive(Debug, Clone)]
pub(crate) struct TodoListHome {
    home_header: Option<gpui::Entity<HomeHeader>>,
    home_menu: Option<gpui::Entity<HomeMenu>>,
}

impl Render for TodoListHome {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        // cx.new会在每次渲染都创建新实体，每次新创建的实体都会导致原本的上下文失效。
        if self.home_header.is_none() {
            self.home_header = Some(cx.new(|_cx| HomeHeader::new()));
        }
        if self.home_menu.is_none() {
            self.home_menu = Some(cx.new(|_cx| HomeMenu::new()));
        }
        self.page_layout(cx)
    }
}

impl PageLayout for TodoListHome {
    fn page_layout(&mut self, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        let mut layout = gpui::div();
        if let Some(header) = &self.home_header {
            layout = layout.child((*header).clone());
        }
        if let Some(menu) = &self.home_menu {
            layout = layout.child((*menu).clone());
        }
        layout
    }
}

impl Default for TodoListHome {

    fn default() -> Self {
        Self {
            home_header: None,
            home_menu: None,
        }
    }

}
