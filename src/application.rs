use crate::{lua::Configuration, render::RenderEngine, viewport::ViewportManager};
use std::error::Error;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

pub struct Application {
    configuration: Configuration,
    viewport_manager: ViewportManager,
    render_engine: Option<RenderEngine>,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let configuration = Configuration::load()?;
        let viewport_manager = ViewportManager::new(configuration.viewport.clone());

        Ok(Self {
            configuration,
            viewport_manager,
            render_engine: None,
        })
    }

    fn resume(&mut self, event_loop: &ActiveEventLoop) -> Result<(), Box<dyn Error>> {
        let viewport = self.viewport_manager.create_viewport(event_loop)?;

        if self.render_engine.is_none() {
            self.render_engine = Some(RenderEngine::new(viewport)?);
        }

        Ok(())
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(render_engine) = &mut self.render_engine {
            render_engine.render()?;
        }

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
        if let Some(render_engine) = &mut self.render_engine {
            render_engine.resize(width, height)?;
        }

        self.viewport_manager.reposition();

        Ok(())
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(error) = self.resume(event_loop) {
            eprintln!("Failed to resume application: {error}");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                if let Err(error) = self.resize(size.width, size.height) {
                    eprintln!("Failed to resize renderer: {error}");
                    event_loop.exit();
                }
            }

            WindowEvent::RedrawRequested => {
                if let Err(error) = self.render() {
                    eprintln!("Rendering failed: {error}");
                    event_loop.exit();
                }
            }

            WindowEvent::KeyboardInput { event, .. } if event.state.is_pressed() => {
                match &event.logical_key {
                    winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) => {
                        event_loop.exit();
                    }

                    winit::keyboard::Key::Character(character)
                        if character.eq_ignore_ascii_case("q") =>
                    {
                        event_loop.exit();
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }
}
