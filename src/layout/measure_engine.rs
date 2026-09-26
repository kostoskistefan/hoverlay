use crate::{layout::size::Size, lua::element::Element};
use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap};

pub struct MeasureEngine {
    buffer: Buffer,
}

impl MeasureEngine {
    pub fn new(font_system: &mut FontSystem) -> Self {
        let mut buffer = Buffer::new(font_system, Metrics::new(1.0, 1.0));
        buffer.set_wrap(Wrap::None);

        Self { buffer }
    }

    pub fn measure_element(&mut self, element: &Element, font_system: &mut FontSystem) -> Size {
        let font_size = element.style.font.size;

        self.buffer.set_metrics(Metrics::new(font_size, font_size));

        self.buffer.set_text(
            &element.content.value,
            &Attrs::new().family(Family::Name(&element.style.font.family)),
            Shaping::Advanced,
            None,
        );

        self.buffer.shape_until_scroll(font_system, false);

        self.buffer
            .layout_runs()
            .fold(Size::default(), |size, run| Size {
                width: size.width.max(run.line_w),
                height: size.height.max(run.line_height),
            })
    }
}
