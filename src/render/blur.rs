use super::canvas::Canvas;
use crate::lua::color::Color;

struct PixelSums {
    alpha: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl PixelSums {
    fn empty() -> Self {
        Self {
            alpha: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn add(&mut self, pixel: u32) {
        let color = Color::new(pixel);

        self.alpha += color.alpha();
        self.red += color.red();
        self.green += color.green();
        self.blue += color.blue();
    }

    fn subtract(&mut self, pixel: u32) {
        let color = Color::new(pixel);

        self.alpha -= color.alpha();
        self.red -= color.red();
        self.green -= color.green();
        self.blue -= color.blue();
    }

    fn average(&self, window: &Window) -> u32 {
        Color::from_channels(
            window.divide(self.alpha),
            window.divide(self.red),
            window.divide(self.green),
            window.divide(self.blue),
        )
        .value()
    }
}

struct Window {
    radius: usize,
    reciprocal: u64,
}

impl Window {
    const FIXED_POINT_ONE: u64 = 1 << 32;

    fn new(radius: usize) -> Self {
        let size = (radius * 2 + 1) as u64;

        Self {
            radius,
            reciprocal: (Self::FIXED_POINT_ONE + size - 1) / size,
        }
    }

    fn divide(&self, sum: u32) -> u32 {
        ((sum as u64 * self.reciprocal) >> 32) as u32
    }
}

struct LineLayout {
    line_count: usize,
    line_length: usize,
    line_step: usize,
    element_step: usize,
}

impl LineLayout {
    fn horizontal(width: usize, height: usize) -> Self {
        Self {
            line_count: height,
            line_length: width,
            line_step: width,
            element_step: 1,
        }
    }

    fn vertical(width: usize, height: usize) -> Self {
        Self {
            line_count: width,
            line_length: height,
            line_step: 1,
            element_step: width,
        }
    }

    fn pixel_index(&self, line_index: usize, position: usize) -> usize {
        line_index * self.line_step + position * self.element_step
    }

    fn clamped_pixel_index(&self, line_index: usize, position: isize) -> usize {
        self.pixel_index(
            line_index,
            position.clamp(0, self.line_length as isize - 1) as usize,
        )
    }
}

pub struct BoxBlur {
    scratch: Vec<u32>,
}

impl BoxBlur {
    const MAXIMUM_RADIUS: usize = 2047;

    pub fn new() -> Self {
        Self {
            scratch: Vec::new(),
        }
    }

    pub fn apply(&mut self, canvas: &mut Canvas<'_>, radius: u32) {
        if radius == 0 {
            return;
        }

        let width = canvas.width() as usize;
        let height = canvas.height() as usize;
        let window = Window::new((radius as usize).min(Self::MAXIMUM_RADIUS));

        self.scratch.resize(width * height, 0);

        let pixels = canvas.pixels_mut();

        Self::blur_lines(
            pixels,
            &mut self.scratch,
            &LineLayout::horizontal(width, height),
            &window,
        );
        Self::blur_lines(
            &self.scratch,
            pixels,
            &LineLayout::vertical(width, height),
            &window,
        );
    }

    fn blur_lines(source: &[u32], destination: &mut [u32], layout: &LineLayout, window: &Window) {
        for line_index in 0..layout.line_count {
            Self::blur_line(source, destination, line_index, layout, window);
        }
    }

    fn blur_line(
        source: &[u32],
        destination: &mut [u32],
        line_index: usize,
        layout: &LineLayout,
        window: &Window,
    ) {
        let mut sums = Self::create_initial_sums(source, line_index, layout, window);
        let radius = window.radius as isize;

        for position in 0..layout.line_length {
            destination[layout.pixel_index(line_index, position)] = sums.average(window);

            let entering_position = position as isize + radius + 1;
            let leaving_position = position as isize - radius;

            sums.add(source[layout.clamped_pixel_index(line_index, entering_position)]);
            sums.subtract(source[layout.clamped_pixel_index(line_index, leaving_position)]);
        }
    }

    fn create_initial_sums(
        source: &[u32],
        line_index: usize,
        layout: &LineLayout,
        window: &Window,
    ) -> PixelSums {
        let radius = window.radius as isize;
        let mut sums = PixelSums::empty();

        for position in -radius..=radius {
            sums.add(source[layout.clamped_pixel_index(line_index, position)]);
        }

        sums
    }
}
