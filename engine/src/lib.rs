use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod shaders;
mod math;
mod renderer;
mod shapes;
mod solar_system;
mod camera;
mod rendering;

use shaders::{compile_shader, link_program, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE};
use renderer::Renderer;
use solar_system::SolarSystem;
use math::create_rotation_matrix_2d;
use shapes::{Triangle, Rectangle, RenderableShape};
use camera::Camera;
use rendering::SolarSystemRenderer;

#[wasm_bindgen]
pub struct GraphicsEngine {
    renderer: Renderer,
    rotation: f32,
    scale: f32,
    color: [f32; 3],
    translation: [f32; 2],
    background_color: [f32; 4],
    wireframe_mode: bool,
    camera: Camera,
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
        
        // Set initial viewport
        let width = canvas.width() as i32;
        let height = canvas.height() as i32;
        context.viewport(0, 0, width, height);

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
            camera: Camera::new(),
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
        self.camera.set_distance(distance);
    }

    pub fn set_camera_angles(&mut self, angle_x: f32, angle_y: f32) {
        self.camera.set_angles(angle_x, angle_y);
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
        
        let target_position = if let Some(index) = self.camera.followed_target {
            if let Some(body) = self.solar_system.get_body(index) {
                body.get_position()
            } else {
                [0.0, 0.0, 0.0]
            }
        } else {
            [0.0, 0.0, 0.0]
        };
        
        self.camera.update_transition(delta_time, target_position);
    }
    
    pub fn set_time_scale(&mut self, scale: f32) {
        self.solar_system.set_time_scale(scale);
    }
    
    pub fn render_solar_system(&self) {
        self.renderer.clear_3d(self.background_color);
        SolarSystemRenderer::render(
            &self.solar_system,
            &self.camera,
            &self.renderer,
            self.wireframe_mode,
        );
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
            self.camera.follow_target(None);
        } else {
            self.camera.follow_target(Some(index as usize));
        }
    }

    pub fn get_follow_planet(&self) -> i32 {
        self.camera.followed_target.map(|i| i as i32).unwrap_or(-1)
    }
    
    pub fn resize_canvas(&mut self, width: u32, height: u32) {
        // Update WebGL viewport to match canvas size
        self.renderer.context.viewport(0, 0, width as i32, height as i32);
        // Store aspect ratio for reference (not used for scaling)
        self.camera.set_aspect_ratio(width as f32 / height as f32);
    }
}