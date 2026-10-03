use crate::{
    layout::layout::Layout,
    lua::container::Container,
    render::{canvas::Canvas, container_renderer::ContainerRenderer},
    viewport::Viewport,
};
use cosmic_text::FontSystem;
use softbuffer::{Context, Surface};
use std::{error::Error, num::NonZeroU32, sync::Arc};

pub struct RenderEngine {
    _context: Context<Arc<Viewport>>,
    surface: Surface<Arc<Viewport>, Arc<Viewport>>,
    container_renderer: ContainerRenderer,
}

impl RenderEngine {
    pub fn new(viewport: Arc<Viewport>) -> Result<Self, Box<dyn Error>> {
        let context = Context::new(viewport.clone())?;
        let mut surface = Surface::new(&context, viewport.clone())?;

        let size = viewport.inner_size();
        Self::resize_surface(&mut surface, size.width, size.height)?;

        Ok(Self {
            _context: context,
            surface,
            container_renderer: ContainerRenderer::new(),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
        Self::resize_surface(&mut self.surface, width, height)
    }

    pub fn render(
        &mut self,
        container: &Container,
        layout: &Layout,
        font_system: &mut FontSystem,
    ) -> Result<(), Box<dyn Error>> {
        let mut canvas = Canvas::new(self.surface.buffer_mut()?);

        canvas.fill(container.background.premultiplied().value());

        self.container_renderer
            .render(container, layout, font_system, &mut canvas);

        canvas.present()?;

        Ok(())
    }

    fn resize_surface(
        surface: &mut Surface<Arc<Viewport>, Arc<Viewport>>,
        width: u32,
        height: u32,
    ) -> Result<(), Box<dyn Error>> {
        if let (Some(non_zero_width), Some(non_zero_height)) =
            (NonZeroU32::new(width), NonZeroU32::new(height))
        {
            surface.resize(non_zero_width, non_zero_height)?;
        }

        Ok(())
    }
}
