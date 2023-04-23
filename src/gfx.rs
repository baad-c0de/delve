use std::{iter::once, marker::PhantomData};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use thiserror::Error;
use tracing::info;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, DeviceType, Dx12Compiler,
    Features, Instance, Limits, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, Surface, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};

pub struct GfxState<'window> {
    surface: Surface,
    surface_config: SurfaceConfiguration,
    surface_size: (u32, u32),
    device: Device,
    queue: Queue,
    window_lifetime: PhantomData<&'window ()>,
}

#[derive(Debug, Error)]
pub enum GfxError {
    #[error("failed to create WGPU surface")]
    SurfaceCreation(#[from] wgpu::CreateSurfaceError),

    #[error("failed to find a suitable GPU adapter")]
    NoSuitableAdapter,

    #[error("failed to create WGPU device")]
    DeviceCreation(#[from] wgpu::RequestDeviceError),

    #[error("failed to find a suitable surface format for sRGB")]
    NoSuitableSurfaceFormat,

    #[error("rendering to a surface failed")]
    BadRender(#[from] wgpu::SurfaceError),
}

impl<'window> GfxState<'window> {
    pub async fn new<W>(window: W, width: u32, height: u32) -> Result<GfxState<'window>, GfxError>
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

        Ok(GfxState {
            surface,
            surface_config,
            surface_size: (width, height),
            device,
            queue,
            window_lifetime: PhantomData,
        })
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

    pub fn render(&mut self) -> Result<(), GfxError> {
        // Get the next texture to render to.
        let frame = self.surface.get_current_texture()?;

        // Create a view of the texture.  Default is fine for now as this covers
        // the whole texture.
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        // Create a render pass encoder.
        //
        // This is used to record the commands that will be used to render the
        // frame.  We can record multiple render passes in a single encoder.
        //
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // Submit the commands to the queue.
        self.queue.submit(once(encoder.finish()));
        frame.present();

        Ok(())
    }
}
