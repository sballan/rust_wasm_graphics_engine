use web_sys::{WebGlProgram, WebGlRenderingContext};
use std::f32::consts::PI;
use crate::shape_trait::{RenderableShape, setup_vertex_buffer, set_uniforms};

pub struct Sphere {
    vertices: Vec<f32>,
    vertex_count: i32,
}

impl Sphere {
    pub fn new(radius: f32, latitude_segments: u32, longitude_segments: u32) -> Self {
        let mut vertices = Vec::new();
        
        // Generate triangles directly (no indices)
        for lat in 0..latitude_segments {
            let theta1 = lat as f32 * PI / latitude_segments as f32;
            let theta2 = (lat + 1) as f32 * PI / latitude_segments as f32;
            
            let sin_theta1 = theta1.sin();
            let cos_theta1 = theta1.cos();
            let sin_theta2 = theta2.sin();
            let cos_theta2 = theta2.cos();
            
            for lon in 0..longitude_segments {
                let phi1 = lon as f32 * 2.0 * PI / longitude_segments as f32;
                let phi2 = (lon + 1) as f32 * 2.0 * PI / longitude_segments as f32;
                
                let sin_phi1 = phi1.sin();
                let cos_phi1 = phi1.cos();
                let sin_phi2 = phi2.sin();
                let cos_phi2 = phi2.cos();
                
                // First triangle
                vertices.extend_from_slice(&[
                    radius * sin_theta1 * cos_phi1, radius * cos_theta1, radius * sin_theta1 * sin_phi1,
                    radius * sin_theta2 * cos_phi1, radius * cos_theta2, radius * sin_theta2 * sin_phi1,
                    radius * sin_theta1 * cos_phi2, radius * cos_theta1, radius * sin_theta1 * sin_phi2,
                ]);
                
                // Second triangle
                vertices.extend_from_slice(&[
                    radius * sin_theta2 * cos_phi1, radius * cos_theta2, radius * sin_theta2 * sin_phi1,
                    radius * sin_theta2 * cos_phi2, radius * cos_theta2, radius * sin_theta2 * sin_phi2,
                    radius * sin_theta1 * cos_phi2, radius * cos_theta1, radius * sin_theta1 * sin_phi2,
                ]);
            }
        }
        
        let vertex_count = (vertices.len() / 3) as i32;
        
        Self {
            vertices,
            vertex_count,
        }
    }
}

impl RenderableShape for Sphere {
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
            web_sys::console::error_1(&format!("Sphere vertex buffer error: {}", e).into());
            return;
        }
        
        // Set uniforms
        set_uniforms(context, program, matrix, color);
        
        // Draw
        let draw_mode = if wireframe {
            WebGlRenderingContext::LINE_STRIP
        } else {
            WebGlRenderingContext::TRIANGLES
        };
        
        context.draw_arrays(draw_mode, 0, self.vertex_count);
    }
}