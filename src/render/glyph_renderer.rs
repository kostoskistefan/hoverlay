use super::{canvas::Canvas, content_area::ContentArea};
use crate::lua::color::Color;
use winit::dpi::LogicalPosition;

pub struct GlyphRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: cosmic_text::Color,
}

pub struct GlyphRenderer<'canvas, 'surface> {
    canvas: &'canvas mut Canvas<'surface>,
    content_area: &'canvas ContentArea,
    origin: LogicalPosition<f32>,
}

impl<'canvas, 'surface> GlyphRenderer<'canvas, 'surface> {
    pub fn new(
        canvas: &'canvas mut Canvas<'surface>,
        content_area: &'canvas ContentArea,
        origin: LogicalPosition<f32>,
    ) -> Self {
        Self {
            canvas,
            content_area,
            origin,
        }
    }

    pub fn render(&mut self, region: GlyphRegion) {
        let foreground = Color::from(region.color);
        let left = self.origin.x as i32 + region.x;
        let top = self.origin.y as i32 + region.y;

        for vertical_offset in 0..region.height as i32 {
            for horizontal_offset in 0..region.width as i32 {
                self.render_pixel(left + horizontal_offset, top + vertical_offset, &foreground);
            }
        }
    }

    fn render_pixel(&mut self, x: i32, y: i32, foreground: &Color) {
        if self.content_area.contains(x, y) {
            self.canvas.blend_pixel(x, y, foreground);
        }
    }
}
