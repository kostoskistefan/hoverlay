use crate::{
    layout::{layout::Layout, position::Position, size::Size},
    lua::{container::Container, element::Element},
};
use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap};

pub struct LayoutEngine {
    text_buffer: Buffer,
}

impl LayoutEngine {
    pub fn new() -> Self {
        let mut text_buffer = Buffer::new_empty(Metrics::new(1.0, 1.0));
        text_buffer.set_wrap(Wrap::None);

        Self { text_buffer }
    }

    pub fn layout(&mut self, container: &Container, font_system: &mut FontSystem) -> Layout {
        let element_sizes = container
            .children
            .iter()
            .map(|element| self.measure_element(element, font_system))
            .collect::<Vec<_>>();

        Layout {
            size: Self::measure_container(&element_sizes, container),
            positions: Self::calculate_positions(&element_sizes, container),
        }
    }

    fn calculate_positions(element_sizes: &[Size], container: &Container) -> Vec<Position> {
        let mut y = container.margin;
        let mut positions = Vec::with_capacity(element_sizes.len());

        for size in element_sizes {
            // TODO: The x coordinate is fixed to container.margin for now to let cosmic-text
            //       perform the actual text alignment in the render engine
            positions.push(Position::new(container.margin, y));
            y += size.height + container.spacing;
        }

        positions
    }

    fn measure_element(&mut self, element: &Element, font_system: &mut FontSystem) -> Size {
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
            .fold(Size::default(), |size, run| {
                Size::new(size.width.max(run.line_w), size.height + run.line_height)
            })
    }

    fn measure_container(element_sizes: &[Size], container: &Container) -> Size {
        let content_size = element_sizes
            .iter()
            .fold(Size::default(), |size, element_size| {
                Size::new(
                    size.width.max(element_size.width),
                    size.height + element_size.height,
                )
            });

        let spacing = container.spacing * element_sizes.len().saturating_sub(1) as f32;

        let margin = container.margin * 2.0;

        Size::new(
            content_size.width + margin,
            content_size.height + spacing + margin,
        )
    }
}
