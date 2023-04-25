use std::ops::RangeBounds;

use wgpu::{
    BufferAddress, Color, CommandEncoder, IndexFormat, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, TextureView,
};

use super::{render_pipeline::RenderPipeline, Buffer};

/// A render pass.
///
/// # Notes
///
/// A render pass is a collection of commands that are sent to the GPU to render a scene.
///
pub struct RenderPass<'encoder> {
    /// The underlying WGPU render pass.
    render_pass: wgpu::RenderPass<'encoder>,
}

impl<'encoder> RenderPass<'encoder> {
    /// Creates a new render pass.
    ///
    /// # Parameters
    ///
    /// * `encoder` - The command encoder.
    /// * `view` - The texture view.
    /// * `desc` - The description for debugging purposes.
    /// * `back_colour` - The background colour.
    ///
    /// # Returns
    ///
    /// The new render pass.
    ///
    pub(crate) fn new(
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

    /// Sets the render pipeline for the render pass.
    ///
    /// # Parameters
    ///
    /// * `pipeline` - The render pipeline.
    ///
    pub fn set_pipeline<'pipeline: 'encoder>(&mut self, pipeline: &'pipeline RenderPipeline) {
        self.render_pass
            .set_pipeline(pipeline.get_render_pipeline());
    }

    /// Sets the vertex buffer for the render pass for the given slot.
    ///
    /// # Parameters
    ///
    /// * `slot` - The slot.
    /// * `buffer` - The buffer containing the vertices.
    /// * `range` - The range of vertices to use.  You can use `..` for this if
    ///   you want all of the vertices.
    ///
    /// # Notes
    ///
    /// This is a wrapper around `wgpu::RenderPass::set_vertex_buffer`.
    ///
    pub fn set_vertex_buffer<R>(&mut self, slot: u32, buffer: &'encoder Buffer, range: R)
    where
        R: RangeBounds<BufferAddress>,
    {
        self.render_pass
            .set_vertex_buffer(slot, buffer.wgpu_buffer().slice(range));
    }

    /// Sets the index buffer for the render pass.
    ///
    /// # Parameters
    ///
    /// * `buffer` - The buffer containing the indices.
    /// * `range` - The range of indices to use.  You can use `..` for this if
    ///   you want all of the indices.
    ///
    /// # Notes
    ///
    /// This is a wrapper around `wgpu::RenderPass::set_index_buffer`.
    ///
    pub fn set_index_buffer<R>(&mut self, buffer: &'encoder Buffer, range: R)
    where
        R: RangeBounds<BufferAddress>,
    {
        self.render_pass
            .set_index_buffer(buffer.wgpu_buffer().slice(range), IndexFormat::Uint16);
    }

    /// Draws the given range of vertices.
    ///
    /// # Parameters
    ///
    /// * `vertices` - The range of vertices to draw.  Typical use is `0..buffer.len()`.
    ///
    /// # Notes
    ///
    /// This is a wrapper around `wgpu::RenderPass::draw`.
    ///
    pub fn draw(&mut self, vertices: std::ops::Range<u32>) {
        self.render_pass.draw(vertices, 0..1);
    }

    /// Draws the given range of vertices with the given range of indices.
    ///
    /// # Parameters
    ///
    /// * `indices` - The range of indices to draw.  Typical use is `0..buffer.len()`.
    ///
    /// # Notes
    ///
    /// This is a wrapper around `wgpu::RenderPass::draw_indexed`.
    ///
    pub fn draw_indexed(&mut self, indices: std::ops::Range<u32>) {
        self.render_pass.draw_indexed(indices, 0, 0..1);
    }
}
