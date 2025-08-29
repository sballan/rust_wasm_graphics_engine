pub fn multiply_matrices(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut result = [0.0; 16];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i * 4 + j] += a[i * 4 + k] * b[k * 4 + j];
            }
        }
    }
    result
}

pub fn create_perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let f = 1.0 / (fov / 2.0).tan();
    let range_inv = 1.0 / (near - far);
    
    [
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (near + far) * range_inv, -1.0,
        0.0, 0.0, near * far * range_inv * 2.0, 0.0,
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

pub fn create_translation_matrix(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        x, y, z, 1.0,
    ]
}

pub fn create_rotation_x_matrix(angle: f32) -> [f32; 16] {
    let cos = angle.cos();
    let sin = angle.sin();
    
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, cos, -sin, 0.0,
        0.0, sin, cos, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn create_rotation_y_matrix(angle: f32) -> [f32; 16] {
    let cos = angle.cos();
    let sin = angle.sin();
    
    [
        cos, 0.0, sin, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin, 0.0, cos, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn create_3d_transform_matrix(
    rotation: f32,
    _scale: f32,
    translation: [f32; 2],
    camera_distance: f32,
    camera_angle_x: f32,
    camera_angle_y: f32,
) -> [f32; 16] {
    let object_rotation = create_rotation_y_matrix(rotation);
    let object_translation = create_translation_matrix(translation[0], translation[1], -camera_distance);
    let camera_rotation_x = create_rotation_x_matrix(camera_angle_x);
    let camera_rotation_y = create_rotation_y_matrix(camera_angle_y);
    
    let combined_object = multiply_matrices(&object_translation, &object_rotation);
    let combined_camera = multiply_matrices(&camera_rotation_x, &camera_rotation_y);
    multiply_matrices(&combined_object, &combined_camera)
}

pub fn create_view_matrix(camera_distance: f32, camera_angle_x: f32, camera_angle_y: f32) -> [f32; 16] {
    // Create camera matrix: translate back, then apply rotations
    let translation = create_translation_matrix(0.0, 0.0, -camera_distance);
    let rotation_x = create_rotation_x_matrix(camera_angle_x);
    let rotation_y = create_rotation_y_matrix(camera_angle_y);
    
    // Apply rotations first, then translation
    let rotation = multiply_matrices(&rotation_y, &rotation_x);
    multiply_matrices(&rotation, &translation)
}

pub fn create_model_matrix(position: [f32; 3], scale: f32) -> [f32; 16] {
    [
        scale, 0.0, 0.0, 0.0,
        0.0, scale, 0.0, 0.0,
        0.0, 0.0, scale, 0.0,
        position[0], position[1], position[2], 1.0,
    ]
}