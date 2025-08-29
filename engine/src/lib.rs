use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
pub struct GraphicsEngine {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    rotation: f32,
    scale: f32,
    color: [f32; 3],
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

        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec4 position;
            uniform mat4 matrix;
            varying vec4 vColor;
            
            void main() {
                gl_Position = matrix * position;
                vColor = vec4(position.xyz * 0.5 + 0.5, 1.0);
            }
            "#,
        )?;

        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            precision mediump float;
            varying vec4 vColor;
            uniform vec3 uColor;
            
            void main() {
                gl_FragColor = vec4(vColor.rgb * uColor, 1.0);
            }
            "#,
        )?;

        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        Ok(GraphicsEngine {
            context,
            program,
            rotation: 0.0,
            scale: 1.0,
            color: [1.0, 1.0, 1.0],
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

    pub fn render(&self) {
        let context = &self.context;
        
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        let vertices: [f32; 9] = [
            0.0,  0.5, 0.0,
           -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
        ];

        let position_attribute_location = context.get_attrib_location(&self.program, "position");
        let buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        let matrix_location = context.get_uniform_location(&self.program, "matrix");
        let color_location = context.get_uniform_location(&self.program, "uColor");
        
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        let s = self.scale;
        
        let matrix: [f32; 16] = [
            cos * s, sin * s, 0.0, 0.0,
            -sin * s, cos * s, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        
        context.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, &matrix);
        context.uniform3fv_with_f32_array(color_location.as_ref(), &self.color);

        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
    }

    pub fn render_cube(&self) {
        let context = &self.context;
        
        context.clear_color(0.1, 0.1, 0.1, 1.0);
        context.enable(WebGlRenderingContext::DEPTH_TEST);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let vertices: [f32; 72] = [
            // Front face
            -0.5, -0.5,  0.5,
             0.5, -0.5,  0.5,
             0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            // Back face
            -0.5, -0.5, -0.5,
            -0.5,  0.5, -0.5,
             0.5,  0.5, -0.5,
             0.5, -0.5, -0.5,
            // Top face
            -0.5,  0.5, -0.5,
            -0.5,  0.5,  0.5,
             0.5,  0.5,  0.5,
             0.5,  0.5, -0.5,
            // Bottom face
            -0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5, -0.5,  0.5,
            -0.5, -0.5,  0.5,
            // Right face
             0.5, -0.5, -0.5,
             0.5,  0.5, -0.5,
             0.5,  0.5,  0.5,
             0.5, -0.5,  0.5,
            // Left face
            -0.5, -0.5, -0.5,
            -0.5, -0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
        ];

        let indices: [u16; 36] = [
            0,  1,  2,      0,  2,  3,    // front
            4,  5,  6,      4,  6,  7,    // back
            8,  9,  10,     8,  10, 11,   // top
            12, 13, 14,     12, 14, 15,   // bottom
            16, 17, 18,     16, 18, 19,   // right
            20, 21, 22,     20, 22, 23,   // left
        ];

        let position_attribute_location = context.get_attrib_location(&self.program, "position");
        
        let position_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let index_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        unsafe {
            let indices_array_buf_view = js_sys::Uint16Array::view(&indices);
            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        let matrix_location = context.get_uniform_location(&self.program, "matrix");
        let color_location = context.get_uniform_location(&self.program, "uColor");
        
        let aspect = 1.0;
        let fov = 45.0 * std::f32::consts::PI / 180.0;
        let near = 0.1;
        let far = 100.0;
        
        let f = 1.0 / (fov / 2.0).tan();
        let range_inv = 1.0 / (near - far);
        
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        let s = self.scale * 0.5;
        
        let perspective: [f32; 16] = [
            f / aspect * s, 0.0, 0.0, 0.0,
            0.0, f * s, 0.0, 0.0,
            0.0, 0.0, (near + far) * range_inv, -1.0,
            0.0, 0.0, near * far * range_inv * 2.0, 0.0,
        ];
        
        let rotation_y: [f32; 16] = [
            cos, 0.0, sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -sin, 0.0, cos, 0.0,
            0.0, 0.0, -3.0, 1.0,
        ];
        
        let mut result = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i * 4 + j] += perspective[i * 4 + k] * rotation_y[k * 4 + j];
                }
            }
        }
        
        context.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, &result);
        context.uniform3fv_with_f32_array(color_location.as_ref(), &self.color);

        context.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}