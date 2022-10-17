use cgmath::{Matrix4, Vector4, Vector3};

fn main() {
  let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);

  let my_mat = Matrix4::from_translation(Vector3::new(2.0, 2.5, 3.0));

  let trans_mat = my_mat * Matrix4::from_translation(Vector3::new(-3.0,-2.0,-1.0));

  let trans_vec = trans_mat * my_vec;

  println!("\nOriginal vector: \n{:?}", my_vec);
  println!("Scaling matrix: \n{:?}", my_mat);
  println!(
    "Total translation matrix after two translations: trans mat: \n{:?}", 
    trans_mat
  );

  println!(
    "Vector after two translations: trans_vec = trans_mat * my_vec = \n{:?}\n", 
    trans_vec
  );
}