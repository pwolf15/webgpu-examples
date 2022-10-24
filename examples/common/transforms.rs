#[allow(dead_code)]
use cgmath::{ Matrix4, Vector3, Rad};

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

fn main() {
  let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);

  let rot_mat_z = Matrix4::from_angle_z(Rad( 20.0 * PI / 180.0));

  let rot_mat = rot_mat_z * Matrix4::from_angle_z(Rad(25.0 * PI / 180.0 ));

  let rot_vec = rot_mat * my_vec;

  println!("\nOriginal vector: \n{:?}", my_vec);
  println!(
    "Total rotation matrix after two rotations: rot_mat: \n{:?}", 
    rot_mat
  );

  println!(
    "Vector after two translations: rot_vec = rot_mat * my_vec = \n{:?}\n", 
    rot_vec
  );
}