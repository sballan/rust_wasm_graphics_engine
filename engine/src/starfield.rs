use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

pub struct Starfield {
    stars: Vec<Star>,
    vertex_buffer: Option<WebGlBuffer>,
    num_stars: usize,
}

#[derive(Clone)]
struct Star {
    position: [f32; 3], // x, y, z in 3D space
    brightness: f32,    // 0.0 to 1.0
    size: f32,          // Point size
}

impl Starfield {
    pub fn new(num_stars: usize, radius: f32) -> Self {
        let mut stars = Vec::with_capacity(num_stars);

        // Generate random stars distributed in a sphere around the solar system
        for _ in 0..num_stars {
            // Use spherical coordinates for even distribution
            let theta = js_sys::Math::random() as f32 * 2.0 * std::f32::consts::PI;
            let phi = js_sys::Math::acos(2.0 * js_sys::Math::random() - 1.0) as f32;
            let r = radius * (0.8 + 0.2 * js_sys::Math::random() as f32); // Vary distance slightly

            let x = r * phi.sin() * theta.cos();
            let y = r * phi.sin() * theta.sin();
            let z = r * phi.cos();

            // Random brightness and size for variety - wider range for more dramatic stars
            let random = js_sys::Math::random() as f32;
            let brightness = if random > 0.9 {
                // 10% of stars are very bright
                0.8 + 0.2 * js_sys::Math::random() as f32  // 0.8 to 1.0 brightness
            } else {
                0.3 + 0.4 * js_sys::Math::random() as f32  // 0.3 to 0.7 for most stars
            };
            
            let size = if random > 0.9 {
                // Same 10% of stars are also bigger
                3.0 + 3.0 * js_sys::Math::random() as f32  // 3.0 to 6.0 size for bright stars
            } else {
                1.0 + 2.0 * js_sys::Math::random() as f32  // 1.0 to 3.0 for regular stars
            };

            stars.push(Star {
                position: [x, y, z],
                brightness,
                size,
            });
        }

        Self {
            stars,
            vertex_buffer: None,
            num_stars,
        }
    }

    pub fn init_buffers(&mut self, context: &WebGlRenderingContext) -> Result<(), JsValue> {
        // Create vertex buffer for star positions
        let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Flatten star data: x, y, z, brightness, size for each star
        let mut vertices = Vec::with_capacity(self.stars.len() * 5);
        for star in &self.stars {
            vertices.push(star.position[0]);
            vertices.push(star.position[1]);
            vertices.push(star.position[2]);
            vertices.push(star.brightness);
            vertices.push(star.size);
        }

        unsafe {
            let vertices_array = js_sys::Float32Array::view(&vertices);
            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertices_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.vertex_buffer = Some(buffer);
        Ok(())
    }

    pub fn render(
        &self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
        view_matrix: &[f32; 16],
        projection_matrix: &[f32; 16],
    ) -> Result<(), JsValue> {
        if let Some(buffer) = &self.vertex_buffer {
            // Bind the buffer
            context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(buffer));

            // Get attribute locations
            let position_loc = context.get_attrib_location(program, "a_star_position") as u32;
            let brightness_loc = context.get_attrib_location(program, "a_brightness") as u32;
            let size_loc = context.get_attrib_location(program, "a_size") as u32;

            // Enable and set up attributes
            let stride = 5 * 4; // 5 floats * 4 bytes

            // Position attribute (x, y, z)
            context.vertex_attrib_pointer_with_i32(
                position_loc,
                3,
                WebGlRenderingContext::FLOAT,
                false,
                stride,
                0,
            );
            context.enable_vertex_attrib_array(position_loc);

            // Brightness attribute
            context.vertex_attrib_pointer_with_i32(
                brightness_loc,
                1,
                WebGlRenderingContext::FLOAT,
                false,
                stride,
                3 * 4, // Offset to brightness
            );
            context.enable_vertex_attrib_array(brightness_loc);

            // Size attribute
            context.vertex_attrib_pointer_with_i32(
                size_loc,
                1,
                WebGlRenderingContext::FLOAT,
                false,
                stride,
                4 * 4, // Offset to size
            );
            context.enable_vertex_attrib_array(size_loc);

            // Set uniforms
            let view_loc = context.get_uniform_location(program, "u_view_matrix");
            let proj_loc = context.get_uniform_location(program, "u_projection_matrix");

            context.uniform_matrix4fv_with_f32_array(view_loc.as_ref(), false, view_matrix);
            context.uniform_matrix4fv_with_f32_array(proj_loc.as_ref(), false, projection_matrix);

            // Draw stars as points
            context.draw_arrays(WebGlRenderingContext::POINTS, 0, self.num_stars as i32);

            // Cleanup
            context.disable_vertex_attrib_array(position_loc);
            context.disable_vertex_attrib_array(brightness_loc);
            context.disable_vertex_attrib_array(size_loc);
        }

        Ok(())
    }
}
