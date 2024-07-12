use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlBuffer, MouseEvent};

struct State {
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
    is_dragging: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = Rc::new(
        canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?,
    );

    let program = Rc::new(setup_program(&context)?);
    let buffer = setup_vertex_buffer(&context)?;

    let state = Rc::new(RefCell::new(State {
        zoom: 1.0,
        offset_x: 0.0,
        offset_y: 0.0,
        is_dragging: false,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
    }));

    // Mouse event handlers
    let state_clone = state.clone();
    let mousedown_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut state = state_clone.borrow_mut();
        state.is_dragging = true;
        state.last_mouse_x = event.client_x() as f32;
        state.last_mouse_y = event.client_y() as f32;
    }) as Box<dyn FnMut(MouseEvent)>);

    let state_clone = state.clone();
    let mouseup_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
        state_clone.borrow_mut().is_dragging = false;
    }) as Box<dyn FnMut(MouseEvent)>);

    let state_clone = state.clone();
    let canvas_clone = canvas.clone();
    let mousemove_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut state = state_clone.borrow_mut();
        if state.is_dragging {
            let dx = event.client_x() as f32 - state.last_mouse_x;
            let dy = event.client_y() as f32 - state.last_mouse_y;
            state.offset_x += dx / canvas_clone.width() as f32 * 2.0;
            state.offset_y -= dy / canvas_clone.height() as f32 * 2.0;
            state.last_mouse_x = event.client_x() as f32;
            state.last_mouse_y = event.client_y() as f32;
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas.add_event_listener_with_callback("mousedown", mousedown_callback.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mouseup", mouseup_callback.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mousemove", mousemove_callback.as_ref().unchecked_ref())?;

    mousedown_callback.forget();
    mouseup_callback.forget();
    mousemove_callback.forget();

    // Resize handler
    fn resize_canvas(canvas: &web_sys::HtmlCanvasElement, context: &WebGl2RenderingContext) {
        let display_width = canvas.client_width() as u32;
        let display_height = canvas.client_height() as u32;

        canvas.set_width(display_width);
        canvas.set_height(display_height);
        context.viewport(0, 0, display_width as i32, display_height as i32);
    }

    // Initial resize
    resize_canvas(&canvas, &context);

    // Resize event listener
    let canvas_clone = canvas.clone();
    let context_clone = context.clone();
    let resize_closure = Closure::wrap(Box::new(move || {
        resize_canvas(&canvas_clone, &context_clone);
    }) as Box<dyn FnMut()>);
    window.add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())?;
    resize_closure.forget();

    // Render function
    let render = Rc::new(RefCell::new(
        move |context: &WebGl2RenderingContext,
              program: &WebGlProgram,
              buffer: &WebGlBuffer,
              state: &State,
              canvas: &web_sys::HtmlCanvasElement| {
            context.clear_color(1.0, 1.0, 1.0, 1.0);
            context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    
            let aspect_ratio = canvas.width() as f32 / canvas.height() as f32;
    
            context.use_program(Some(program));
    
            context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    
            let position_attribute_location = context.get_attrib_location(program, "a_position");
            context.enable_vertex_attrib_array(position_attribute_location as u32);
            context.vertex_attrib_pointer_with_i32(
                position_attribute_location as u32,
                2,
                WebGl2RenderingContext::FLOAT,
                false,
                0,
                0,
            );
    
            if let Some(location) = context.get_uniform_location(program, "u_aspect_ratio") {
                context.uniform1f(Some(&location), aspect_ratio);
            }
            if let Some(location) = context.get_uniform_location(program, "u_zoom") {
                context.uniform1f(Some(&location), state.zoom);
            }
            if let Some(location) = context.get_uniform_location(program, "u_offset") {
                context.uniform2f(Some(&location), state.offset_x, state.offset_y);
            }
    
            context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 51 * 51);
        },
    ));

    // Animation loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let context_clone = context.clone();
    let program_clone = program.clone();
    let buffer_clone = buffer.clone();
    let canvas_clone = canvas.clone();
    let state_clone = state.clone();
    let render_clone = render.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        render_clone.borrow()(
            &context_clone,
            &program_clone,
            &buffer_clone,
            &state_clone.borrow(),
            &canvas_clone,
        );

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

fn setup_program(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
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
        }"##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
        precision mediump float;
        out vec4 outColor;
        void main() {
            outColor = vec4(0.8, 0.8, 0.8, 1.0); // Lighter grey color
        }"##,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;

    Ok(program)
}

fn setup_vertex_buffer(context: &WebGl2RenderingContext) -> Result<WebGlBuffer, JsValue> {
    let grid_size = 51; // Odd number to have a center point
    let mut positions = Vec::with_capacity(grid_size * grid_size * 2);
    for y in 0..grid_size {
        for x in 0..grid_size {
            positions.push((x as f32 / (grid_size - 1) as f32) * 2.0 - 1.0);
            positions.push((y as f32 / (grid_size - 1) as f32) * 2.0 - 1.0);
        }
    }

    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
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

