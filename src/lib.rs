use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

mod events;
mod renderer;
mod shaders;
mod state;
mod utils;

use events::{setup_mouse_events, setup_resize_events};
use renderer::WebGLRenderer;
use state::State;
use utils::request_animation_frame;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Initialize canvas and context
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    // Initialize state and renderer
    let state = State::new();
    let renderer = WebGLRenderer::new(&context)?;

    // Setup events
    setup_mouse_events(&canvas, state.clone())?;
    setup_resize_events(&window, &canvas, &context)?;

    // Initial resize
    renderer.resize_canvas(&canvas, &context);

    // Setup animation loop
    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        renderer.render(&context, &state.borrow());
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}