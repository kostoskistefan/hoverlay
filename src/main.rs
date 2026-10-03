use crate::application::Application;
use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

mod application;
mod font;
mod layout;
mod lua;
mod render;
mod viewport;

#[derive(Debug)]
pub enum ApplicationEvent {
    Shutdown,
}

fn install_signal_handler(proxy: EventLoopProxy<ApplicationEvent>) -> Result<(), Box<dyn Error>> {
    ctrlc::set_handler(move || {
        let _ = proxy.send_event(ApplicationEvent::Shutdown);
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::<ApplicationEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let proxy = event_loop.create_proxy();
    install_signal_handler(proxy.clone())?;

    let mut application = Application::new()?;

    event_loop.run_app(&mut application)?;

    Ok(())
}
