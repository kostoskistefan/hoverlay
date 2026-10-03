use crate::{lua::color::Color, viewport::Viewport};
use softbuffer::SoftBufferError;
use std::sync::Arc;

type SurfaceBuffer<'surface> = softbuffer::Buffer<'surface, Arc<Viewport>, Arc<Viewport>>;

pub struct Canvas<'surface> {
    buffer: SurfaceBuffer<'surface>,
}

impl<'surface> Canvas<'surface> {
    pub fn new(buffer: SurfaceBuffer<'surface>) -> Self {
        Self { buffer }
    }

    pub fn width(&self) -> u32 {
        self.buffer.width().get()
    }

    pub fn height(&self) -> u32 {
        self.buffer.height().get()
    }

    pub fn fill(&mut self, color_value: u32) {
        self.buffer.fill(color_value);
    }

    pub fn present(self) -> Result<(), SoftBufferError> {
        self.buffer.present()
    }

    pub fn blend_pixel(&mut self, x: i32, y: i32, foreground: &Color) {
        if self.contains(x, y) == false {
            return;
        }

        let pixel_index = self.pixel_index(x, y);
        let background = Color::new(self.buffer[pixel_index]);

        self.buffer[pixel_index] = background
            .blend_premultiplied_background(foreground)
            .value();
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width() as i32 && y < self.height() as i32
    }

    fn pixel_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.width() as usize + x as usize
    }
}
