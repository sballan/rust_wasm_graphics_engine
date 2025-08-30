use web_sys::{WebGlProgram, WebGlRenderingContext};
use super::traits::{RenderableShape, setup_vertex_buffer, set_uniforms};

pub struct Triangle {
    vertices: [f32; 9],
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            vertices: [
                0.0,  0.5, 0.0,
               -0.5, -0.5, 0.0,
                0.5, -0.5, 0.0,
            ],
        }
    }
}

impl RenderableShape for Triangle {
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
            web_sys::console::error_1(&format!("Triangle vertex buffer error: {}", e).into());
            return;
        }
        
        // Set uniforms
        set_uniforms(context, program, matrix, color);
        
        // Draw
        let draw_mode = if wireframe {
            WebGlRenderingContext::LINE_LOOP
        } else {
            WebGlRenderingContext::TRIANGLES
        };
        
        context.draw_arrays(draw_mode, 0, 3);
    }
}