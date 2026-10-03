use super::glyph_renderer::{GlyphRegion, GlyphRenderer};
use crate::lua::color::Color;
use cosmic_text::{Buffer as TextBuffer, FontSystem, SwashCache};

pub struct TextRenderer {
    swash_cache: SwashCache,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            swash_cache: SwashCache::new(),
        }
    }

    pub fn render(
        &mut self,
        text_buffer: &mut TextBuffer,
        text_color: Color,
        font_system: &mut FontSystem,
        mut glyph_renderer: GlyphRenderer<'_, '_>,
    ) {
        text_buffer.draw(
            font_system,
            &mut self.swash_cache,
            text_color.into(),
            |x, y, width, height, color| {
                glyph_renderer.render(GlyphRegion {
                    x,
                    y,
                    width,
                    height,
                    color,
                });
            },
        );
    }
}
