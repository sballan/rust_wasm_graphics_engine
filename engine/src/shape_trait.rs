use web_sys::{WebGlProgram, WebGlRenderingContext};

/// Common trait for all renderable shapes
pub trait RenderableShape {
    /// Render this shape with the given parameters
    fn render(
        &self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
        position: [f32; 3],
        color: [f32; 3],
        matrix: &[f32; 16],
        wireframe: bool,
    );
}

/// Helper function to set up vertex buffer and attributes
pub fn setup_vertex_buffer(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    vertices: &[f32],
) -> Result<(), String> {
    let position_attribute_location = context.get_attrib_location(program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(vertices);
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

    Ok(())
}

/// Helper function to set uniforms
pub fn set_uniforms(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    matrix: &[f32; 16],
    color: [f32; 3],
) {
    let matrix_location = context.get_uniform_location(program, "matrix");
    let color_location = context.get_uniform_location(program, "uColor");
    
    context.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, matrix);
    context.uniform3fv_with_f32_array(color_location.as_ref(), &color);
}