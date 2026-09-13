use mlua::{FromLua, Lua, Value};

pub struct Color(u32);

impl Color {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub const fn red(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub const fn green(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub const fn blue(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub const fn alpha(self) -> u8 {
        self.0 as u8
    }

    fn conversion_error(from: &'static str) -> mlua::Error {
        mlua::Error::FromLuaConversionError {
            from,
            to: std::any::type_name::<Self>().to_owned(),
            message: Some("expected a 32-bit RGBA color".into()),
        }
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
