mod gfx;

use std::env::set_var;

use color_eyre::{eyre::Context, Report};
use gfx::{GfxError, RenderPipeline};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use wgpu::{include_wgsl, Color, SurfaceError};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::gfx::Screen;

#[tokio::main]
async fn main() -> Result<(), Report> {
    //
    // Set up error handling and logging
    //

    // Install color_eyre to get prettier error messages
    color_eyre::install().context("initialising color_eyre")?;

    set_var("RUST_LOG", "delve=debug,wgpu=warn");

    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting game...");

    //
    // Set up window
    //

    let event_loop = EventLoop::new();
    let window_size = PhysicalSize::new(1024, 768);
    let window = WindowBuilder::new()
        .with_title("Delve (Mage Engine)")
        .with_inner_size(window_size)
        .build(&event_loop)
        .context("creating primary window")?;

    //
    // Set up graphics system
    //

    let mut screen = Screen::new(&window, window_size.width, window_size.height).await?;

    // Load the shader module for our render pipeline.
    //
    // This is the shader that will be used to render our frames. It is
    // written in the WebGPU Shader Language (WGSL), which is a new language
    // that is designed to be easy to read and write.
    //
    // The shader is loaded from a file using the `include_wgsl!` macro.
    // This macro will compile the shader at compile time, and embed the
    // compiled shader into the binary. This means that we don't need to
    // worry about loading the shader at runtime.
    let triangle_material =
        screen.create_material(include_wgsl!("shader.wgsl"), "vs_main", "fs_main");

    let render_pipeline = screen
        .create_render_pipeline("triangle render")
        .vertex_shader(&triangle_material)
        .fragment_shader(&triangle_material)
        .build(&screen)?;

    //
    // Main loop
    //

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
                    screen.resize(width, height);
                }

                _ => {}
            },

            Event::RedrawRequested(_) => match render(&screen, &render_pipeline) {
                Ok(_) => {}
                Err(GfxError::BadRender(SurfaceError::Lost)) => screen.recreate(),
                Err(GfxError::BadRender(SurfaceError::OutOfMemory)) => {
                    *control_flow = ControlFlow::Exit
                }
                Err(e) => error!("Error rendering: {}", e),
            },

            _ => {}
        }
    });
}

fn render(screen: &Screen, pipeline: &RenderPipeline) -> Result<(), GfxError> {
    let mut frame = screen.start_frame("Main frame")?;

    {
        let mut render_pass = frame.create_render_pass(
            "Main render pass",
            Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        );

        render_pass.set_pipeline(pipeline);
        render_pass.draw(0..3);
    }

    frame.finish(screen.get_queue());

    Ok(())
}
