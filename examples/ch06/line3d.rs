use std:: {iter, mem};
use wgpu::util::DeviceExt;
use cgmath::*;
use winit::{
  event::*,
  event_loop::{ControlFow, EventLoop},
  window::{Window, WindowBuilder},
};
use bytemuck:: {Pod, Zeroable, cast_slice};
#[path="../common/transforms.rs"]
mod transforms;

const IS_PERSPECTIVE:bool = false;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
  position: [f32; 3],
}

fn create_vertices() -> [Vertex; 300] {
  let mut vertices = [Vertex{position:[0.0,0.0,0.0]}; 300];
  for i in 0..300 {
    let t = 0.1*(i as f32)/30.0;
    let x = (-t).exp()*(30.0*t).sin();
    let z = (-t).exp()*(30.0*t).cos();
    let y = 2.0*t-1.0;
    vertice[i] = Vertex{position:[x, y, z]};
  }
  vertices
}

impl Vertex {
  const ATTRIBUTES: [wgpu::VertexAttribuute; 1] = wgpu::vertex_attr_array![0=>Float32x3];
  fn desc<'a> -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {ES,
      array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBUTES,
    }
  }
}

struct State {
  init: trasnforms::InitWgpu,
  pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,
  uniform_buffer: wgpu::Buffer,
  uniform_bind_group: wgpu::BindGroup,
  model_mat: Matrix4<f32>,
  view_mat: Matrix4<f32>,
  project_mat: Matrix4<f32>,
}