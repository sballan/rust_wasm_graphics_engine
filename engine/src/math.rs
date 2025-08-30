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

pub fn create_aspect_corrected_matrix(rotation: f32, scale: f32, translation: [f32; 2], aspect_ratio: f32) -> [f32; 16] {
    let cos = rotation.cos();
    let sin = rotation.sin();
    let s = scale;
    
    // Apply aspect ratio correction to X axis
    // For wide screens (aspect > 1), we scale X down
    let x_scale = if aspect_ratio > 1.0 { s / aspect_ratio } else { s };
    let y_scale = if aspect_ratio < 1.0 { s * aspect_ratio } else { s };
    
    [
        cos * x_scale, sin * x_scale, 0.0, 0.0,
        -sin * y_scale, cos * y_scale, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        translation[0] / aspect_ratio.max(1.0), translation[1] * aspect_ratio.min(1.0), 0.0, 1.0,
    ]
}

