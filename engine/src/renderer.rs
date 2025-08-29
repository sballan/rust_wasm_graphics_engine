use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::geometry::{TRIANGLE_VERTICES, CUBE_VERTICES, CUBE_INDICES, CUBE_WIREFRAME_INDICES};
use crate::math::{create_rotation_matrix_2d, create_perspective_matrix, create_3d_transform_matrix, multiply_matrices};

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
        let vertices = TRIANGLE_VERTICES;

        let position_attribute_location = self.context.get_attrib_location(&self.program, "position");
        let buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
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

        if wireframe_mode {
            self.context.draw_arrays(WebGlRenderingContext::LINE_LOOP, 0, 3);
        } else {
            self.context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
        }
    }

    pub fn render_cube(&self, rotation: f32, scale: f32, color: [f32; 3], translation: [f32; 2], 
                       camera_distance: f32, camera_angle_x: f32, camera_angle_y: f32, wireframe_mode: bool) {
        let vertices = CUBE_VERTICES;

        let position_attribute_location = self.context.get_attrib_location(&self.program, "position");
        
        let position_buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        if wireframe_mode {
            let wireframe_indices = CUBE_WIREFRAME_INDICES;
            let index_buffer = self.context.create_buffer().unwrap();
            self.context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
            unsafe {
                let indices_array_buf_view = js_sys::Uint16Array::view(&wireframe_indices);
                self.context.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    &indices_array_buf_view,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }
        } else {
            let solid_indices = CUBE_INDICES;
            let index_buffer = self.context.create_buffer().unwrap();
            self.context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
            unsafe {
                let indices_array_buf_view = js_sys::Uint16Array::view(&solid_indices);
                self.context.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    &indices_array_buf_view,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }
        }

        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
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
        
        let aspect = 1.0;
        let fov = 45.0 * std::f32::consts::PI / 180.0;
        let near = 0.1;
        let far = 100.0;
        
        let perspective = create_perspective_matrix(fov, aspect, near, far);
        
        // Apply transformations: rotation, scale, translation, camera
        let cos_y = rotation.cos();
        let sin_y = rotation.sin();
        let cos_x = camera_angle_x.cos();
        let sin_x = camera_angle_x.sin();
        let cos_cam_y = camera_angle_y.cos();
        let sin_cam_y = camera_angle_y.sin();
        let s = scale * 0.8; // Visible size
        
        // Object rotation (Y-axis)
        let rotation_matrix = [
            cos_y * s, 0.0, sin_y * s, 0.0,
            0.0, s, 0.0, 0.0,
            -sin_y * s, 0.0, cos_y * s, 0.0,
            translation[0], translation[1], -camera_distance, 1.0,
        ];
        
        // Camera rotation around X-axis
        let camera_x_matrix = [
            1.0, 0.0, 0.0, 0.0,
            0.0, cos_x, -sin_x, 0.0,
            0.0, sin_x, cos_x, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        
        // Camera rotation around Y-axis
        let camera_y_matrix = [
            cos_cam_y, 0.0, sin_cam_y, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -sin_cam_y, 0.0, cos_cam_y, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        
        // Combine camera rotations
        let camera_combined = multiply_matrices(&camera_x_matrix, &camera_y_matrix);
        let model_view = multiply_matrices(&rotation_matrix, &camera_combined);
        
        let result = multiply_matrices(&perspective, &model_view);
        
        self.context.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, &result);
        self.context.uniform3fv_with_f32_array(color_location.as_ref(), &color);

        if wireframe_mode {
            self.context.draw_elements_with_i32(
                WebGlRenderingContext::LINES,
                CUBE_WIREFRAME_INDICES.len() as i32,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        } else {
            self.context.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                CUBE_INDICES.len() as i32,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
}