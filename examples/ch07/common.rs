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

struct State {
    init: transforms::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
    num_vertices: u32,
}

impl State {
    async fn new(window: &Window, mesh_data: &Vec<Vertex>) -> Self {
        let init = transforms::InitWgpu::init_wgpu(window).await;
        
        let shader = init.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                "/examples/ch06/line3d.wgsl")).into()),
        });

        // uniform data
        let camera_position = (3.0, 1.5, 3.0).into();
        let look_direction = (0.0,0.0,0.0).into();
        let up_direction = cgmath::Vector3::unit_y();
        
        let model_mat = transforms::create_transforms([0.0,0.0,0.0],[0.0,0.0,0.0],[1.0,1.0,1.0]);
        let (view_mat, project_mat, view_project_mat) =
            transforms::create_view_projection(camera_position, look_direction, up_direction,
            init.config.width as f32 / init.config.height as f32, IS_PERSPECTIVE);
        let mvp_mat = view_project_mat * model_mat;

        let mvp_ref:&[f32; 16] = mvp_mat.as_ref();
        let uniform_buffer = init.device.create_buffer_init(&wpgu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mvp_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = 
            init.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                }
                count: None,
            }],
            label: Some("Uniform Bind Group Layout"),
        });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                    format: init.config.format,
                    blend: Some(wgpu::blendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primite: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });
    }
}