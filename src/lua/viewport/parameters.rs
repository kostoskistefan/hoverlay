use crate::lua::{anchor::Anchor, offset::Offset, viewport::ViewportSizePolicy};
use mlua::{FromLua, Lua, Value};

#[derive(Debug, Clone)]
pub struct ViewportParameters {
    pub anchor: Anchor,
    pub offset: Offset,
    pub size_policy: ViewportSizePolicy,
}

impl FromLua for ViewportParameters {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected table".into()),
            });
        };

        Ok(Self {
            anchor: table.get("anchor")?,
            offset: table.get("offset")?,
            size_policy: table.get("size")?,
        })
    }
}
