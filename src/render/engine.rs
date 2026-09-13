use crate::viewport::Viewport;
use softbuffer::{Context, Surface};
use std::{error::Error, num::NonZeroU32, sync::Arc};

pub struct RenderEngine {
    _context: Context<Arc<Viewport>>,
    surface: Surface<Arc<Viewport>, Arc<Viewport>>,
}

impl RenderEngine {
    pub fn new(viewport: Arc<Viewport>) -> Result<Self, Box<dyn Error>> {
        let context = Context::new(viewport.clone())?;
        let mut surface = Surface::new(&context, viewport.clone())?;

        let size = viewport.inner_size();

        if let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        {
            surface.resize(width, height)?;
        }

        Ok(Self {
            _context: context,
            surface,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
        let Some(width) = NonZeroU32::new(width) else {
            return Ok(());
        };

        let Some(height) = NonZeroU32::new(height) else {
            return Ok(());
        };

        self.surface.resize(width, height)?;

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buffer = self.surface.buffer_mut()?;

        buffer.fill(0xff000000);

        buffer.present()?;

        Ok(())
    }
}
