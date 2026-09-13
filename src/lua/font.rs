use mlua::{FromLua, Lua, Value};

#[derive(Debug)]
pub struct Font {
    pub family: String,
    pub size: f32,
}

impl FromLua for Font {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected table".into()),
            });
        };

        Ok(Self {
            family: table.get("family")?,
            size: table.get("size")?,
        })
    }
}
