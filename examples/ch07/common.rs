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
#[derive(COpy, Clone, Debug, Pod, Zeroables)]