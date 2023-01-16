use std:: {iter, mem};
use wgpu::util::DeviceExt;
use cgmath::*;
use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};
use bytemuck:: {Pod, Zeroable, cast_slice};
#[path="../common/transforms.rs"]
mod transforms;
mod camera;
#[path="../common/vertex_data.rs"]
mod vertex_data;

#[repr(C)]
#[derive(Copy,Clone,bytemuck::Pod,bytemuck::Zeroable)]
struct CameraUniform {
  view_mat: [[f32; 4]; 4],
}

impl CameraUniform {
  fn new() -> Self {
    Self {
      view_mat: cgmath::Matrix4::identity().into(),
    }
  }

  fn update_view_project(&mut self, camera: &camera::Camera, project_mat: Matrix4<f32>) {
    self.view_mat = (project_mat * camera.view_mat()).into()
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
  position: [f32; 4],
  color: [f32; 4],
}

fn vertex(p:[i8;3], c:[i8; 3]) -> Vertex {
  Vertex {
    position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
    color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
  }
}

fn create_vertices() -> Vec<Vertex> {
  let (pos, col, _uv, _normal) = vertex_data::cube_data();
  let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
  for i in 0..pos.len() {
    data.push(vertex(pos[i], col[i]));
  }
  data.to_vec()
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

struct State {
  init: transforms::InitWgpu,
  pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,

  camera: camera::Camera,
  projection: Matrix4<f32>,
  camera_controller: camera::CameraController,
  camera_uniform: CameraUniform,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
  mouse_pressed: bool,
}

impl State {

  async fn new(window: &Window) -> Self {
    let init = transforms::InitWgpu::init_wgpu(window).await;

    let shader = init.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
      label: Some("Shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("cube_face_color.wgsl").into()),
    });

    let camera = camera::Camera::new((2.0,3.0,5.0), cgmath::Deg(-80.0), cgmath::Deg(-30.0));
    let projection = transforms::create_projection(
      init.config.width as f32/init.config.height as f32, true);
    let camera_controller = camera::CameraController::new(0.005);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_project(&camera, projection);

    let camera_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Camera Buffer"),
      contents: bytemuck::cast_slice(&[camera_uniform]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_bind_group_layout =
      init.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        }],
        label: Some("Uniform Bind Group Layout",)
      });

    let camera_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor{
      layout: &camera_bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
      label: Some("Uniform Bind Group"),
    });

    let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[&camera_bind_group_layout],
      push_constant_ranges: &[],
    });

    let pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
      label: Some("Render Pipeline"),
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[Vertex::desc()]
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[wgpu::ColorTargetState {
          format: init.config.format,
          blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
          }),
          write_mask: wgpu::ColorWrites::ALL,
        }],
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        cull_mode: Some(wgpu::Face::Back),
        ..Default::default()
      },
      // depth_stencil: None,
      depth_stencil: Some(wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth24Plus,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::LessEqual,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
      }),
      multisample: wgpu::MultisampleState::default()
    });

    let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Buffer"),
      contents: cast_slice(&create_vertices()),
      usage: wgpu::BufferUsages::VERTEX,
    });

    Self {
      init,
      pipeline,
      vertex_buffer,
      camera,
      projection,
      camera_controller,
      camera_buffer,
      camera_bind_group,
      camera_uniform,
      mouse_pressed: false
    }
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.projection = transforms::create_projection(new_size.width as f32/new_size.height as f32, true);
      self.init.size = new_size;
      self.init.config.width = new_size.width;
      self.init.config.height = new_size.height;
      self.init.surface.configure(&self.init.device, &self.init.config);
    }
  }

  #[allow(unused_variables)]
  fn input(&mut self, event: &DeviceEvent) -> bool {
    match event {
      DeviceEvent::Button {
        button: 1,
        state,
      } => {
        self.mouse_pressed = *state == ElementState::Pressed;
        true
      }
      DeviceEvent::MouseMotion { delta } => {
        if self.mouse_pressed {
          self.camera_controller.mouse_move(delta.0, delta.1);
        }
        true
      }
      _ => false,
    }
  }

  fn update(&mut self) {
    self.camera_controller.update_camera(&mut self.camera);
    self.camera_uniform
        .update_view_project(&self.camera, self.projection);
    self.init.queue.write_buffer(
      &self.camera_buffer,
      0,
      bytemuck::cast_slice(&[self.camera_uniform]),
    );
  }

  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.init.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
      size: wgpu::Extent3d {
        width: self.init.config.width,
        height: self.init.config.height,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Depth24Plus,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      label: None,
    });
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self
      .init.device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.2,
              g: 0.247,
              b: 0.314,
              a: 1.0,
            }),
            store: true,
          },
        }],
        // depth_stencil_attachment: None,
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &depth_view,
          depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: false,
          }),
          stencil_ops: None,
        }),
      });

      render_pass.set_pipeline(&self.pipeline);
      render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
      render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
      render_pass.draw(0..36, 0..1);
    }

    self.init.queue.submit(iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}

fn main() {
  env_logger::init();
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  window.set_title(&*format!("{}", "ch06-rotate-cube"));
  let mut state = pollster::block_on(State::new(&window));

  let render_start_time = std::time::Instant::now();

  event_loop.run(move | event, _, control_flow| {
    match event {
      Event::MainEventsCleared => window.request_redraw(),
      Event::DeviceEvent {
        ref event,
        ..
      } => {
        state.input(event);
      }
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == window.id() => {
        // if !state.input(event) {
          match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
              input: 
                KeyboardInput {
                  state: ElementState::Pressed,
                  virtual_keycode: Some(VirtualKeyCode::Escape),
                  ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
              state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
              state.resize(**new_inner_size);
            }
            _ => {}
          }
        //}
      }
      Event::RedrawRequested(_) => {
        state.update();

        match state.render() {
          Ok(_) => {}
          Err(wgpu::SurfaceError::Lost) => state.resize(state.init.size),
          Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
          Err(e) => eprintln!("{:?}", e),
        }
      }
      Event::MainEventsCleared => {
        window.request_redraw();
      }
      _ => {}
    }
  });
}