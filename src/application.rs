use crate::{
    font::manager::FontManager, layout::engine::LayoutEngine, lua::Configuration,
    render::RenderEngine, viewport::ViewportManager,
};
use std::{
    error::Error,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::WindowId,
};

const RENDER_INTERVAL: Duration = Duration::from_secs(1);

pub struct Application {
    configuration: Configuration,
    viewport_manager: ViewportManager,
    font_manager: FontManager,
    render_engine: Option<RenderEngine>,
    layout_engine: LayoutEngine,
    next_render_time: Instant,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let configuration = Configuration::load()?;
        let viewport_manager = ViewportManager::new(configuration.viewport_parameters.clone());
        let font_manager = FontManager::new();
        let render_engine = None;
        let layout_engine = LayoutEngine::new();
        let next_render_time = Instant::now() + RENDER_INTERVAL;

        Ok(Self {
            configuration,
            viewport_manager,
            font_manager,
            render_engine,
            layout_engine,
            next_render_time,
        })
    }

    fn resume(&mut self, event_loop: &ActiveEventLoop) -> Result<(), Box<dyn Error>> {
        let viewport = self.viewport_manager.create_viewport(event_loop)?;

        if self.render_engine.is_none() {
            self.render_engine = Some(RenderEngine::new(viewport)?);
        }

        self.prepare_frame();
        self.viewport_manager.show_viewport();

        Ok(())
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.render_engine()?.render()?;

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
        self.render_engine()?.resize(width, height)?;
        self.viewport_manager.reposition();

        Ok(())
    }

    fn prepare_frame(&mut self) {
        self.configuration.update();

        self.layout_engine.layout(
            &self.configuration.container,
            self.font_manager.font_system_mut(),
        );
    }

    fn render_engine(&mut self) -> Result<&mut RenderEngine, Box<dyn Error>> {
        self.render_engine
            .as_mut()
            .ok_or_else(|| "Render engine is not initialized".into())
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(error) = self.resume(event_loop) {
            eprintln!("Resume failed: {error}");
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.configuration.is_static() {
            event_loop.set_control_flow(ControlFlow::Wait);
            return;
        }

        if Instant::now() < self.next_render_time {
            event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_render_time));
            return;
        }

        self.prepare_frame();
        self.viewport_manager.request_redraw();
        self.next_render_time += RENDER_INTERVAL;
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
                    eprintln!("Resize failed: {error}");
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
