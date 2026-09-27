use crate::lua::{color::Color, element::Element};
use mlua::{FromLua, Lua, Value};

#[derive(Debug)]
pub struct Container {
    pub margin: u32,
    pub spacing: u32,
    pub background: Color,
    pub children: Vec<Element>,
}

impl Container {
    pub fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }
}

impl FromLua for Container {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected table".into()),
            });
        };

        Ok(Self {
            margin: table.get("margin")?,
            spacing: table.get("spacing")?,
            background: table.get("background")?,
            children: table.get("children")?,
        })
    }
}
