use mlua::{FromLua, Lua, Value};

#[derive(Copy, Clone)]
pub struct Color(u32);

impl Color {
    pub const MAXIMUM_CHANNEL_VALUE: u32 = 255;

    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> u32 {
        self.0
    }

    pub fn premultiplied(&self) -> Self {
        let alpha = self.alpha();

        Self::from_channels(
            alpha,
            Self::multiply_channel(self.red(), alpha),
            Self::multiply_channel(self.green(), alpha),
            Self::multiply_channel(self.blue(), alpha),
        )
    }

    pub fn blend_premultiplied_background(&self, straight_foreground: &Color) -> Self {
        if straight_foreground.alpha() == 0 {
            return *self;
        }

        if straight_foreground.alpha() == Self::MAXIMUM_CHANNEL_VALUE {
            return *straight_foreground;
        }

        let foreground = straight_foreground.premultiplied();
        let inverse_foreground_alpha = Self::MAXIMUM_CHANNEL_VALUE - straight_foreground.alpha();

        let blend_channel = |foreground_channel: u32, background_channel: u32| {
            (foreground_channel
                + Self::multiply_channel(background_channel, inverse_foreground_alpha))
            .min(Self::MAXIMUM_CHANNEL_VALUE)
        };

        Self::from_channels(
            blend_channel(foreground.alpha(), self.alpha()),
            blend_channel(foreground.red(), self.red()),
            blend_channel(foreground.green(), self.green()),
            blend_channel(foreground.blue(), self.blue()),
        )
    }

    const fn multiply_channel(channel: u32, alpha: u32) -> u32 {
        (channel * alpha + 127) / Self::MAXIMUM_CHANNEL_VALUE
    }

    pub const fn from_channels(alpha: u32, red: u32, green: u32, blue: u32) -> Self {
        Self::new((alpha << 24) | (red << 16) | (green << 8) | blue)
    }

    pub const fn alpha(&self) -> u32 {
        self.0 >> 24
    }

    pub const fn red(&self) -> u32 {
        (self.0 >> 16) & 0xff
    }

    pub const fn green(&self) -> u32 {
        (self.0 >> 8) & 0xff
    }

    pub const fn blue(&self) -> u32 {
        self.0 & 0xff
    }

    pub fn composite_over_premultiplied(&self, background: &Color) -> Self {
        let inverse_alpha = Self::MAXIMUM_CHANNEL_VALUE - self.alpha();

        let composite_channel = |foreground_channel: u32, background_channel: u32| {
            (foreground_channel + Self::multiply_channel(background_channel, inverse_alpha))
                .min(Self::MAXIMUM_CHANNEL_VALUE)
        };

        Self::from_channels(
            composite_channel(self.alpha(), background.alpha()),
            composite_channel(self.red(), background.red()),
            composite_channel(self.green(), background.green()),
            composite_channel(self.blue(), background.blue()),
        )
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
