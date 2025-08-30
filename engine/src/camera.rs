pub struct Camera {
    pub distance: f32,
    pub angle_x: f32,
    pub angle_y: f32,
    pub followed_target: Option<usize>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            distance: 3.0,
            angle_x: 0.0,
            angle_y: 0.0,
            followed_target: None,
        }
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }

    pub fn set_angles(&mut self, angle_x: f32, angle_y: f32) {
        self.angle_x = angle_x;
        self.angle_y = angle_y;
    }

    pub fn follow_target(&mut self, index: Option<usize>) {
        self.followed_target = index;
    }

    pub fn transform_point(&self, point: [f32; 3], center: [f32; 3]) -> ([f32; 2], f32) {
        // Apply camera translation to center on followed object
        let x = point[0] - center[0];
        let y = point[1] - center[1];
        let z = point[2] - center[2];
        
        // Rotate around Y axis (horizontal rotation)
        let cos_y = self.angle_y.cos();
        let sin_y = self.angle_y.sin();
        let x_rotated = x * cos_y - z * sin_y;
        let z_rotated = x * sin_y + z * cos_y;
        
        // Rotate around X axis (vertical rotation)
        let cos_x = self.angle_x.cos();
        let sin_x = self.angle_x.sin();
        let y_rotated = y * cos_x - z_rotated * sin_x;
        let z_final = y * sin_x + z_rotated * cos_x;
        
        // Apply camera distance (zoom)
        let scale_factor = 1.0 / self.distance;
        
        // Project to screen coordinates
        let screen_x = x_rotated * scale_factor;
        let screen_y = y_rotated * scale_factor;
        
        ([screen_x, screen_y], z_final)
    }
}