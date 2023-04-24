use wgpu::{
    ColorTargetState, Device, FragmentState, ShaderModule, ShaderModuleDescriptor, VertexState,
};

pub struct Material {
    shader: ShaderModule,
    vertex_entry_point: &'static str,
    fragment_entry_point: &'static str,
}

impl Material {
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
        }
    }

    pub fn vertex_state(&self) -> VertexState {
        VertexState {
            module: &self.shader,
            entry_point: self.vertex_entry_point,
            buffers: &[],
        }
    }

    pub fn fragment_state<'a>(&'a self, targets: &'a [Option<ColorTargetState>]) -> FragmentState {
        FragmentState {
            module: &self.shader,
            entry_point: self.fragment_entry_point,
            targets,
        }
    }
}
