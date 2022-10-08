use std:: {iter, mem};
use wgpu::util::DeviceExt;
use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};
use bytemuck:: {Pod, Zeroable, cast_slice};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
  position: [f32; 2],
  color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
  Vertex { // Vertex A
    position: [0.0, 0.5],
    color: [1.0, 0.0, 0.0]
  },
  Vertex { // Vertex B
    position: [-0.5, -0.5],
    color: [0.0, 1.0, 0.0]
  },
  Vertex { // Vertex C
    position: [0.5, -0.5],
    color: [0.0, 0.0, 1.0]
  },
];

impl Vertex {
  fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x2,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x3,
        },
      ],
    }
  }
}

struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceCOnfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,
}

impl State {
  async fn new(window: &window) -> Self {
    let size = window.inner_size();
    let instance = wgpu::instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();

  let (device, queue) = adapter
    .request_device(
      &wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
      },
      None, // Trace path
    )
    .await
    .unwrap();

  let config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: surface.get_preferred_format(&adapter).unwrap(),
    width: size.width,
    height: size.height,
    present_mode: wgpu::PresentMode::Fifo,
  };
  surface.configure(&device, &config);

  let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
    label: Some("Shader"),
    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
  });

  let pipeline_layout = 
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });

  let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(&pipeline_layout),
    vertex: wgpu::VertexState {
      module: &shader,
      entry_point: "vs_main",
      buffers: &[Vertex::desc()],
    },
    fragment: Some(wgpu::FragmentState {
      module: &shader,
      entry_point: "fs_main",
      targets: &[wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BLendState {
          color: wgpu::BlendComponent::REPLACE,
          alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
      }],
    }),
    primitive: wgpu::PrimitiveState{
      topology: wgpu::PrimitveTopology::TriangleList,
      strip_index_format: None,
      ..Default::default()
    },
    depth_stencil: None,
    multisample: wgpu::MultiSampleState::default(),
  });

  let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescrptor {
    label: Some("Vertex Buffer"),
    contents: cast_slice(VERTICES),
    usage: wgpu::BufferUsages::VERTEX,
  });

  Self {
    surface,
    device,
    queue,
    config,
    size,
    pipeline,
    vertex_buffer,
  }
}

pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
  if new_size.width > 0 && new_size.height > 0 {
    self.size = new_size;
    self.config.width = new_size.width;
    self.config.height = new_size.height;
    self.surface.configure(&self.device, &self.config);
  }
}

#[allow(unused_variables)]
fn input(&mut self, event: &WindowEvent) -> bool {
  false
}

fn update(&mut self) {}

fn render(&mut self) -> Result<(), wgpu::SurfaceRror> {
  let output = self.surface.get_current_texture()?;
  let view = output
    .texture
    .create_view(&wgpu::TextureViewDescriptor::default());
}