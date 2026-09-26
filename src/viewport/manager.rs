use std::{error::Error, sync::Arc};

use winit::{
    dpi::PhysicalPosition,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowLevel},
};

use crate::{
    lua::{
        anchor::{HorizontalAnchor, VerticalAnchor},
        viewport::ViewportParameters,
    },
    viewport::Viewport,
};

pub struct ViewportManager {
    viewport: Option<Arc<Viewport>>,
    parameters: ViewportParameters,
}

impl ViewportManager {
    pub fn new(parameters: ViewportParameters) -> Self {
        Self {
            viewport: None,
            parameters,
        }
    }

    pub fn create_viewport(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) -> Result<Arc<Viewport>, Box<dyn Error>> {
        if let Some(viewport) = &self.viewport {
            return Ok(viewport.clone());
        }

        let viewport = Arc::new(event_loop.create_window(Self::viewport_attributes())?);

        viewport.set_cursor_hittest(false)?;

        self.viewport = Some(viewport.clone());

        Ok(viewport)
    }

    pub fn request_redraw(&self) {
        if let Some(viewport) = &self.viewport {
            viewport.request_redraw();
        }
    }

    pub fn reposition(&self) {
        let Some(viewport) = &self.viewport else {
            return;
        };

        let Some(monitor) = viewport.current_monitor() else {
            return;
        };

        let monitor_position = monitor.position();
        let monitor_size = monitor.size();
        let viewport_size = viewport.outer_size();
        let scale_factor = monitor.scale_factor();

        let horizontal_offset = (self.parameters.offset.horizontal * scale_factor).round() as i32;
        let vertical_offset = (self.parameters.offset.vertical * scale_factor).round() as i32;

        let position = PhysicalPosition::new(
            Self::horizontal_position(
                self.parameters.anchor.horizontal,
                monitor_position.x,
                monitor_size.width,
                viewport_size.width,
                horizontal_offset,
            ),
            Self::vertical_position(
                self.parameters.anchor.vertical,
                monitor_position.y,
                monitor_size.height,
                viewport_size.height,
                vertical_offset,
            ),
        );

        viewport.set_outer_position(position);
    }

    fn horizontal_position(
        anchor: HorizontalAnchor,
        monitor_position: i32,
        monitor_width: u32,
        viewport_width: u32,
        offset: i32,
    ) -> i32 {
        match anchor {
            HorizontalAnchor::Left => monitor_position + offset,

            HorizontalAnchor::Center => {
                monitor_position + (monitor_width as i32 - viewport_width as i32) / 2 + offset
            }

            HorizontalAnchor::Right => {
                monitor_position + monitor_width as i32 - viewport_width as i32 - offset
            }
        }
    }

    fn vertical_position(
        anchor: VerticalAnchor,
        monitor_position: i32,
        monitor_height: u32,
        viewport_height: u32,
        offset: i32,
    ) -> i32 {
        match anchor {
            VerticalAnchor::Top => monitor_position + offset,

            VerticalAnchor::Center => {
                monitor_position + (monitor_height as i32 - viewport_height as i32) / 2 + offset
            }

            VerticalAnchor::Bottom => {
                monitor_position + monitor_height as i32 - viewport_height as i32 - offset
            }
        }
    }

    fn viewport_attributes() -> WindowAttributes {
        WindowAttributes::default()
            .with_blur(false)
            .with_title("hoverlay")
            .with_transparent(true)
            .with_decorations(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
    }
}
