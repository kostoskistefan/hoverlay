use mlua::{FromLua, Lua, Value};

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl FromLua for Alignment {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::String(value) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected alignment string".into()),
            });
        };

        match value.to_str()?.as_ref() {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected left, center, or right".into()),
            }),
        }
    }
}
