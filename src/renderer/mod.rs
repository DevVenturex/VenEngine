mod context;
pub use context::*;

mod resources;
pub use resources::*;

use anyhow::Result;
use winit::window::Window;

use std::{collections::HashMap, sync::Arc};

pub struct Renderer<'a> {
    context: Context<'a>,
    resources: Vec<Resource>,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let context = Context::new(window).await?;

        let desc = wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        };
        let render_pipeline = context.create_render_pipeline(desc);

        Ok(Self {
            context,
            resources: Vec::new(),
            render_pipeline,
        })
    }

    pub fn draw(&mut self) {
        let mut render_pass = self.context.create_render_pass();

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }

    pub fn begin_pipelines(&mut self) {
        
    }
}