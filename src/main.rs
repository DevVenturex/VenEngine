use std::env;
use log::error;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::application::Application;

mod application;
pub mod texture;

fn main() {
    unsafe { env::set_var("RUST_LOG", "debug") };
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = Application::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        error!("Event loop error: {}", e);
    }
}

