use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::math::create_rotation_matrix_2d;
use crate::triangle::Triangle;
use crate::rectangle::Rectangle;
use crate::sphere::Sphere;
use crate::shape_trait::RenderableShape;

pub struct Renderer {
    pub context: WebGlRenderingContext,
    pub program: WebGlProgram,
}

impl Renderer {
    pub fn new(context: WebGlRenderingContext, program: WebGlProgram) -> Self {
        Self { context, program }
    }

    pub fn clear(&self, background_color: [f32; 4]) {
        self.context.clear_color(background_color[0], background_color[1], background_color[2], background_color[3]);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn clear_3d(&self, background_color: [f32; 4]) {
        self.context.clear_color(background_color[0], background_color[1], background_color[2], background_color[3]);
        self.context.enable(WebGlRenderingContext::DEPTH_TEST);
        self.context.depth_func(WebGlRenderingContext::LESS);
        self.context.disable(WebGlRenderingContext::CULL_FACE); // Show both front and back faces
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn render_triangle(&self, rotation: f32, scale: f32, color: [f32; 3], translation: [f32; 2], wireframe_mode: bool) {
        let triangle = Triangle::new();
        let matrix = create_rotation_matrix_2d(rotation, scale, translation);
        triangle.render(&self.context, &self.program, [0.0, 0.0, 0.0], color, &matrix, wireframe_mode);
    }

    pub fn render_cube(&self, rotation: f32, scale: f32, color: [f32; 3], translation: [f32; 2], 
                       _camera_distance: f32, _camera_angle_x: f32, _camera_angle_y: f32, wireframe_mode: bool) {
        let rectangle = Rectangle::new();
        let matrix = create_rotation_matrix_2d(rotation, scale, translation);
        rectangle.render(&self.context, &self.program, [0.0, 0.0, 0.0], color, &matrix, wireframe_mode);
    }
    
    pub fn render_sphere(&self, position: [f32; 3], radius: f32, color: [f32; 3], wireframe_mode: bool) {
        let sphere = Sphere::new(radius, 16, 16);
        let translation_2d = [position[0], position[1]];
        let matrix = create_rotation_matrix_2d(0.0, 1.0, translation_2d);
        sphere.render(&self.context, &self.program, position, color, &matrix, wireframe_mode);
    }

    // Old generic render_shape method removed - each shape now handles its own rendering
}