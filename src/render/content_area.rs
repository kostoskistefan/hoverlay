use winit::dpi::LogicalSize;

pub struct ContentArea {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    size: LogicalSize<f32>,
}

impl ContentArea {
    pub fn new(margin: f32, surface_width: f32, surface_height: f32) -> Self {
        let size = LogicalSize::new(
            Self::shrink_by_margin(surface_width, margin),
            Self::shrink_by_margin(surface_height, margin),
        );

        let left = margin as i32;
        let top = margin as i32;

        Self {
            left,
            top,
            right: left + size.width as i32,
            bottom: top + size.height as i32,
            size,
        }
    }

    pub fn size(&self) -> LogicalSize<f32> {
        self.size
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }

    fn shrink_by_margin(length: f32, margin: f32) -> f32 {
        (length - margin * 2.0).max(0.0)
    }
}
