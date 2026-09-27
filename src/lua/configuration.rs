use crate::lua::{container::Container, viewport::ViewportParameters};
use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};
use mlua::Lua;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Configuration {
    _lua: Lua,

    pub container: Container,
    pub viewport_parameters: ViewportParameters,
}

impl Configuration {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let file_path = Self::get_file_path()?;
        let file_contents = Self::get_file_contents(file_path)?;

        let lua = Lua::new();
        let table = lua.load(file_contents).eval::<mlua::Table>()?;

        Ok(Self {
            container: table.get("container")?,
            viewport_parameters: table.get("viewport")?,
            _lua: lua,
        })
    }

    pub fn update(&mut self) {
        self.container.update();
    }

    pub fn is_static(&self) -> bool {
        self.container
            .children
            .iter()
            .all(|element| element.content.resolve_function.is_none())
    }

    fn get_file_contents(file_path: PathBuf) -> Result<String, Box<dyn Error>> {
        if file_path.exists() {
            return Ok(std::fs::read_to_string(file_path)?);
        }

        let default = include_str!("default.lua");

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&file_path, default)?;

        Ok(default.to_owned())
    }

    fn get_file_path() -> Result<PathBuf, Box<dyn Error>> {
        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "io.github".to_string(),
            author: "Kostoski Stefan".to_string(),
            app_name: "Hoverlay".to_string(),
        })?;

        Ok(strategy.in_config_dir("config.lua"))
    }
}
