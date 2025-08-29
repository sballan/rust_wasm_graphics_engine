use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod shaders;
mod math;
mod geometry;
mod renderer;
mod shapes;
mod solar_system;

use shaders::{compile_shader, link_program, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE};
use renderer::Renderer;
use solar_system::SolarSystem;

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
        self.renderer.render_triangle(
            self.rotation,
            self.scale,
            self.color,
            self.translation,
            self.wireframe_mode,
        );
    }

    pub fn render_cube(&self) {
        self.renderer.clear_3d(self.background_color);
        self.renderer.render_cube(
            self.rotation,
            self.scale,
            self.color,
            self.translation,
            self.camera_distance,
            self.camera_angle_x,
            self.camera_angle_y,
            self.wireframe_mode,
        );
    }
    
    pub fn update_solar_system(&mut self, delta_time: f32) {
        self.solar_system.update(delta_time);
    }
    
    pub fn set_time_scale(&mut self, scale: f32) {
        self.solar_system.set_time_scale(scale);
    }
    
    pub fn render_solar_system(&self) {
        self.renderer.clear_3d(self.background_color);
        
        // Simple 3D to 2D projection with camera rotation
        for body in &self.solar_system.bodies {
            let position = body.get_position();
            
            // Apply camera rotation manually
            let x = position[0];
            let y = position[1];
            let z = position[2];
            
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
            
            self.renderer.render_sphere(
                [screen_x, screen_y, 0.0], 
                final_radius, 
                body.color, 
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
}