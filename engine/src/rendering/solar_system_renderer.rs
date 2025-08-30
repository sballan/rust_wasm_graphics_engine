use crate::camera::Camera;
use crate::solar_system::SolarSystem;
use crate::shapes::{Sphere, RenderableShape};
use crate::math::create_rotation_matrix_2d;
use crate::renderer::Renderer;

pub struct SolarSystemRenderer;

impl SolarSystemRenderer {
    pub fn render(
        solar_system: &SolarSystem,
        camera: &Camera,
        renderer: &Renderer,
        wireframe_mode: bool,
    ) {
        // Get the center position (origin or followed planet)
        let center_position = if let Some(index) = camera.followed_target {
            if let Some(body) = solar_system.get_body(index) {
                body.get_position()
            } else {
                [0.0, 0.0, 0.0]
            }
        } else {
            [0.0, 0.0, 0.0]
        };
        
        // Render each celestial body
        for body in &solar_system.bodies {
            let position = body.get_position();
            
            // Transform position through camera
            let (screen_pos, z_depth) = camera.transform_point(position, center_position);
            
            // Use z for depth-based scaling (simple perspective)
            let depth_factor = 1.0 / (1.0 + z_depth * 0.1).max(0.1);
            let scale_factor = 1.0 / camera.distance;
            let final_radius = body.radius * scale_factor * depth_factor;
            
            // Create and render sphere
            let sphere = Sphere::new(final_radius, 16, 16);
            let matrix = create_rotation_matrix_2d(0.0, 1.0, screen_pos);
            
            sphere.render(
                &renderer.context,
                &renderer.program,
                [screen_pos[0], screen_pos[1], 0.0],
                body.color,
                &matrix,
                wireframe_mode,
            );
        }
    }
}