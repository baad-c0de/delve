use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use tracing::info;
use wgpu::{
    Backends, Device, DeviceDescriptor, DeviceType, Dx12Compiler, Features, Instance, Limits,
    Queue, ShaderModuleDescriptor, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};

use super::{render_pipeline::RenderPipelineBuilder, Buffer, Frame, GfxError, Material};

/// The main interface to the gfx library.
///
/// This is the main interface to the gfx library.  It is used to create
/// render pipelines and render frames.
///
/// You can create a screen using the [Screen::new] method.
///
/// # Examples
///
/// ```
/// # use gfx::Screen;
/// # async fn example() -> Result<(), gfx::GfxError> {
/// let screen = Screen::new()?;
/// # Ok(())
/// # }
/// ```
///
pub struct Screen<'window> {
    /// The WGPU surface.
    surface: Surface,

    /// The WGPU surface configuration.
    surface_config: SurfaceConfiguration,

    /// The size (in pixels) of the surface.
    surface_size: (u32, u32),

    /// The WGPU device.
    device: Device,

    /// The WGPU queue.
    queue: Queue,

    /// Used to tie the lifetime of the screen object to the lifetime of the
    /// window.
    ///
    /// This is needed because the WGPU surface is created from the window via
    /// the raw window handle, and the raw window handle is not guaranteed to
    /// live as long as the screen otherwise.
    window_lifetime: PhantomData<&'window ()>,
}

impl<'window> Screen<'window> {
    /// Creates a new screen.
    ///
    /// # Parameters
    ///
    /// * `window` - The window to create the screen for.
    /// * `width` - The width of the screen (in pixels).
    /// * `height` - The height of the screen (in pixels).
    ///
    /// # Returns
    ///
    /// The new screen.
    ///
    /// # Notes
    ///
    /// This method is asynchronous because it creates a WGPU device and queue, which
    /// are asynchronous operations.  Therefore, this method must be called within an async
    /// runtime like tokio or pollster, etc.
    ///
    pub async fn new<W>(window: W, width: u32, height: u32) -> Result<Screen<'window>, GfxError>
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        // Create a WGPU instance.
        //
        // This is the entry point to the WGPU API. It is used to create
        // devices, buffers, textures, and more.
        //
        let backends = Backends::PRIMARY;
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: Dx12Compiler::default(),
        });

        // Create a WGPU surface.
        //
        // This is the surface that we will render to. It is created from a raw
        // window handle, which is provided by the windowing library.
        //
        // The surface is used to create swap chains, which are used to present
        // rendered frames to the screen.
        //
        let surface = unsafe { instance.create_surface(&window) }?;

        // Find a suitable GPU adapter.
        //
        // This is the GPU that we will use to render our frames. We want to
        // find a discrete GPU, as this is the most powerful type of GPU
        // available. We also want to make sure that the adapter supports the
        // surface that we created earlier.
        //
        let mut adapter_list = instance.enumerate_adapters(backends).filter(|adapter| {
            adapter.is_surface_supported(&surface)
                && adapter.get_info().device_type == DeviceType::DiscreteGpu
        });

        // If we couldn't find a suitable adapter, then we can't continue.
        let adapter = adapter_list.next().ok_or(GfxError::NoSuitableAdapter)?;
        info!("Using GPU: {}", adapter.get_info().name);

        // Create a WGPU device and queue.
        //
        // The device is used to create buffers, textures, and other resources.
        // The queue is used to submit commands to the device.
        //
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some(&format!("Device for {}", adapter.get_info().name)),
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await?;

        // Figure out the surface capabilities when using the adapter.  We will
        // use this to find the format that allows sRGB textures.
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|format| format.describe().srgb)
            .ok_or(GfxError::NoSuitableSurfaceFormat)?;
        info!("Surface format: {:?}", surface_format);
        info!("Surface present modes: {:?}", surface_caps.present_modes);
        info!("Surface alpha modes: {:?}", surface_caps.alpha_modes);

        // Now we have the format we can create the surface configuration.  This
        // encapsulates the surface format, the size of the surface, the present
        // and alpha modes, and other information.
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: *surface_format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        Ok(Screen {
            window_lifetime: PhantomData,
            surface,
            surface_config,
            surface_size: (width, height),
            device,
            queue,
        })
    }

    /// Creates a new material from a WGPU ShaderModuleDescriptor.
    ///
    /// # Parameters
    ///
    /// * `shader` - The shader module descriptor. This can be created via the
    ///   [`include_wgsl!`] macro.
    /// * `vertex_entry_point` - The name of the shader's vertex entry point
    ///   function.
    /// * `fragment_entry_point` - The name of the shader's fragment entry point
    ///   function.
    ///
    /// # Returns
    ///
    /// The new material.
    ///
    /// # Notes
    ///
    /// This will call [`Material::new`] to create the material internally.
    ///
    /// [`Material::new`]: struct.Material.html#method.new
    /// [`include_wgsl!`]: https://docs.rs/wgpu/latest/wgpu/macro.include_wgsl.html
    ///
    pub fn create_material(
        &self,
        shader: ShaderModuleDescriptor,
        vertex_entry_point: &'static str,
        fragment_entry_point: &'static str,
    ) -> Material {
        Material::new(
            &self.device,
            shader,
            vertex_entry_point,
            fragment_entry_point,
        )
    }

    /// Creates a new render pipeline builder.
    ///
    /// # Parameters
    ///
    /// * `pipeline_desc` - The name of the pipeline for debugging purposes.
    ///
    /// # Returns
    ///
    /// The new render pipeline builder.
    ///
    /// # Notes
    ///
    /// This will call [`RenderPipelineBuilder::new`] to create the builder
    /// that you can attach shaders to.  Finally, you can call
    /// [`RenderPipelineBuilder::build`] to create the render pipeline.
    ///
    /// [`RenderPipelineBuilder::new`]: struct.RenderPipelineBuilder.html#method.new
    /// [`RenderPipelineBuilder::build`]: struct.RenderPipelineBuilder.html#method.build
    ///
    pub fn create_render_pipeline(&self, pipeline_desc: &'static str) -> RenderPipelineBuilder {
        RenderPipelineBuilder::new(pipeline_desc)
    }

    /// Creates a new vertex buffer.
    ///
    /// # Parameters
    ///
    /// * `desc` - The name of the vertex buffer for debugging purposes.
    /// * `data` - The vertex data.
    ///
    /// # Returns
    ///
    /// The new vertex buffer.
    ///
    /// # Notes
    ///
    /// This will call [`Buffer::new_vertex_buffer`] to create the vertex buffer.
    ///
    /// [`Buffer::new_vertex_buffer`]: struct.Buffer.html#method.new_vertex_buffer
    ///
    pub fn create_vertex_buffer<T>(&self, desc: &'static str, data: &[T]) -> Buffer
    where
        T: Pod + Zeroable,
    {
        Buffer::new_vertex_buffer(desc, &self.device, data)
    }

    /// Creates a new index buffer.
    ///
    /// # Parameters
    ///
    /// * `desc` - The name of the index buffer for debugging purposes.
    /// * `data` - The index data.
    ///
    /// # Returns
    ///
    /// The new index buffer.
    ///
    /// # Notes
    ///
    /// This will call [`Buffer::new_index_buffer`] to create the index buffer.
    ///
    /// [`Buffer::new_index_buffer`]: struct.Buffer.html#method.new_index_buffer
    ///
    pub fn create_index_buffer(&self, desc: &'static str, data: &[u16]) -> Buffer {
        Buffer::new_index_buffer(desc, &self.device, data)
    }

    /// Creates a new [Frame] that can be used to render to the screen.
    ///
    /// # Parameters
    ///
    /// * `frame_desc` - The name of the frame for debugging purposes.
    ///
    /// # Returns
    ///
    /// The new frame.
    ///
    /// # Notes
    ///
    /// This will call [`Frame::new`] to create the frame.
    ///
    /// [Frame]: struct.Frame.html
    ///
    /// [`Frame::new`]: struct.Frame.html#method.new
    ///
    pub fn start_frame(&self, frame_desc: &'static str) -> Result<Frame, GfxError> {
        Frame::new(&self.device, &self.queue, &self.surface, frame_desc).map_err(GfxError::from)
    }

    pub(crate) fn get_device(&self) -> &Device {
        &self.device
    }

    pub(crate) fn get_surface_format(&self) -> TextureFormat {
        self.surface_config.format
    }

    /// Resizes the surface.
    ///
    /// # Parameters
    ///
    /// * `width` - The new width of the surface.
    /// * `height` - The new height of the surface.
    ///
    /// # Notes
    ///
    /// This will call [`Screen::recreate`] to recreate the surface.  This
    /// should be called every time the window is resized.
    ///
    /// [`Screen::recreate`]: struct.Screen.html#method.recreate
    ///
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface_size = (width, height);
            self.recreate();
        }
    }

    /// Recreates the surface.
    ///
    /// # Notes
    ///
    /// This should be called whenever a frane is started and an error is
    /// returned stating that the surface is lost.
    ///
    pub fn recreate(&mut self) {
        self.surface.configure(&self.device, &self.surface_config);
    }
}
