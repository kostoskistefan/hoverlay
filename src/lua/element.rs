use crate::lua::{content::Content, style::Style};
use mlua::{FromLua, Lua, Value};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Element {
    pub content: Content,
    pub style: Style,
}

impl Element {
    pub fn update(&mut self) {
        self.content.update();
    }
}

impl FromLua for Element {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected table".into()),
            });
        };

        Ok(Self {
            content: table.get("content")?,
            style: table.get("style")?,
        })
    }
}
