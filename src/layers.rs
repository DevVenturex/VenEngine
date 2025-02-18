use std::any::Any;
use winit::event::WindowEvent;

pub trait Layer {
    fn attach(&mut self);
    fn detach(&mut self);
    fn update(&mut self);
    fn event(&mut self, event: &WindowEvent) -> bool;
}

#[derive(Default)]
pub struct LayerStack {
    stack: Vec<Box<dyn Layer>>,
    overlay_start: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            overlay_start: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.stack.insert(self.overlay_start, layer);
        self.overlay_start += 1;
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.stack.push(overlay);
    }

    pub fn pop_layer(&mut self, layer: Box<dyn Layer>) -> Option<&Box<dyn Layer>> {
        if self.stack.get(self.overlay_start).type_id() == layer.type_id() {
            let result: Option<&Box<dyn Layer>> = self.stack.get(self.overlay_start);
            self.stack.remove(self.overlay_start);
            self.overlay_start -= 1;
            result
        } else {
            None
        }
    }

    pub fn pop_overlay(&mut self, overlay: Box<dyn Layer>) -> Option<Box<dyn Layer>> {
        if self.stack.pop().type_id() == overlay.type_id() {
            self.stack.pop()
        } else {
            None
        }
    }

    pub fn layer_stack(&mut self) -> &Vec<Box<dyn Layer>> {
        &self.stack
    }
}

impl Default for &mut LayerStack {
    fn default() -> Self {
        &mut Self::default()
    }
}