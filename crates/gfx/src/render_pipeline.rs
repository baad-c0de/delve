use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor,
};

use super::{GfxError, Material, Screen};

pub struct RenderPipelineBuilder<'material> {
    desc: &'static str,
    shader: Option<&'material Material<'material>>,
}

#[derive(Debug)]
pub struct RenderPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl<'material> RenderPipelineBuilder<'material> {
    pub fn new(desc: &'static str) -> Self {
        Self { desc, shader: None }
    }

    pub fn shader(mut self, material: &'material Material<'material>) -> Self {
        self.shader = Some(material);
        self
    }

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
    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }
}
