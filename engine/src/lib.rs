use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod shaders;
mod math;
mod renderer;
mod shape_trait;
mod sphere;
mod triangle; 
mod rectangle;
mod solar_system;

use shaders::{compile_shader, link_program, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE};
use renderer::Renderer;
use solar_system::SolarSystem;
use math::create_rotation_matrix_2d;
use triangle::Triangle;
use rectangle::Rectangle;
use sphere::Sphere;
use shape_trait::RenderableShape;

#[wasm_bindgen]
pub struct GraphicsEngine {
    renderer: Renderer,
    rotation: f32,
    scale: f32,
    color: [f32; 3],
    translation: [f32; 2],
    background_color: [f32; 4],
    wireframe_mode: bool,
    camera_distance: f32,
    camera_angle_x: f32,
    camera_angle_y: f32,
    solar_system: SolarSystem,
    followed_planet_index: Option<usize>,
}

#[wasm_bindgen]
impl GraphicsEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<GraphicsEngine, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Failed to cast to canvas"))?;

        let context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            VERTEX_SHADER_SOURCE,
        ).map_err(|e| JsValue::from_str(&e))?;

        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SOURCE,
        ).map_err(|e| JsValue::from_str(&e))?;

        let program = link_program(&context, &vert_shader, &frag_shader).map_err(|e| JsValue::from_str(&e))?;
        context.use_program(Some(&program));

        let renderer = Renderer::new(context, program);

        Ok(GraphicsEngine {
            renderer,
            rotation: 0.0,
            scale: 1.0,
            color: [1.0, 1.0, 1.0],
            translation: [0.0, 0.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            wireframe_mode: false,
            camera_distance: 3.0,
            camera_angle_x: 0.0,
            camera_angle_y: 0.0,
            solar_system: SolarSystem::new(),
            followed_planet_index: None,
        })
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    pub fn set_translation(&mut self, x: f32, y: f32) {
        self.translation = [x, y];
    }

    pub fn set_background_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.background_color = [r, g, b, a];
    }

    pub fn set_wireframe_mode(&mut self, wireframe: bool) {
        self.wireframe_mode = wireframe;
    }

    pub fn set_camera_distance(&mut self, distance: f32) {
        self.camera_distance = distance;
    }

    pub fn set_camera_angles(&mut self, angle_x: f32, angle_y: f32) {
        self.camera_angle_x = angle_x;
        self.camera_angle_y = angle_y;
    }

    pub fn render(&self) {
        self.renderer.clear(self.background_color);
        
        let triangle = Triangle::new();
        let matrix = create_rotation_matrix_2d(self.rotation, self.scale, self.translation);
        triangle.render(
            &self.renderer.context, 
            &self.renderer.program, 
            [0.0, 0.0, 0.0], 
            self.color, 
            &matrix, 
            self.wireframe_mode
        );
    }

    pub fn render_cube(&self) {
        self.renderer.clear_3d(self.background_color);
        
        let rectangle = Rectangle::new();
        let matrix = create_rotation_matrix_2d(self.rotation, self.scale, self.translation);
        rectangle.render(
            &self.renderer.context, 
            &self.renderer.program, 
            [0.0, 0.0, 0.0], 
            self.color, 
            &matrix, 
            self.wireframe_mode
        );
    }
    
    pub fn update_solar_system(&mut self, delta_time: f32) {
        self.solar_system.update(delta_time);
        // Update camera to follow selected planet after physics update
        self.update_camera_for_followed_planet();
    }
    
    pub fn set_time_scale(&mut self, scale: f32) {
        self.solar_system.set_time_scale(scale);
    }
    
    pub fn render_solar_system(&self) {
        self.renderer.clear_3d(self.background_color);
        
        // Get the center position (origin or followed planet)
        let center_position = if let Some(index) = self.followed_planet_index {
            if let Some(body) = self.solar_system.get_body(index) {
                let pos = body.get_position();
                [pos[0], pos[1], pos[2]]
            } else {
                [0.0, 0.0, 0.0]
            }
        } else {
            [0.0, 0.0, 0.0]
        };
        
        // Simple 3D to 2D projection with camera rotation
        for body in &self.solar_system.bodies {
            let position = body.get_position();
            
            // Apply camera translation to center on followed planet
            let x = position[0] - center_position[0];
            let y = position[1] - center_position[1];
            let z = position[2] - center_position[2];
            
            // Rotate around Y axis (horizontal rotation)
            let cos_y = self.camera_angle_y.cos();
            let sin_y = self.camera_angle_y.sin();
            let x_rotated = x * cos_y - z * sin_y;
            let z_rotated = x * sin_y + z * cos_y;
            
            // Rotate around X axis (vertical rotation)
            let cos_x = self.camera_angle_x.cos();
            let sin_x = self.camera_angle_x.sin();
            let y_rotated = y * cos_x - z_rotated * sin_x;
            let z_final = y * sin_x + z_rotated * cos_x;
            
            // Apply camera distance (zoom)
            let scale_factor = 1.0 / self.camera_distance;
            
            // Project to screen coordinates
            let screen_x = x_rotated * scale_factor;
            let screen_y = y_rotated * scale_factor;
            
            // Use z for depth-based scaling (simple perspective)
            let depth_factor = 1.0 / (1.0 + z_final * 0.1).max(0.1);
            let final_radius = body.radius * scale_factor * depth_factor;
            
            let sphere = Sphere::new(final_radius, 16, 16);
            let translation_2d = [screen_x, screen_y];
            let matrix = create_rotation_matrix_2d(0.0, 1.0, translation_2d);
            sphere.render(
                &self.renderer.context, 
                &self.renderer.program, 
                [screen_x, screen_y, 0.0], 
                body.color, 
                &matrix, 
                self.wireframe_mode
            );
        }
    }
    
    pub fn get_planet_count(&self) -> usize {
        self.solar_system.bodies.len()
    }
    
    pub fn get_planet_name(&self, index: usize) -> String {
        self.solar_system.get_body(index)
            .map(|body| body.name.clone())
            .unwrap_or_default()
    }
    
    pub fn set_follow_planet(&mut self, index: i32) {
        if index < 0 {
            self.followed_planet_index = None;
        } else {
            self.followed_planet_index = Some(index as usize);
        }
    }

    pub fn get_follow_planet(&self) -> i32 {
        self.followed_planet_index.map(|i| i as i32).unwrap_or(-1)
    }

    fn update_camera_for_followed_planet(&mut self) {
        // Camera centering is now handled in render_solar_system()
        // This function is kept for future enhancements (e.g., auto-zoom)
    }
}