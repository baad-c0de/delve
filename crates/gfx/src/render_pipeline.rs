use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor,
};

use super::{GfxError, Material, Screen};

/// A render pipeline builder.
///
/// # Notes
///
/// This is a builder for a render pipeline.  It is used to create a render pipeline
/// from a material.
///
/// You can create the render pipeline using the [RenderPipelineBuilder::build] method.
///
pub struct RenderPipelineBuilder<'material> {
    desc: &'static str,
    shader: Option<&'material Material<'material>>,
}

/// A render pipeline.
///
/// # Notes
///
/// This is a render pipeline.  It is used to render a frame.
///
/// You can set the render pipeline for a render pass using the
/// [RenderPass::set_pipeline] method.
///
/// You can get the render pipeline from a render pipeline builder using the
/// [RenderPipelineBuilder::build] method.
///
/// [RenderPass::set_pipeline]: struct.RenderPass.html#method.set_pipeline
/// [RenderPipelineBuilder::build]: struct.RenderPipelineBuilder.html#method.build
///
#[derive(Debug)]
pub struct RenderPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl<'material> RenderPipelineBuilder<'material> {
    /// Creates a new render pipeline builder.
    ///
    /// # Parameters
    ///
    /// * `desc` - The description of the render pipeline for debugging purposes.
    ///
    /// # Returns
    ///
    /// The new render pipeline builder.
    ///
    pub(crate) fn new(desc: &'static str) -> Self {
        Self { desc, shader: None }
    }

    /// Sets the material for the render pipeline.
    ///
    /// # Parameters
    ///
    /// * `material` - The material.
    ///
    /// # Returns
    ///
    /// The render pipeline builder with the material set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Material;
    /// # let material = Material::new();
    /// # let screen = gfx::Screen::new();
    /// let render_pipeline = gfx::RenderPipelineBuilder::new("Render pipeline")
    ///     .shader(&material)
    ///     .build(&screen)
    ///     .unwrap();
    /// ```
    ///
    pub fn shader(mut self, material: &'material Material<'material>) -> Self {
        self.shader = Some(material);
        self
    }

    /// Builds the render pipeline.
    ///
    /// # Parameters
    ///
    /// * `screen` - The screen.
    ///
    /// # Returns
    ///
    /// The render pipeline if it was built successfully.
    ///
    /// # Errors
    ///
    /// If the material was not set with the [RenderPipelineBuilder::shader] method, then
    /// this will return an error of type [GfxError::BadMaterialMissingShaders].
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Material;
    /// # let material = Material::new();
    /// # let screen = gfx::Screen::new();
    /// let render_pipeline = gfx::RenderPipelineBuilder::new("Render pipeline")
    ///     .shader(&material)
    ///     .build(&screen)
    ///     .unwrap();
    /// ```
    ///
    pub fn build(self, screen: &Screen) -> Result<RenderPipeline, GfxError> {
        let shader = self.shader.ok_or(GfxError::BadMaterialMissingShaders)?;

        let render_pipeline_layout =
            screen
                .get_device()
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("Render pipeline layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        // Create the render pipeline.
        //
        // This is the pipeline that will be used to render our frames. It
        // specifies the shader that will be used, the layout of the vertex
        // buffers, and the layout of the render targets.

        let targets = &[Some(ColorTargetState {
            format: screen.get_surface_format(),
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL,
        })];

        let render_pipeline =
            screen
                .get_device()
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: Some(self.desc),
                    layout: Some(&render_pipeline_layout),
                    vertex: shader.vertex_state(),
                    fragment: Some(shader.fragment_state(targets)),
                    primitive: PrimitiveState {
                        topology: PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: FrontFace::Ccw,
                        cull_mode: Some(Face::Back),
                        polygon_mode: PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        Ok(RenderPipeline { render_pipeline })
    }
}

impl RenderPipeline {
    /// Gets the render pipeline.
    ///
    /// # Returns
    ///
    /// The underlying WGPU render pipeline.
    ///
    pub(crate) fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }
}
