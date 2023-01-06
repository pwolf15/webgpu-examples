#![allow(dead_code)]

pub fn cube_data() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[i8; 2]>, Vec<[i8; 3]>) {
    let positions = [
        // front
        [-1, -1, 1],
        [1, -1, 1],
        [-1, 1, 1],
        [-1, 1, 1],
        [1, -1, 1],
        [1, 1, 1],
        // right
        [1, -1, 1],
        [1, -1, -1],
        [1, 1, 1],
        [1, 1, 1],
        [1, -1, -1],
        [1, 1, -1],
        // back
        [1, -1, -1],
        [-1, -1, -1],
        [1, 1, -1],
        [1, 1, -1],
        [-1, -1, -1],
        [-1, 1, -1],
        // left
        [-1, -1, -1],
        [-1, -1, 1],
        [-1, 1, -1],
        [-1, 1, -1],
        [-1, -1, 1],
        [-1, 1, 1],
        // top
        [-1, 1, 1],
        [1, 1, 1],
        [-1, 1, -1],
        [-1, 1, -1],
        [1,1,1],
        [1,1,-1],
        // bottom
        [-1, -1, -1],
        [1, -1, -1],
        [-1, -1, 1],
        [-1, -1, 1],
        [1, -1, -1],
        [1, -1, 1],
    ];

    let colors = [
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
    ];

    let uvs = [
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
    ];

    let normals = [
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
    ];

  (positions.to_vec(), colors.to_vec(), uvs.to_vec(), normals.to_vec())
}

pub fn cube_data_index() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<u16>) {
    let positions = [
        [-1,-1,1],
        [1,-1,1],
        [1,1,1],
        [-1,1,1],
        [-1,-1,-1],
        [1,-1,-1],
        [1,1,-1],
        [-1,1,-1],
    ];

    let colors = [
        [0,0,1],
        [1,0,1],
        [1,1,1],
        [0,1,1],
        [0,0,0],
        [1,0,0],
        [1,1,0],
        [0,1,0],
    ];

    let indices = [
        0,1,2,2,3,0,
        1,5,6,6,2,1,
        4,7,6,6,5,4,
        0,3,7,7,4,0,
        3,2,6,6,7,3,
        0,4,5,5,1,0,
    ];

    (positions.to_vec(), colors.to_vec(), indices.to_vec())
}