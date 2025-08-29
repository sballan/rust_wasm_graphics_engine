use web_sys::WebGlRenderingContext;

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

// Future: This is how we'd implement a proper 3D cube
/*
pub struct Cube {
    vertices: [f32; 24],
    indices: [u16; 36],
    wireframe_indices: [u16; 24],
}

impl Cube {
    pub fn new() -> Self {
        // 8 vertices of a cube
        Self {
            vertices: [
                -0.5, -0.5, -0.5,  // 0: back-bottom-left
                 0.5, -0.5, -0.5,  // 1: back-bottom-right
                 0.5,  0.5, -0.5,  // 2: back-top-right
                -0.5,  0.5, -0.5,  // 3: back-top-left
                -0.5, -0.5,  0.5,  // 4: front-bottom-left
                 0.5, -0.5,  0.5,  // 5: front-bottom-right
                 0.5,  0.5,  0.5,  // 6: front-top-right
                -0.5,  0.5,  0.5,  // 7: front-top-left
            ],
            indices: [/* proper 3D cube face indices */],
            wireframe_indices: [/* 12 edges */],
        }
    }
}

impl Shape for Cube {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_indices(&self) -> Option<&[u16]> {
        Some(&self.indices)
    }

    fn get_vertex_count(&self) -> i32 {
        if let Some(indices) = self.get_indices() {
            indices.len() as i32
        } else {
            8
        }
    }

    fn get_draw_mode(&self, wireframe: bool) -> u32 {
        if wireframe {
            WebGlRenderingContext::LINES
        } else {
            WebGlRenderingContext::TRIANGLES
        }
    }

    fn draw(&self, context: &WebGlRenderingContext, wireframe: bool) {
        let indices = if wireframe {
            &self.wireframe_indices
        } else {
            &self.indices
        };
        
        context.draw_elements_with_i32(
            self.get_draw_mode(wireframe),
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}
*/