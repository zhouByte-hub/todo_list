use std::{env, fs, path::PathBuf};

use anyhow::Result;
use getset::{CloneGetters, Getters, MutGetters, Setters, WithSetters};
use gpui::{AppContext, ElementId, Hsla, ParentElement, Render, SharedString, Styled, px};
use gpui_component::{
    Sizable, StyledExt, button::{Button, ButtonCustomVariant, ButtonVariants}, hsl, input::{Input, InputState}
};
use serde::{Deserialize, Serialize};

pub(crate) struct HomeMenu {
    home_menus: Vec<MenuItem>,
    select_menu: usize,
    input_content: Option<SharedString>,
    input_state: Option<gpui::Entity<InputState>>,
    subscription: Option<gpui::Subscription>,
}

#[derive(
    Debug, Clone, Deserialize, Serialize, Getters, Setters, WithSetters, MutGetters, CloneGetters,
)]
struct MenuItem {
    #[getset(get = "pub", set = "pub", get_mut = "pub", set_with = "pub")]
    id: usize,

    #[getset(get = "pub", set = "pub", get_mut = "pub", set_with = "pub")]
    title: String,

    #[getset(get = "pub", set = "pub", get_mut = "pub", set_with = "pub")]
    category: String,
}

impl Render for HomeMenu {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div().child(self.init_menu(cx)).child(self.init_search_bar(window, cx))
    }
        
}

impl HomeMenu {
    pub fn new() -> Self {
        Self {
            home_menus: vec![],
            select_menu: 0,
            input_content: None,
            input_state: None,
            subscription: None,
        }
    }

    fn init_menu(&mut self, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        let default_button_variant = ButtonCustomVariant::new(cx)
            .color(Hsla::white())
            .hover(hsl(40.0, 5.0, 88.0))
            .active(hsl(189.0, 91.0, 40.0));

        let menu_container = gpui::div().p_2().mt(px(60.0)).w_full().h_flex().gap_2();

        // 异步加载菜单数据
        cx.spawn(async move |weak_entity, cx| -> Result<()> {
            let config_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
                .join("config")
                .join("home_menu.json");

            let json_content = fs::read_to_string(config_path)?;
            let home_menus: Vec<MenuItem> = serde_json::from_str(&json_content)?;
            weak_entity.update(cx, |entity, _cx| {
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
                .h(px(40.0))
                .rounded(px(20.0));
            if index == self.select_menu {
                let selected_variant = ButtonCustomVariant::new(cx)
                    .color(hsl(189.0, 91.0, 40.0))
                    .foreground(Hsla::white())
                    .hover(hsl(189.0, 91.0, 40.0));
                btn = btn.custom(selected_variant);
            } else {
                btn = btn.custom(default_button_variant);
            }
            btn = btn.on_click(cx.listener(move |this, _event, _window, cx| {
                this.select_menu = index;
                cx.notify();
            }));
            childrens.push(btn);
        }
        menu_container.children(childrens)
    }


    fn init_search_bar(&mut self, window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        if self.input_state.is_none() {
            let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("请输入任务标题"));
            let subscription = cx.subscribe_in(&input_state, window, |view, state, event, _window, cx| {
                match event {
                    gpui_component::input::InputEvent::Change => {
                        let text = state.read(cx).value();
                        view.input_content = Some(text);
                    }
                    gpui_component::input::InputEvent::PressEnter { secondary } => {
                        println!("Enter pressed, secondary: {}", secondary);
                    }
                    gpui_component::input::InputEvent::Focus => println!("Input focused"),
                    gpui_component::input::InputEvent::Blur => println!("Input blurred"),
                }
            });
            self.input_state = Some(input_state);
            self.subscription = Some(subscription);
        }
        
        Input::new(self.input_state.as_ref().unwrap()).large().cleanable(true)
    }
}
