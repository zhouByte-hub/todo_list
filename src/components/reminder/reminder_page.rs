use gpui::{ParentElement, Render, Styled, div};

#[derive(Default, Debug, Clone)]
pub(crate) struct ReminderPage {}

impl Render for ReminderPage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().w_full().h_full().child("提醒内容")
    }
}
