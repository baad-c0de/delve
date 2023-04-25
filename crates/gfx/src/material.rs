use wgpu::{
    ColorTargetState, Device, FragmentState, ShaderModule, ShaderModuleDescriptor,
    VertexBufferLayout, VertexState,
};

/// A material.
///
/// # Notes
///
/// This is a wrapper around a WGPU shader module and a list of vertex buffer layouts.
/// A material represents a set of shaders and vertex buffer layouts that can be used
/// to render a mesh.
///
/// # See Also
///
/// * [wgpu::ShaderModule](https://docs.rs/wgpu/latest/wgpu/struct.ShaderModule.html)
///
#[derive(Debug)]
pub struct Material<'layout> {
    shader: ShaderModule,
    vertex_entry_point: &'static str,
    fragment_entry_point: &'static str,
    buffer_layouts: Vec<VertexBufferLayout<'layout>>,
}

impl<'material> Material<'material> {
    /// Creates a new material.
    ///
    /// # Notes
    ///
    /// This is a wrapper around a WGPU shader module and a list of vertex buffer layouts.
    /// A material represents a set of shaders and vertex buffer layouts that can be used
    /// to render a mesh.
    ///
    /// # Parameters
    ///
    /// * `device` - The WGPU device.
    /// * `shader` - The shader module.
    /// * `vertex_entry_point` - The name of the vertex entry point function.
    /// * `fragment_entry_point` - The name of the fragment entry point function.
    ///
    /// # Returns
    ///
    /// The new material.
    ///
    /// # See Also
    ///
    /// * [wgpu::ShaderModule](https://docs.rs/wgpu/latest/wgpu/struct.ShaderModule.html)
    ///
    /// # Examples
    ///
    /// ```
    /// # use wgpu::Device;
    /// # use gfx::Material;
    /// # let device = Device::headless_default();
    /// let material = Material::new(
    ///     &device,
    ///     include_wgsl!("shader.wgsl"),
    ///     "vs_main",
    ///     "fs_main");
    /// ```
    ///
    pub(crate) fn new(
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

    /// Returns the vertex state.
    ///
    /// # Notes
    ///
    /// This produces a vertex state that can be used to create a render pipeline.
    ///
    /// # Returns
    ///
    /// The vertex state.
    ///
    /// # See Also
    ///
    /// * [wgpu::VertexState](https://docs.rs/wgpu/latest/wgpu/struct.VertexState.html)
    ///
    pub(crate) fn vertex_state(&self) -> VertexState {
        VertexState {
            module: &self.shader,
            entry_point: self.vertex_entry_point,
            buffers: &self.buffer_layouts,
        }
    }

    /// Returns the fragment state.
    ///
    /// # Notes
    ///
    /// This produces a fragment state that can be used to create a render pipeline.
    ///
    /// # Parameters
    ///
    /// * `targets` - The color targets.
    ///
    /// # Returns
    ///
    /// The fragment state.
    ///
    /// # See Also
    ///
    /// * [wgpu::FragmentState](https://docs.rs/wgpu/latest/wgpu/struct.FragmentState.html)
    ///
    pub(crate) fn fragment_state<'a>(
        &'a self,
        targets: &'a [Option<ColorTargetState>],
    ) -> FragmentState {
        FragmentState {
            module: &self.shader,
            entry_point: self.fragment_entry_point,
            targets,
        }
    }

    /// Adds a vertex buffer layout.
    ///
    /// # Notes
    ///
    /// This adds a vertex buffer layout to the material. The vertex buffer layout
    /// describes the layout of the vertex buffer that will be used to render the
    /// mesh.
    ///
    /// This is intended to be used in a builder pattern after the creation of the
    /// material.
    ///
    /// Use the `VertexLayout` derive macro from the `wgpu_macros` crate to generate
    /// the vertex buffer layout that can be passed to this method.
    ///
    /// # Parameters
    ///
    /// * `layout` - The vertex buffer layout.
    ///
    /// # Returns
    ///
    /// The material.
    ///
    pub fn add_buffer_layout(mut self, layout: VertexBufferLayout<'material>) -> Self {
        self.buffer_layouts.push(layout);
        self
    }
}
