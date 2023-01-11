use cgmath::*;
use std::f32::consts::PI;

pub struct Camera {
    pub position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<Pt: Into<Point3<f32>>, Yaw: Into<Rad<f32>>, Pitch: Into<Rad<f32>>>(
        position: Pt,
        yaw: Yaw,
        pitch: Pitch,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn view_mat(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position,
            Vector3::new(self.pitch.0.cos()*self.yaw.0.cos(), self.pitch.0.sin(),
                self.pitch.0.cos()*self.yaw.0.sin()).normalize(),
            Vector3::unit_y()
        )
    }
}

pub struct CameraController {
    rotatex: f32,
    rotatey: f32,
    speed: f32,
}

impl CameraController {
    
}