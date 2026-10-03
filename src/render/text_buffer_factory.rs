use crate::lua::element::Element;
use cosmic_text::{Attrs, Buffer as TextBuffer, Family, FontSystem, Metrics, Shaping, Wrap};
use winit::dpi::LogicalSize;

pub struct TextBufferFactory;

impl TextBufferFactory {
    pub fn create_shaped(
        element: &Element,
        content_size: LogicalSize<f32>,
        font_system: &mut FontSystem,
    ) -> TextBuffer {
        let mut text_buffer = Self::create_text_buffer(element.style.font.size, font_system);

        Self::apply_text_layout(&mut text_buffer, content_size);
        Self::apply_text_content(&mut text_buffer, element);
        Self::shape_text_buffer(&mut text_buffer, font_system);

        text_buffer
    }

    fn create_text_buffer(font_size: f32, font_system: &mut FontSystem) -> TextBuffer {
        TextBuffer::new(font_system, Metrics::new(font_size, font_size))
    }

    fn apply_text_layout(text_buffer: &mut TextBuffer, content_size: LogicalSize<f32>) {
        text_buffer.set_wrap(Wrap::None);
        text_buffer.set_size(Some(content_size.width), Some(content_size.height));
    }

    fn apply_text_content(text_buffer: &mut TextBuffer, element: &Element) {
        text_buffer.set_text(
            &element.content.value,
            &Self::create_text_attributes(element),
            Shaping::Advanced,
            Some(element.style.alignment.into()),
        );
    }

    fn create_text_attributes(element: &Element) -> Attrs<'_> {
        Attrs::new().family(Family::Name(&element.style.font.family))
    }

    fn shape_text_buffer(text_buffer: &mut TextBuffer, font_system: &mut FontSystem) {
        text_buffer.shape_until_scroll(font_system, false);
    }
}
