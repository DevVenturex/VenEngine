use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;
use std::borrow::BorrowMut;
use bytemuck::Pod;
use log::{error, info, warn};
use wgpu::util::{DeviceExt, RenderEncoder};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};
use cgmath::prelude::*;
use futures::executor;
use winit::dpi::LogicalSize;
use crate::{model, resources};
use crate::model::{DrawModel, Vertex};
use crate::texture::Texture;
use crate::window::WindowState;



#[derive(Default)]
pub struct Application<'a> {
    window_state: Option<WindowState<'a>>,
}

impl ApplicationHandler for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_state.is_none() {
            let window = Arc::new(event_loop.create_window(window_attributes()).unwrap());
            let state = executor::block_on(WindowState::new(window.clone()));
            self.window_state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if window_id != self.window_state.as_ref().unwrap().window.id() {
            return;
        }
        let mut window = self.window_state.as_mut().unwrap();
        if !window.event(&event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                    ..
                } => event_loop.exit(),
                WindowEvent::Resized(size) => {
                    window.resize(size);
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let mut state = self.window_state.as_mut().unwrap();
        state.window.request_redraw();

        state.update();
        match state.render() {
            Ok(_) => {}
            Err(
                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
            ) => state.resize(state.size),
            Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                error!("OutOfMemory");
                event_loop.exit();
            }
            Err(wgpu::SurfaceError::Timeout) => {
                warn!("Surface Timeout");
            }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {

    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {

    }
}

fn window_attributes() -> WindowAttributes {
    WindowAttributes::default()
        .with_title("WGPU Window")
        .with_inner_size(LogicalSize::new(1280, 720))
}
