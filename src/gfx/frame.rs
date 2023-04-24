use wgpu::{
    Color, CommandEncoder, CommandEncoderDescriptor, Device, Queue, Surface, SurfaceTexture,
    TextureView, TextureViewDescriptor,
};

use super::{GfxError, RenderPass};

pub struct Frame {
    texture: SurfaceTexture,
    texture_view: TextureView,
    encoder: CommandEncoder,
}

impl Frame {
    pub fn new(device: &Device, surface: &Surface, encoder_desc: &str) -> Result<Frame, GfxError> {
        let texture = surface.get_current_texture()?;
        let texture_view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some(encoder_desc),
        });

        Ok(Frame {
            texture,
            texture_view,
            encoder,
        })
    }

    pub fn create_render_pass(&mut self, render_pass_desc: &str, back_colour: Color) -> RenderPass {
        RenderPass::new(
            &mut self.encoder,
            &self.texture_view,
            render_pass_desc,
            back_colour,
        )
    }

    pub fn finish(self, queue: &Queue) {
        queue.submit(std::iter::once(self.encoder.finish()));
        self.texture.present();
    }
}
