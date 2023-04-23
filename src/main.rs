use color_eyre::{eyre::Context, Report};
use tracing::info;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Report> {
    // Install color_eyre to get prettier error messages
    color_eyre::install().context("initialising color_eyre")?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .without_time()
        .init();

    info!("Starting game...");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Delve")
        .with_inner_size(PhysicalSize::new(1024, 768))
        .build(&event_loop)
        .context("creating primary window")?;

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
                _ => {}
            },
            _ => {}
        }
    });
}
