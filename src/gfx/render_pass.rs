use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    TextureView,
};

use super::render_pipeline::RenderPipeline;

pub struct RenderPass<'encoder> {
    render_pass: wgpu::RenderPass<'encoder>,
}

impl<'encoder> RenderPass<'encoder> {
    pub fn new(
        encoder: &'encoder mut CommandEncoder,
        view: &'encoder TextureView,
        desc: &str,
        back_colour: Color,
    ) -> Self {
        let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some(desc),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(back_colour),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        Self { render_pass }
    }

    pub fn set_pipeline<'pipeline: 'encoder>(&mut self, pipeline: &'pipeline RenderPipeline) {
        self.render_pass
            .set_pipeline(pipeline.get_render_pipeline());
    }

    pub fn draw(&mut self, vertices: std::ops::Range<u32>) {
        self.render_pass.draw(vertices, 0..1);
    }
}
