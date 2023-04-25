use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use tracing::info;
use wgpu::{
    Backends, Device, DeviceDescriptor, DeviceType, Dx12Compiler, Features, Instance, Limits,
    Queue, ShaderModuleDescriptor, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};

use super::{render_pipeline::RenderPipelineBuilder, Buffer, Frame, GfxError, Material};

pub struct Screen<'window> {
    surface: Surface,
    surface_config: SurfaceConfiguration,
    surface_size: (u32, u32),
    device: Device,
    queue: Queue,
    window_lifetime: PhantomData<&'window ()>,
}

impl<'window> Screen<'window> {
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

    pub fn create_render_pipeline(&self, pipeline_desc: &'static str) -> RenderPipelineBuilder {
        RenderPipelineBuilder::new(pipeline_desc)
    }

    pub fn create_vertex_buffer<T>(&self, desc: &'static str, data: &[T]) -> Buffer
    where
        T: Pod + Zeroable,
    {
        Buffer::new_vertex_buffer(desc, &self.device, data)
    }

    pub fn create_index_buffer(&self, desc: &'static str, data: &[u16]) -> Buffer {
        Buffer::new_index_buffer(desc, &self.device, data)
    }

    pub fn start_frame(&self, frame_desc: &'static str) -> Result<Frame, GfxError> {
        Frame::new(&self.device, &self.surface, frame_desc).map_err(GfxError::from)
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn get_queue(&self) -> &Queue {
        &self.queue
    }

    pub fn get_surface_format(&self) -> TextureFormat {
        self.surface_config.format
    }

    /// Resize the surface.
    ///
    /// This is used when the window is resized.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface_size = (width, height);
            self.recreate();
        }
    }

    /// Recreate the surface.
    ///
    /// This is used when the surface is lost, such as when the window is
    /// minimized. It is also used when the window is resized.
    ///
    pub fn recreate(&mut self) {
        self.surface.configure(&self.device, &self.surface_config);
    }
}
