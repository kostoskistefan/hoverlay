use super::{
    canvas::Canvas, content_area::ContentArea, glyph_renderer::GlyphRenderer,
    text_buffer_factory::TextBufferFactory, text_renderer::TextRenderer,
};
use crate::{
    layout::layout::Layout,
    lua::{container::Container, element::Element},
};
use cosmic_text::FontSystem;
use winit::dpi::LogicalPosition;

pub struct ContainerRenderer {
    text_renderer: TextRenderer,
}

impl ContainerRenderer {
    pub fn new() -> Self {
        Self {
            text_renderer: TextRenderer::new(),
        }
    }

    pub fn render(
        &mut self,
        container: &Container,
        layout: &Layout,
        font_system: &mut FontSystem,
        canvas: &mut Canvas<'_>,
    ) {
        let content_area = ContentArea::new(
            container.margin as f32,
            canvas.width() as f32,
            canvas.height() as f32,
        );

        for (element, position) in container.children.iter().zip(&layout.positions) {
            self.render_element(element, *position, &content_area, font_system, canvas);
        }
    }

    fn render_element(
        &mut self,
        element: &Element,
        position: LogicalPosition<f32>,
        content_area: &ContentArea,
        font_system: &mut FontSystem,
        canvas: &mut Canvas<'_>,
    ) {
        let mut text_buffer =
            TextBufferFactory::create_shaped(element, content_area.size(), font_system);

        let glyph_renderer = GlyphRenderer::new(canvas, content_area, position);

        self.text_renderer.render(
            &mut text_buffer,
            element.style.color,
            font_system,
            glyph_renderer,
        );
    }
}
