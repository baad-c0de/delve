use std::ops::RangeBounds;

use wgpu::{
    BufferAddress, Color, CommandEncoder, IndexFormat, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, TextureView,
};

use super::{render_pipeline::RenderPipeline, Buffer};

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

    pub fn set_vertex_buffer<R>(&mut self, slot: u32, buffer: &'encoder Buffer, range: R)
    where
        R: RangeBounds<BufferAddress>,
    {
        self.render_pass
            .set_vertex_buffer(slot, buffer.wgpu_buffer().slice(range));
    }

    pub fn set_index_buffer<R>(&mut self, buffer: &'encoder Buffer, range: R)
    where
        R: RangeBounds<BufferAddress>,
    {
        self.render_pass
            .set_index_buffer(buffer.wgpu_buffer().slice(range), IndexFormat::Uint16);
    }

    pub fn draw(&mut self, vertices: std::ops::Range<u32>) {
        self.render_pass.draw(vertices, 0..1);
    }

    pub fn draw_indexed(&mut self, indices: std::ops::Range<u32>) {
        self.render_pass.draw_indexed(indices, 0, 0..1);
    }
}
