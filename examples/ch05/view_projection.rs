use std::f32::consts::FRAC_PI_6;
use cgmath::*;

fn main() {
  let eye: Point3<f32> = Point3::new(3.0, 4.0, 5.0);
  let center: Point3<f32> = Point3::new(-3.0, -4.0, -5.0);
  let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

  let view_mat = Matrix4::look_at_rh(eye, center, up);

  println!("\nposition of viewer: {:?}", eye);
  println!("point the viewer is looking at: {:?}", center);
  println!("up direction: {:?}", up);
  println!("view matrix: {:?}\n", view_mat);
}