use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn compile_shader(
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

pub fn link_program(
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

pub const VERTEX_SHADER_SOURCE: &str = r#"
attribute vec4 position;
uniform mat4 matrix;
varying vec4 vColor;

void main() {
    gl_Position = matrix * position;
    vColor = vec4(position.xyz * 0.5 + 0.5, 1.0);
}
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
precision mediump float;
varying vec4 vColor;
uniform vec3 uColor;

void main() {
    gl_FragColor = vec4(vColor.rgb * uColor, 1.0);
}
"#;

pub const STARFIELD_VERTEX_SHADER: &str = r#"
attribute vec3 a_star_position;
attribute float a_brightness;
attribute float a_size;

uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

varying float v_brightness;

void main() {
    gl_Position = u_projection_matrix * u_view_matrix * vec4(a_star_position, 1.0);
    // Moderate star size that scales with distance
    float distance = length(gl_Position.xyz);
    gl_PointSize = a_size * 250.0 / distance;  // Much bigger size multiplier
    v_brightness = a_brightness;
}
"#;

pub const STARFIELD_FRAGMENT_SHADER: &str = r#"
precision mediump float;

varying float v_brightness;

void main() {
    // Create circular star shape
    vec2 coord = gl_PointCoord - vec2(0.5);
    float dist = length(coord);
    
    // Sharp core with softer glow for bright stars
    float intensity = 1.0 - smoothstep(0.0, 0.5, dist);
    
    // Brighter stars get sharper cores
    if (v_brightness > 0.8) {
        intensity = pow(intensity, 0.4);  // Sharper, brighter core for bright stars
    } else {
        intensity = pow(intensity, 0.8);  // Softer for dim stars
    }
    
    // Brighter star colors - more white for bright stars
    vec3 starColor = mix(
        vec3(0.9, 0.88, 0.85),  // Dim star color
        vec3(1.0, 0.98, 0.95),  // Bright star color
        v_brightness
    );
    
    // Higher alpha for bright stars
    float alpha = intensity * (0.5 + 0.5 * v_brightness);
    
    gl_FragColor = vec4(starColor, alpha);
}
"#;