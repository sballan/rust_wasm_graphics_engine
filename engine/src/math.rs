// Removed unused 3D matrix functions - using manual projection instead

pub fn create_view_matrix(camera_pos: [f32; 3], angle_x: f32, angle_y: f32) -> [f32; 16] {
    // Create view matrix from camera position and angles
    let cos_x = angle_x.cos();
    let sin_x = angle_x.sin();
    let cos_y = angle_y.cos();
    let sin_y = angle_y.sin();
    
    // Combined rotation matrix (Y rotation * X rotation)
    [
        cos_y, sin_x * sin_y, -cos_x * sin_y, 0.0,
        0.0, cos_x, sin_x, 0.0,
        sin_y, -sin_x * cos_y, cos_x * cos_y, 0.0,
        -camera_pos[0], -camera_pos[1], -camera_pos[2], 1.0,
    ]
}

pub fn create_perspective_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> [f32; 16] {
    let f = 1.0 / (fov / 2.0).tan();
    let range_inv = 1.0 / (near - far);
    
    [
        f / aspect_ratio, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (near + far) * range_inv, -1.0,
        0.0, 0.0, 2.0 * near * far * range_inv, 0.0,
    ]
}

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

