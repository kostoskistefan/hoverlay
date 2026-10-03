use mlua::{FromLua, Lua, Value};

pub struct Color(u32);

impl Color {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> u32 {
        self.0
    }

    fn conversion_error(from: &'static str) -> mlua::Error {
        mlua::Error::FromLuaConversionError {
            from,
            to: std::any::type_name::<Self>().to_owned(),
            message: Some("expected a 32-bit RGBA color".into()),
        }
    }
}

impl From<Color> for cosmic_text::Color {
    fn from(color: Color) -> Self {
        cosmic_text::Color(color.0)
    }
}

impl From<cosmic_text::Color> for Color {
    fn from(color: cosmic_text::Color) -> Self {
        Self::new(color.0)
    }
}

impl FromLua for Color {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        let value_type = value.type_name();

        let Value::Integer(value) = value else {
            return Err(Self::conversion_error(value_type));
        };

        let value = u32::try_from(value).map_err(|_| Self::conversion_error(value_type))?;

        Ok(Self::new(value))
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}
