extern crate vecmath;

pub fn vec2_rotate(vec: vecmath::Vector2<f64>, rotation: f64) -> vecmath::Vector2<f64> {
    let rotation_matrix =  [
        [rotation.cos(), -rotation.sin(), 0.0],
        [rotation.sin(), rotation.cos(), 0.0]
    ];

    vecmath::row_mat2x3_transform_vec2(rotation_matrix, vec)
}

pub fn get_rotation(pos1: vecmath::Vector2<f64>, pos2: vecmath::Vector2<f64>) -> f64 {
    (pos2[1] - pos1[1]).atan2(pos2[0] - pos1[0])
}

pub fn vec2_is_zero(vec:vecmath::Vector2<f64>) -> bool {
    vec[0] == 0.0 && vec[1] == 0.0
}