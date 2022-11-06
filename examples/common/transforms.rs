#[allow(dead_code)]
use cgmath::{ Matrix4, Vector3, Rad};
use std::f32::consts::PI;
use cgmath::*;
use winit::window::Window;

pub fn create_transforms(translation:[f32; 3], rotation:[f32; 3], scaling:[f32; 3]) -> Matrix4<f32> {
  let trans_mat = Matrix4::from_translation(Vector3::new(translation[0],
    translation[1], translation[2]));
  let rotate_mat_x = Matrix4::from_angle_x(Rad(rotation[0]));
  let rotate_mat_y = Matrix4::from_angle_x(Rad(rotation[1]));
  let rotate_mat_z = Matrix4::from_angle_x(Rad(rotation[2]));
  let scale_mat = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);

  let model_mat = trans_mat * rotate_mat_z * rotate_mat_y * rotate_mat_x * scale_mat;

  model_mat
}

pub struct InitWgpu {
  pub surface: wgpu::Surface,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
}

impl InitWgpu {
  pub async fn init_wgpu(window: &Window) -> Self {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wpgu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_perference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await()
      .unwrap();

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
        },
        None,
      )
      .await()
      .unwrap();

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_preferred_format(&adapter).unwrap(),
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);

    Self {
      surface,
      device,
      queue,
      config,
      size,
    }
  }
}