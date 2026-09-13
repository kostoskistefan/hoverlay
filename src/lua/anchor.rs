use mlua::{FromLua, Lua, Value};

#[derive(Debug, Clone, Copy)]
pub struct Anchor {
    pub horizontal: HorizontalAnchor,
    pub vertical: VerticalAnchor,
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAnchor {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAnchor {
    Top,
    Center,
    Bottom,
}

fn conversion_error<T>(from: &'static str) -> mlua::Error {
    mlua::Error::FromLuaConversionError {
        from,
        to: std::any::type_name::<T>().to_owned(),
        message: Some("expected a valid position value".into()),
    }
}

fn get_string(value: Value, lua: &Lua) -> mlua::Result<String> {
    let value_type = value.type_name();

    String::from_lua(value, lua).map_err(|_| conversion_error::<String>(value_type))
}

impl FromLua for Anchor {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return Err(conversion_error::<Self>("value"));
        };

        Ok(Self {
            horizontal: table.get("horizontal")?,
            vertical: table.get("vertical")?,
        })
    }
}

impl FromLua for HorizontalAnchor {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        match get_string(value, lua)?.as_str() {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            _ => Err(conversion_error::<Self>("string")),
        }
    }
}

impl FromLua for VerticalAnchor {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        match get_string(value, lua)?.as_str() {
            "top" => Ok(Self::Top),
            "center" => Ok(Self::Center),
            "bottom" => Ok(Self::Bottom),
            _ => Err(conversion_error::<Self>("string")),
        }
    }
}
