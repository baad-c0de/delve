use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor,
};

use super::{GfxError, Material, Screen};

pub struct RenderPipelineBuilder<'v_material, 'f_material> {
    desc: &'static str,
    vertex_shader: Option<&'v_material Material>,
    fragment_shader: Option<&'f_material Material>,
}

pub struct RenderPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl<'v_material, 'f_material> RenderPipelineBuilder<'v_material, 'f_material> {
    pub fn new(desc: &'static str) -> Self {
        Self {
            desc,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn vertex_shader(mut self, material: &'v_material Material) -> Self {
        self.vertex_shader = Some(material);
        self
    }

    pub fn fragment_shader(mut self, material: &'f_material Material) -> Self {
        self.fragment_shader = Some(material);
        self
    }

    pub fn build(self, screen: &Screen) -> Result<RenderPipeline, GfxError> {
        let vertex_shader = self
            .vertex_shader
            .ok_or(GfxError::BadMaterialMissingShaders)?;
        let fragment_shader = self
            .fragment_shader
            .ok_or(GfxError::BadMaterialMissingShaders)?;

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
                    vertex: vertex_shader.vertex_state(),
                    fragment: Some(fragment_shader.fragment_state(targets)),
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
