use std::{error::Error, sync::Arc};

use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowLevel},
};

use crate::{
    lua::{
        anchor::{HorizontalAnchor, VerticalAnchor},
        viewport::{ViewportParameters, ViewportSizePolicy},
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

    pub fn get_or_create(
        &mut self,
        event_loop: &ActiveEventLoop,
        initial_content_size: LogicalSize<f32>,
    ) -> Result<Arc<Viewport>, Box<dyn Error>> {
        if let Some(viewport) = &self.viewport {
            return Ok(viewport.clone());
        }

        let size = match &self.parameters.size_policy {
            ViewportSizePolicy::Fixed(size) => size.cast::<f64>(),
            ViewportSizePolicy::InitialContent => initial_content_size.cast::<f64>(),
        };

        let monitor = event_loop
            .primary_monitor()
            .ok_or("No primary monitor available")?;

        let scale_factor = monitor.scale_factor();

        let monitor_position = monitor.position().to_logical::<f64>(scale_factor);
        let monitor_size = monitor.size().to_logical::<f64>(scale_factor);

        let position = LogicalPosition::new(
            Self::horizontal_position(
                self.parameters.anchor.horizontal,
                monitor_position.x,
                monitor_size.width,
                size.width,
                self.parameters.offset.horizontal as f64,
            ),
            Self::vertical_position(
                self.parameters.anchor.vertical,
                monitor_position.y,
                monitor_size.height,
                size.height,
                self.parameters.offset.vertical as f64,
            ),
        );

        let attributes = Self::viewport_attributes()
            .with_inner_size(size)
            .with_position(position);

        let viewport = Arc::new(event_loop.create_window(attributes)?);

        viewport.set_cursor_hittest(false)?;

        self.viewport = Some(viewport.clone());

        Ok(viewport)
    }

    pub fn show(&self) {
        if let Some(viewport) = &self.viewport {
            viewport.set_visible(true);
        }
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

        let scale_factor = monitor.scale_factor();

        let monitor_position = monitor.position().to_logical::<f64>(scale_factor);
        let monitor_size = monitor.size().to_logical::<f64>(scale_factor);
        let viewport_size = viewport.outer_size().to_logical::<f64>(scale_factor);

        let position = LogicalPosition::new(
            Self::horizontal_position(
                self.parameters.anchor.horizontal,
                monitor_position.x,
                monitor_size.width,
                viewport_size.width,
                self.parameters.offset.horizontal as f64,
            ),
            Self::vertical_position(
                self.parameters.anchor.vertical,
                monitor_position.y,
                monitor_size.height,
                viewport_size.height,
                self.parameters.offset.vertical as f64,
            ),
        );

        viewport.set_outer_position(position);
    }

    fn horizontal_position(
        anchor: HorizontalAnchor,
        monitor_position: f64,
        monitor_width: f64,
        viewport_width: f64,
        offset: f64,
    ) -> f64 {
        match anchor {
            HorizontalAnchor::Left => monitor_position + offset,

            HorizontalAnchor::Center => {
                monitor_position + (monitor_width - viewport_width) / 2.0 + offset
            }

            HorizontalAnchor::Right => monitor_position + monitor_width - viewport_width - offset,
        }
    }

    fn vertical_position(
        anchor: VerticalAnchor,
        monitor_position: f64,
        monitor_height: f64,
        viewport_height: f64,
        offset: f64,
    ) -> f64 {
        match anchor {
            VerticalAnchor::Top => monitor_position + offset,

            VerticalAnchor::Center => {
                monitor_position + (monitor_height - viewport_height) / 2.0 + offset
            }

            VerticalAnchor::Bottom => monitor_position + monitor_height - viewport_height - offset,
        }
    }

    fn viewport_attributes() -> WindowAttributes {
        let attributes = WindowAttributes::default()
            .with_visible(false)
            .with_blur(false)
            .with_title("hoverlay")
            .with_transparent(true)
            .with_decorations(false)
            .with_window_level(WindowLevel::AlwaysOnTop);

        #[cfg(target_os = "windows")]
        let attributes = {
            use winit::platform::windows::WindowAttributesExtWindows;

            attributes.with_skip_taskbar(true)
        };

        #[cfg(target_os = "linux")]
        let attributes = {
            use winit::platform::x11::{WindowAttributesExtX11, WindowType};

            attributes.with_x11_window_type(vec![WindowType::Dock])
        };

        attributes
    }
}
