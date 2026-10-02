use mlua::{FromLua, Lua, Value};
use winit::dpi::LogicalSize;

#[derive(Debug, Clone)]
pub enum ViewportSizePolicy {
    Fixed(LogicalSize<i32>),
    InitialContent,
}

impl FromLua for ViewportSizePolicy {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("expected { policy = \"fixed\" | \"initial\", ... }".into()),
            });
        };

        match table.get::<String>("policy")?.as_str() {
            "initial_content" => Ok(Self::InitialContent),

            "fixed" => {
                let width: i32 = table.get("width")?;
                let height: i32 = table.get("height")?;

                Ok(Self::Fixed(LogicalSize::new(width, height)))
            }

            other => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: std::any::type_name::<Self>().to_string(),
                message: Some(format!(
                    "expected policy \"fixed\" or \"initial\", got \"{other}\""
                )),
            }),
        }
    }
}
