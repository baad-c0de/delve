use thiserror::Error;

/// An error that can occur when using the `gfx` crate.
///
/// # See Also
///
/// * [wgpu::RequestDeviceError](https://docs.rs/wgpu/latest/wgpu/enum.RequestDeviceError.html)
/// * [wgpu::SurfaceError](https://docs.rs/wgpu/latest/wgpu/enum.SurfaceError.html)
/// * [wgpu::CreateSurfaceError](https://docs.rs/wgpu/latest/wgpu/enum.CreateSurfaceError.html)
///
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

    #[error("bad material: missing vertex shader")]
    BadMaterialMissingShaders,
}
