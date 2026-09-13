use crate::lua::{alignment::Alignment, color::Color, font::Font};
use mlua::{FromLua, Lua, Value};

#[derive(Debug)]
pub struct Style {
    pub alignment: Alignment,
    pub color: Color,
    pub font: Font,
}

impl FromLua for Style {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected table".into()),
            });
        };

        Ok(Self {
            alignment: table.get("alignment")?,
            color: table.get("color")?,
            font: table.get("font")?,
        })
    }
}
