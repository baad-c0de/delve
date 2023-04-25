use std::ops::Range;

use bytemuck::{cast_slice, Pod, Zeroable};
use tracing::debug;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Device,
};

#[derive(Debug)]
pub struct Buffer {
    buffer: wgpu::Buffer,
    size: usize,
}

impl Buffer {
    pub fn new_vertex_buffer<T>(desc: &'static str, device: &Device, data: &[T]) -> Self
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

    pub fn new_index_buffer(desc: &'static str, device: &Device, indices: &[u16]) -> Self {
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

    pub fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn all(&self) -> Range<u32> {
        0..(self.size as u32)
    }
}
