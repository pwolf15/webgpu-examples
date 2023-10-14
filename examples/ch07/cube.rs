mod common;

fn create_vertices() -> Vec<common::Vertex> {
    let p:[[f32; 3]; 8] = [
        [-1.0,  1.0,  1.0],
        [-1.0,  1.0, -1.0],
        [ 1.0,  1.0, -1.0],
        [ 1.0,  1.0,  1.0],
        [-1.0, -1.0,  1.0],
        [-1.0, -1.0, -1.0],
        [ 1.0, -1.0, -1.0],
        [ 1.0, -1.0,  1.0],
    ];

    let lines: [[f32; 3]; 24] = [
        p[0], p[1], p[1], p[2], p[2], p[3], p[3], p[0],
        p[4], p[5], p[5], p[6], p[6], p[7], p[7], p[4],
        p[0], p[4], p[1], p[5], p[2], p[6], p[3], p[7],
    ];

    let mut data: Vec<common::Vertex> = Vec::with_capacity(lines.len());
    for i in 0..lines.len() {
        data.push(common::vertex(lines[i]));
    }
    data.to_vec()
}

fn main() {
    let title = "cube";
    let mesh_data = create_vertices();
    common::run(&mesh_data, title);
}