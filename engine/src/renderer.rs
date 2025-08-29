use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::math::create_rotation_matrix_2d;
use crate::shapes::{Shape, Triangle, Rectangle, Sphere};

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
        self.render_shape(&triangle, rotation, scale, color, translation, wireframe_mode);
    }

    pub fn render_cube(&self, rotation: f32, scale: f32, color: [f32; 3], translation: [f32; 2], 
                       _camera_distance: f32, _camera_angle_x: f32, _camera_angle_y: f32, wireframe_mode: bool) {
        let rectangle = Rectangle::new();
        self.render_shape(&rectangle, rotation, scale, color, translation, wireframe_mode);
    }
    
    pub fn render_sphere(&self, position: [f32; 3], radius: f32, color: [f32; 3], wireframe_mode: bool) {
        let sphere = Sphere::new(radius, 16, 16);
        
        // For now, use 2D translation until we update the matrix math
        let translation_2d = [position[0], position[1]];
        self.render_shape(&sphere, 0.0, 1.0, color, translation_2d, wireframe_mode);
    }
    
    // render_sphere_3d removed - using manual projection in GraphicsEngine instead

    // Generic shape rendering method
    fn render_shape(&self, shape: &dyn Shape, rotation: f32, scale: f32, color: [f32; 3], translation: [f32; 2], wireframe_mode: bool) {
        let vertices = shape.get_vertices();

        let position_attribute_location = self.context.get_attrib_location(&self.program, "position");
        let buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(vertices);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(position_attribute_location as u32);

        let matrix_location = self.context.get_uniform_location(&self.program, "matrix");
        let color_location = self.context.get_uniform_location(&self.program, "uColor");
        
        let matrix = create_rotation_matrix_2d(rotation, scale, translation);
        
        self.context.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, &matrix);
        self.context.uniform3fv_with_f32_array(color_location.as_ref(), &color);

        // Use the shape's own draw method
        shape.draw(&self.context, wireframe_mode);
    }
}