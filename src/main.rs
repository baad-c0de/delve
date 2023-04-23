mod gfx;

use std::env::set_var;

use color_eyre::{eyre::Context, Report};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use wgpu::SurfaceError;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::gfx::{GfxError, GfxState};

#[tokio::main]
async fn main() -> Result<(), Report> {
    // Install color_eyre to get prettier error messages
    color_eyre::install().context("initialising color_eyre")?;

    set_var("RUST_LOG", "delve=debug,wgpu=warn");

    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting game...");

    let event_loop = EventLoop::new();
    let window_size = PhysicalSize::new(1024, 768);
    let window = WindowBuilder::new()
        .with_title("Delve (Mage Engine)")
        .with_inner_size(window_size)
        .build(&event_loop)
        .context("creating primary window")?;

    let mut gfx = GfxState::new(&window, window_size.width, window_size.height).await?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::Resized(PhysicalSize { width, height })
                | WindowEvent::ScaleFactorChanged {
                    new_inner_size: &mut PhysicalSize { width, height },
                    ..
                } => {
                    gfx.resize(width, height);
                }

                _ => {}
            },

            Event::RedrawRequested(_) => match gfx.render() {
                Ok(_) => {}
                Err(GfxError::BadRender(SurfaceError::Lost)) => gfx.recreate(),
                Err(GfxError::BadRender(SurfaceError::OutOfMemory)) => {
                    *control_flow = ControlFlow::Exit
                }
                Err(e) => error!("Error rendering: {}", e),
            },

            _ => {}
        }
    });
}
