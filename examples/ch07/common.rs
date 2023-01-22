use std:: {iter, mem};
use cgmath::Matrix4;
use wgpu::Util::DeviceExt;
use winit::{
    event::*,
    window::Window,
    event_loop::{ControlFlow, EventLoop},
};
use bytemuck:: {Pod, Zeroable, cast_slice};

#[path="../common/transforms.rs"]
mod transforms;

const ANIMATION_SPEED:f32 = 1.0;
const IS_PERSPECTIVE:bool = true;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroables)]
pub struct Vertex {
    position: [f32; 4],
}

pub fn vertex(p:[f32;3]) -> Vertex {
    Vertex {
        position: [p[0], p[1], p[2], 1.0],
    }
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}