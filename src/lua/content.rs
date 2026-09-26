use mlua::{FromLua, Function, Lua, Value};
use std::fmt::Debug;

pub struct Content {
    pub value: String,
    pub resolve_function: Option<Function>,
}

impl Content {
    pub fn update(&mut self) {
        if let Some(resolve_function) = &self.resolve_function {
            self.value = resolve_function.call::<String>(()).unwrap_or_default();
        }
    }
}

impl FromLua for Content {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        match value {
            Value::String(value) => Ok(Self {
                value: value.to_str()?.to_owned(),
                resolve_function: None,
            }),

            Value::Function(resolve_function) => Ok(Self {
                value: resolve_function.call::<String>(())?,
                resolve_function: Some(resolve_function),
            }),

            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected string or function".into()),
            }),
        }
    }
}

impl Debug for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}
