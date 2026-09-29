use crate::{
    layout::layout::Layout,
    lua::{container::Container, element::Element},
};
use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap};
use winit::dpi::{LogicalPosition, LogicalSize};

pub struct LayoutEngine {
    layout: Layout,
    text_buffer: Buffer,
}

impl LayoutEngine {
    pub fn new() -> Self {
        let mut text_buffer = Buffer::new_empty(Metrics::new(1.0, 1.0));
        text_buffer.set_wrap(Wrap::None);

        Self {
            layout: Layout::default(),
            text_buffer,
        }
    }

    pub fn update(&mut self, container: &Container, font_system: &mut FontSystem) {
        let element_sizes = container
            .children
            .iter()
            .map(|element| self.measure_element(element, font_system))
            .collect::<Vec<_>>();

        self.layout = Layout {
            size: Self::measure_container(&element_sizes, container),
            positions: Self::calculate_positions(&element_sizes, container),
        }
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    fn calculate_positions(
        element_sizes: &[LogicalSize<f32>],
        container: &Container,
    ) -> Vec<LogicalPosition<f32>> {
        let margin = container.margin as f32;
        let spacing = container.spacing as f32;

        let mut y = margin;
        let mut positions = Vec::with_capacity(element_sizes.len());

        for size in element_sizes {
            positions.push(LogicalPosition::<f32>::new(margin, y));
            y += size.height + spacing;
        }

        positions
    }

    fn measure_element(
        &mut self,
        element: &Element,
        font_system: &mut FontSystem,
    ) -> LogicalSize<f32> {
        let font_size = element.style.font.size;

        self.text_buffer
            .set_metrics(Metrics::new(font_size, font_size));

        self.text_buffer.set_text(
            &element.content.value,
            &Attrs::new().family(Family::Name(&element.style.font.family)),
            Shaping::Advanced,
            None,
        );

        self.text_buffer.shape_until_scroll(font_system, false);

        self.text_buffer
            .layout_runs()
            .fold(LogicalSize::<f32>::new(0.0, 0.0), |size, run| {
                LogicalSize::<f32>::new(size.width.max(run.line_w), size.height + run.line_height)
            })
    }

    fn measure_container(
        element_sizes: &[LogicalSize<f32>],
        container: &Container,
    ) -> LogicalSize<f32> {
        let content_size =
            element_sizes
                .iter()
                .fold(LogicalSize::<f32>::new(0.0, 0.0), |size, element_size| {
                    LogicalSize::<f32>::new(
                        size.width.max(element_size.width),
                        size.height + element_size.height,
                    )
                });

        let spacing = container.spacing as f32 * element_sizes.len().saturating_sub(1) as f32;

        let margin = container.margin as f32 * 2.0;

        LogicalSize::<f32>::new(
            content_size.width + margin,
            content_size.height + spacing + margin,
        )
    }
}
