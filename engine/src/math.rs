// Removed unused 3D matrix functions - using manual projection instead

pub fn create_rotation_matrix_2d(rotation: f32, scale: f32, translation: [f32; 2]) -> [f32; 16] {
    let cos = rotation.cos();
    let sin = rotation.sin();
    let s = scale;
    
    [
        cos * s, sin * s, 0.0, 0.0,
        -sin * s, cos * s, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        translation[0], translation[1], 0.0, 1.0,
    ]
}

