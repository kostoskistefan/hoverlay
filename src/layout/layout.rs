use winit::dpi::{LogicalPosition, LogicalSize};

#[derive(Debug)]
pub struct Layout {
    pub size: LogicalSize<f32>,
    pub positions: Vec<LogicalPosition<f32>>,
}
