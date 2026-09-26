use crate::application::Application;
use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};

mod application;
mod font;
mod lua;
mod render;
mod viewport;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut application = Application::new()?;
    event_loop.run_app(&mut application)?;

    Ok(())
}
