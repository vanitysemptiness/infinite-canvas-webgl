use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlBuffer, HtmlCanvasElement};
use crate::shaders::ShaderProgram;
use crate::state::State;

pub struct WebGLRenderer {
    program: ShaderProgram,
    buffer: WebGlBuffer,
    grid_size: i32,
}

impl WebGLRenderer {
    pub fn new(context: &WebGl2RenderingContext) -> Result<Self, String> {
        let program = ShaderProgram::new(context)?;
        let buffer = Self::setup_vertex_buffer(context)?;
        
        Ok(Self {
            program,
            buffer,
            grid_size: 51,
        })
    }

    fn setup_vertex_buffer(context: &WebGl2RenderingContext) -> Result<WebGlBuffer, String> {
        let grid_size = 51;
        let mut positions = Vec::with_capacity(grid_size * grid_size * 2);
        
        for y in 0..grid_size {
            for x in 0..grid_size {
                positions.push((x as f32 / (grid_size - 1) as f32) * 2.0 - 1.0);
                positions.push((y as f32 / (grid_size - 1) as f32) * 2.0 - 1.0);
            }
        }

        let buffer = context.create_buffer()
            .ok_or("Failed to create buffer")?;
        
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&positions);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        Ok(buffer)
    }

    pub fn resize_canvas(&self, canvas: &HtmlCanvasElement, context: &WebGl2RenderingContext) {
        let display_width = canvas.client_width() as u32;
        let display_height = canvas.client_height() as u32;

        canvas.set_width(display_width);
        canvas.set_height(display_height);
        context.viewport(0, 0, display_width as i32, display_height as i32);
    }

    pub fn render(&self, context: &WebGl2RenderingContext, state: &State) {
        context.clear_color(1.0, 1.0, 1.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let canvas = context.canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        
        let aspect_ratio = canvas.width() as f32 / canvas.height() as f32;

        context.use_program(Some(&self.program.program));
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));

        let position_loc = context.get_attrib_location(&self.program.program, "a_position");
        context.enable_vertex_attrib_array(position_loc as u32);
        context.vertex_attrib_pointer_with_i32(
            position_loc as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        // Set uniforms
        self.program.set_uniform_1f(context, "u_aspect_ratio", aspect_ratio);
        self.program.set_uniform_1f(context, "u_zoom", state.zoom);
        self.program.set_uniform_2f(context, "u_offset", state.offset_x, state.offset_y);

        context.draw_arrays(WebGl2RenderingContext::POINTS, 0, self.grid_size * self.grid_size);
    }
}