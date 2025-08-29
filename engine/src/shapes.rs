use web_sys::WebGlRenderingContext;
use std::f32::consts::PI;

pub trait Shape {
    fn get_vertices(&self) -> &[f32];
    fn get_indices(&self) -> Option<&[u16]> { None }
    fn get_vertex_count(&self) -> i32;
    fn get_draw_mode(&self, wireframe: bool) -> u32;
    fn draw(&self, context: &WebGlRenderingContext, wireframe: bool);
}

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

impl Shape for Triangle {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_vertex_count(&self) -> i32 {
        3
    }

    fn get_draw_mode(&self, wireframe: bool) -> u32 {
        if wireframe {
            WebGlRenderingContext::LINE_LOOP
        } else {
            WebGlRenderingContext::TRIANGLES
        }
    }

    fn draw(&self, context: &WebGlRenderingContext, wireframe: bool) {
        context.draw_arrays(self.get_draw_mode(wireframe), 0, self.get_vertex_count());
    }
}

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

impl Shape for Rectangle {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_vertex_count(&self) -> i32 {
        4
    }

    fn get_draw_mode(&self, wireframe: bool) -> u32 {
        if wireframe {
            WebGlRenderingContext::LINE_LOOP
        } else {
            WebGlRenderingContext::TRIANGLE_FAN
        }
    }

    fn draw(&self, context: &WebGlRenderingContext, wireframe: bool) {
        context.draw_arrays(self.get_draw_mode(wireframe), 0, self.get_vertex_count());
    }
}

pub struct Sphere {
    vertices: Vec<f32>,
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
                // Vertex 1 (lat, lon)
                vertices.push(radius * sin_theta1 * cos_phi1);
                vertices.push(radius * cos_theta1);
                vertices.push(radius * sin_theta1 * sin_phi1);
                
                // Vertex 2 (lat+1, lon)
                vertices.push(radius * sin_theta2 * cos_phi1);
                vertices.push(radius * cos_theta2);
                vertices.push(radius * sin_theta2 * sin_phi1);
                
                // Vertex 3 (lat, lon+1)
                vertices.push(radius * sin_theta1 * cos_phi2);
                vertices.push(radius * cos_theta1);
                vertices.push(radius * sin_theta1 * sin_phi2);
                
                // Second triangle
                // Vertex 1 (lat+1, lon)
                vertices.push(radius * sin_theta2 * cos_phi1);
                vertices.push(radius * cos_theta2);
                vertices.push(radius * sin_theta2 * sin_phi1);
                
                // Vertex 2 (lat+1, lon+1)
                vertices.push(radius * sin_theta2 * cos_phi2);
                vertices.push(radius * cos_theta2);
                vertices.push(radius * sin_theta2 * sin_phi2);
                
                // Vertex 3 (lat, lon+1)
                vertices.push(radius * sin_theta1 * cos_phi2);
                vertices.push(radius * cos_theta1);
                vertices.push(radius * sin_theta1 * sin_phi2);
            }
        }
        
        Self {
            vertices,
        }
    }
}

impl Shape for Sphere {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }
    
    fn get_vertex_count(&self) -> i32 {
        (self.vertices.len() / 3) as i32
    }
    
    fn get_draw_mode(&self, wireframe: bool) -> u32 {
        if wireframe {
            WebGlRenderingContext::LINE_STRIP
        } else {
            WebGlRenderingContext::TRIANGLES
        }
    }
    
    fn draw(&self, context: &WebGlRenderingContext, wireframe: bool) {
        context.draw_arrays(self.get_draw_mode(wireframe), 0, self.get_vertex_count());
    }
}