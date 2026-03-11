use std::{env, fs, path::PathBuf};

use anyhow::Result;
use getset::{CloneGetters, Getters, MutGetters, Setters, WithSetters};
use gpui::{ElementId, Hsla, ParentElement, Render, SharedString, Styled, px};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    hsl,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub(crate) struct HomeMenu {
    home_menus: Vec<MenuItem>,
    select_menu: usize,
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
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        println!("render menu");
        let default_button_variant = ButtonCustomVariant::new(cx)
            .color(Hsla::white())
            .hover(hsl(40.0, 5.0, 88.0))
            .active(hsl(189.0, 91.0, 40.0));

        let menu_container = gpui::div().p_2().mt(px(80.0)).w_full().h_flex().gap_2();

        // 异步加载菜单数据
        cx.spawn(async move |weak_entity, cx| -> Result<()> {
            let config_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
                .join("config")
                .join("home_menu.json");

            let json_content = fs::read_to_string(config_path)?;
            let home_menus: Vec<MenuItem> = serde_json::from_str(&json_content)?;
            weak_entity.update(cx, |entity, _cx| {
                eprintln!("{:?}", home_menus);
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
}

impl HomeMenu {
    pub fn new() -> Self {
        Self {
            home_menus: vec![],
            select_menu: 0,
        }
    }
}
