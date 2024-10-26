use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

const VERTEX_SHADER: &str = r##"#version 300 es
layout(location = 0) in vec2 a_position;
uniform float u_aspect_ratio;
uniform float u_zoom;
uniform vec2 u_offset;
void main() {
    vec2 zoomedPosition = a_position * u_zoom;
    vec2 offsetPosition = zoomedPosition + u_offset;
    
    // Wrap the position to create an infinite grid effect
    offsetPosition = fract(offsetPosition + 0.5) - 0.5;
    
    // Adjust for aspect ratio while maintaining square grid
    float adjustedX = offsetPosition.x * min(1.0, 1.0 / u_aspect_ratio);
    float adjustedY = offsetPosition.y * min(1.0, u_aspect_ratio);
    
    gl_Position = vec4(adjustedX, adjustedY, 0.0, 1.0);
    gl_PointSize = 2.0;
}"##;

const FRAGMENT_SHADER: &str = r##"#version 300 es
precision mediump float;
out vec4 outColor;
void main() {
    outColor = vec4(0.8, 0.8, 0.8, 1.0);
}"##;

pub struct ShaderProgram {
    pub program: WebGlProgram,
}

impl ShaderProgram {
    pub fn new(context: &WebGl2RenderingContext) -> Result<Self, String> {
        let vert_shader = compile_shader(context, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER)?;
        let frag_shader = compile_shader(context, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER)?;
        let program = link_program(context, &vert_shader, &frag_shader)?;
        
        Ok(Self { program })
    }

    pub fn set_uniform_1f(&self, context: &WebGl2RenderingContext, name: &str, value: f32) {
        if let Some(location) = context.get_uniform_location(&self.program, name) {
            context.uniform1f(Some(&location), value);
        }
    }

    pub fn set_uniform_2f(&self, context: &WebGl2RenderingContext, name: &str, value1: f32, value2: f32) {
        if let Some(location) = context.get_uniform_location(&self.program, name) {
            context.uniform2f(Some(&location), value1, value2);
        }
    }
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
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
    context: &WebGl2RenderingContext,
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
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
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