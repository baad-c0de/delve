use std::ops::Range;

use bytemuck::{cast_slice, Pod, Zeroable};
use tracing::debug;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Device,
};

/// A buffer of data that can be sent to the GPU.
#[derive(Debug)]
pub struct Buffer {
    /// The underlying WGPU buffer.
    buffer: wgpu::Buffer,

    /// The number of elements in the buffer.
    size: usize,
}

impl Buffer {
    /// Create a new vertex buffer from the given data.
    ///
    /// # Parameters
    ///
    /// * `desc` - A description of the buffer for debugging purposes.
    /// * `device` - The WGPU device.
    /// * `data` - The data to store in the buffer.
    ///
    /// # Returns
    ///
    /// A new vertex buffer.
    ///
    /// # Panics
    ///
    /// Panics if the data is empty.
    ///
    /// # Notes
    ///
    /// The buffer is created with the `VERTEX` usage flag.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Buffer;
    /// # use wgpu::Device;
    /// # let device = Device::headless_default();
    /// let vertices = vec![
    ///    // Vertex data
    /// ];
    /// let buffer = Buffer::new_vertex_buffer("My vertex buffer", &device, &vertices);
    /// ```
    ///
    pub(crate) fn new_vertex_buffer<T>(desc: &'static str, device: &Device, data: &[T]) -> Self
    where
        T: Zeroable + Pod,
    {
        let size = data.len();
        let buffer_size = size * std::mem::size_of::<T>();
        debug!(
            "Creating buffer: {} ({} vertices, {} bytes)",
            desc, size, buffer_size
        );
        let usage = BufferUsages::VERTEX;

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(desc),
            contents: cast_slice(data),
            usage,
        });
        Self { buffer, size }
    }

    /// Create a new index buffer from the given data.
    ///
    /// # Parameters
    ///
    /// * `desc` - A description of the buffer for debugging purposes.
    /// * `device` - The WGPU device.
    /// * `indices` - The indices to store in the buffer.
    ///
    /// # Returns
    ///
    /// A new index buffer.
    ///
    /// # Notes
    ///
    /// The buffer is created with the `INDEX` usage flag.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Buffer;
    /// # use wgpu::Device;
    /// # let device = Device::headless_default();
    /// let indices = vec![
    ///   // Index data
    /// ];
    /// let buffer = Buffer::new_index_buffer("My index buffer", &device, &indices);
    /// ```
    ///
    pub(crate) fn new_index_buffer(desc: &'static str, device: &Device, indices: &[u16]) -> Self {
        let size = indices.len();
        let buffer_size = size * std::mem::size_of::<u16>();
        debug!(
            "Creating buffer: {} ({} indices, {} bytes)",
            desc, size, buffer_size
        );
        let usage = BufferUsages::INDEX;

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(desc),
            contents: cast_slice(indices),
            usage,
        });
        Self { buffer, size }
    }

    /// Returns the underlying WGPU buffer.
    ///
    /// # Returns
    ///
    /// The underlying WGPU buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Buffer;
    /// # use wgpu::Device;
    /// # let device = Device::headless_default();
    /// let vertices = vec![
    ///   // Vertex data
    /// ];
    /// let buffer = Buffer::new_vertex_buffer("My vertex buffer", &device, &vertices);
    /// let wgpu_buffer = buffer.wgpu_buffer();
    /// ```
    ///
    /// # Notes
    ///
    /// This is useful for creating bind groups and setting up the render passes.
    ///
    /// # See Also
    ///
    /// * [wgpu::Buffer](https://docs.rs/wgpu/latest/wgpu/struct.Buffer.html)
    ///
    pub(crate) fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    /// Returns the number of elements in the buffer.
    ///
    /// # Returns
    ///
    /// The number of elements in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Buffer;
    /// # use wgpu::Device;
    /// # let device = Device::headless_default();
    /// let vertices = vec![
    ///  // Vertex data
    /// ];
    /// let buffer = Buffer::new_vertex_buffer("My vertex buffer", &device, &vertices);
    /// let len = buffer.len();
    /// ```
    ///
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns the complete range of indices in the buffer.
    ///
    /// # Returns
    ///
    /// The complete range of indices in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gfx::Buffer;
    /// # use wgpu::Device;
    /// # let device = Device::headless_default();
    /// let vertices = vec![
    /// // Vertex data
    /// ];
    /// let buffer = Buffer::new_vertex_buffer("My vertex buffer", &device, &vertices);
    /// let range = buffer.all();
    /// ```
    ///
    pub fn all(&self) -> Range<u32> {
        0..(self.size as u32)
    }
}
