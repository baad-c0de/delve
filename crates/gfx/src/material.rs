use wgpu::{
    ColorTargetState, Device, FragmentState, ShaderModule, ShaderModuleDescriptor,
    VertexBufferLayout, VertexState,
};

#[derive(Debug)]
pub struct Material<'layout> {
    shader: ShaderModule,
    vertex_entry_point: &'static str,
    fragment_entry_point: &'static str,
    buffer_layouts: Vec<VertexBufferLayout<'layout>>,
}

impl<'material> Material<'material> {
    pub fn new(
        device: &Device,
        shader: ShaderModuleDescriptor,
        vertex_entry_point: &'static str,
        fragment_entry_point: &'static str,
    ) -> Self {
        let shader = device.create_shader_module(shader);
        Self {
            shader,
            vertex_entry_point,
            fragment_entry_point,
            buffer_layouts: Vec::new(),
        }
    }

    pub fn vertex_state(&self) -> VertexState {
        VertexState {
            module: &self.shader,
            entry_point: self.vertex_entry_point,
            buffers: &self.buffer_layouts,
        }
    }

    pub fn fragment_state<'a>(&'a self, targets: &'a [Option<ColorTargetState>]) -> FragmentState {
        FragmentState {
            module: &self.shader,
            entry_point: self.fragment_entry_point,
            targets,
        }
    }

    pub fn add_buffer_layout(mut self, layout: VertexBufferLayout<'material>) -> Self {
        self.buffer_layouts.push(layout);
        self
    }
}
