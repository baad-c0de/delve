use wgpu::{
    Color, CommandEncoder, CommandEncoderDescriptor, Device, Queue, Surface, SurfaceTexture,
    TextureView, TextureViewDescriptor,
};

use super::{GfxError, RenderPass};

/// A frame that can be rendered to.
///
/// # Notes
///
/// This is a wrapper around a WGPU surface texture, texture view, and command encoder.
///
/// # Examples
///
/// ```
/// # use gfx::Frame;
/// # use wgpu::Device;
/// # let device = Device::headless_default();
/// # let surface = device.create_surface(&wgpu::SurfaceConfiguration::default());
/// let mut frame = Frame::new(&device, &surface, "My frame").unwrap();
/// let render_pass = frame.create_render_pass("My render pass", wgpu::Color::BLACK);
/// // Render to the frame...
/// frame.finish(&device.get_queue());
/// ```
///
pub struct Frame<'queue> {
    /// The surface texture.
    ///
    /// # Notes
    ///
    /// This is the texture that is presented to the screen.
    ///
    /// # See Also
    ///
    /// * [wgpu::SurfaceTexture](https://docs.rs/wgpu/latest/wgpu/struct.SurfaceTexture.html)
    ///
    texture: SurfaceTexture,

    /// The texture view.
    ///
    /// # Notes
    ///
    /// This is the texture view that is rendered to.  A view is an abstraction that allows
    /// a section of a texture to be used as the destination for rendering.
    ///
    /// # See Also
    ///
    /// * [wgpu::TextureView](https://docs.rs/wgpu/latest/wgpu/struct.TextureView.html)
    ///
    texture_view: TextureView,

    /// The command encoder.
    ///
    /// # Notes
    ///
    /// This is the command encoder that is used to encode the commands that are sent to the GPU.
    ///
    /// # See Also
    ///
    /// * [wgpu::CommandEncoder](https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html)
    ///
    encoder: CommandEncoder,

    /// The queue that will eventually execute the commands.
    ///
    /// # Notes
    ///
    /// This is the queue that will eventually execute the commands that are encoded by the
    /// command encoder.
    ///
    /// # See Also
    ///
    /// * [wgpu::Queue](https://docs.rs/wgpu/latest/wgpu/struct.Queue.html)
    ///
    queue: &'queue Queue,
}

impl<'queue> Frame<'queue> {
    /// Creates a new frame.
    ///
    /// # Notes
    ///
    /// This is a wrapper around a WGPU surface texture, texture view, and
    /// command encoder. A frame represents a single frame that can be rendered
    /// to.
    ///
    /// # Parameters
    ///
    /// * `device` - The WGPU device.
    /// * `surface` - The WGPU surface.
    /// * `encoder_desc` - The encoder description for debugging purposes.
    ///
    /// # Returns
    ///
    /// The new frame.
    ///
    /// # Errors
    ///
    /// * `GfxError::SurfaceError` - If the surface is invalid.
    /// * `GfxError::DeviceError` - If the device is invalid.
    ///
    pub(crate) fn new(
        device: &Device,
        queue: &'queue Queue,
        surface: &Surface,
        encoder_desc: &str,
    ) -> Result<Frame<'queue>, GfxError> {
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
            queue: queue,
        })
    }

    /// Creates a new render pass.
    ///
    /// # Parameters
    ///
    /// * `render_pass_desc` - The render pass description for debugging purposes.
    /// * `back_colour` - The background colour.
    ///
    /// # Returns
    ///
    /// The new render pass.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Frame;
    /// # use wgpu::Device;
    /// # let device = Device::headless_default();
    /// # let surface = device.create_surface(&wgpu::SurfaceConfiguration::default());
    /// let mut frame = Frame::new(&device, &surface, "My frame").unwrap();
    /// let render_pass = frame.create_render_pass("My render pass", wgpu::Color::BLACK);
    /// ```
    ///
    /// # See Also
    ///
    /// * [wgpu::RenderPass](https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html)
    ///
    /// * [wgpu::Color](https://docs.rs/wgpu/latest/wgpu/struct.Color.html)
    ///
    /// * [wgpu::RenderPassDescriptor](https://docs.rs/wgpu/latest/wgpu/struct.RenderPassDescriptor.html)
    ///
    pub fn create_render_pass(&mut self, render_pass_desc: &str, back_colour: Color) -> RenderPass {
        RenderPass::new(
            &mut self.encoder,
            &self.texture_view,
            render_pass_desc,
            back_colour,
        )
    }

    /// Finishes the frame.
    ///
    /// # Notes
    ///
    /// This submits the command encoder to the GPU and presents the frame to the screen.
    /// The frame is consumed by this method.
    ///
    pub fn finish(self) {
        self.queue.submit(std::iter::once(self.encoder.finish()));
        self.texture.present();
    }
}
