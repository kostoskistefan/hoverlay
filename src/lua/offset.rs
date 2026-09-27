use mlua::{FromLua, Lua, Value};

#[derive(Debug, Clone)]
pub struct Offset {
    pub horizontal: i32,
    pub vertical: i32,
}

impl FromLua for Offset {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected table".into()),
            });
        };

        Ok(Self {
            horizontal: table.get("horizontal")?,
            vertical: table.get("vertical")?,
        })
    }
}
