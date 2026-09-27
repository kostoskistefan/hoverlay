use crate::layout::{position::Position, size::Size};

#[derive(Debug)]
pub struct Layout {
    pub size: Size,
    pub positions: Vec<Position>,
}
