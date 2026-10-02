use crate::{
    font::manager::FontManager,
    layout::engine::LayoutEngine,
    lua::Configuration,
    render::{
        RenderEngine,
        scheduler::{Schedule, Scheduler},
    },
    viewport::ViewportManager,
};
use std::error::Error;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::WindowId,
};

pub struct Application {
    configuration: Configuration,
    viewport_manager: ViewportManager,
    font_manager: FontManager,
    render_engine: Option<RenderEngine>,
    layout_engine: LayoutEngine,
    scheduler: Scheduler,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let configuration = Configuration::load()?;
        let viewport_manager = ViewportManager::new(configuration.viewport_parameters.clone());
        let scheduler = Scheduler::new(configuration.is_static());

        Ok(Self {
            configuration,
            viewport_manager,
            font_manager: FontManager::new(),
            render_engine: None,
            layout_engine: LayoutEngine::new(),
            scheduler,
        })
    }

    fn resume(&mut self, event_loop: &ActiveEventLoop) -> Result<(), Box<dyn Error>> {
        let viewport = self.viewport_manager.acquire(event_loop)?;

        if self.render_engine.is_none() {
            self.render_engine = Some(RenderEngine::new(viewport)?);
        }

        self.prepare_frame();
        self.viewport_manager.show();

        Ok(())
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.render_engine()?.render()
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
        self.render_engine()?.resize(width, height)?;
        self.viewport_manager.reposition();

        Ok(())
    }

    fn prepare_frame(&mut self) {
        self.configuration.update();

        self.layout_engine.update(
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
        exit_on_failure(event_loop, "Resume", self.resume(event_loop));
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match self.scheduler.next() {
            Schedule::Wait => {
                event_loop.set_control_flow(ControlFlow::Wait);
            }

            Schedule::WaitUntil(time) => {
                event_loop.set_control_flow(ControlFlow::WaitUntil(time));
            }

            Schedule::Render => {
                self.prepare_frame();
                self.viewport_manager.request_redraw();
            }
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
                exit_on_failure(event_loop, "Resize", self.resize(size.width, size.height));
            }

            WindowEvent::RedrawRequested => {
                exit_on_failure(event_loop, "Rendering", self.render());
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

fn exit_on_failure(
    event_loop: &ActiveEventLoop,
    action_name: &str,
    result: Result<(), Box<dyn Error>>,
) {
    if let Err(error) = result {
        eprintln!("{action_name} failed: {error}");
        event_loop.exit();
    }
}
