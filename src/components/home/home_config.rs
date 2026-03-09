use std::{env, fs, path::PathBuf};

use anyhow::Result;
use getset::{CloneGetters, Getters, MutGetters, Setters, WithSetters};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Deserialize, Serialize, Getters, Setters, WithSetters, MutGetters, CloneGetters,
)]
pub(crate) struct HomeMenu {
    #[getset(get = "pub", set = "pub", get_mut = "pub", set_with = "pub")]
    id: usize,

    #[getset(get = "pub", set = "pub", get_mut = "pub", set_with = "pub")]
    title: String,

    #[getset(get = "pub", set = "pub", get_mut = "pub", set_with = "pub")]
    category: String,
}

pub fn read_home_menu() -> Result<Vec<HomeMenu>> {
    let config_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("config")
        .join("home_menu.json");

    let json_content = fs::read_to_string(config_path)?;
    let home_menus: Vec<HomeMenu> = serde_json::from_str(&json_content)?;
    Ok(home_menus)
}

#[cfg(test)]
mod home_config_test {

    use crate::components::home::home_config::read_home_menu;
    use anyhow::Result;

    #[test]
    fn read_test() -> Result<()> {
        let home_menus = read_home_menu()?;
        for item in home_menus.iter() {
            println!("{:?}", item);
        }
        Ok(())
    }
}
