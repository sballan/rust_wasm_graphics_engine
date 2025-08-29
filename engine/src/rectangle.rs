use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::shape_trait::{RenderableShape, setup_vertex_buffer, set_uniforms};

pub struct Rectangle {
    vertices: [f32; 12],
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            vertices: [
                -0.5, -0.5, 0.0,  // bottom-left
                 0.5, -0.5, 0.0,  // bottom-right
                 0.5,  0.5, 0.0,  // top-right
                -0.5,  0.5, 0.0,  // top-left
            ],
        }
    }
}

impl RenderableShape for Rectangle {
    fn render(
        &self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
        _position: [f32; 3], // Position handled by matrix
        color: [f32; 3],
        matrix: &[f32; 16],
        wireframe: bool,
    ) {
        // Set up vertex buffer
        if let Err(e) = setup_vertex_buffer(context, program, &self.vertices) {
            web_sys::console::error_1(&format!("Rectangle vertex buffer error: {}", e).into());
            return;
        }
        
        // Set uniforms
        set_uniforms(context, program, matrix, color);
        
        // Draw
        let draw_mode = if wireframe {
            WebGlRenderingContext::LINE_LOOP
        } else {
            WebGlRenderingContext::TRIANGLE_FAN
        };
        
        context.draw_arrays(draw_mode, 0, 4);
    }
}