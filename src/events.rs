use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement, MouseEvent, Window};
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::State;

pub fn setup_mouse_events(
    canvas: &HtmlCanvasElement,
    state: Rc<RefCell<State>>,
) -> Result<(), JsValue> {
    let state_clone = state.clone();
    let mousedown_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut state = state_clone.borrow_mut();
        state.start_drag(event.client_x() as f32, event.client_y() as f32);
    }) as Box<dyn FnMut(MouseEvent)>);

    let state_clone = state.clone();
    let mouseup_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
        state_clone.borrow_mut().stop_drag();
    }) as Box<dyn FnMut(MouseEvent)>);

    let state_clone = state;
    let canvas_clone = canvas.clone();
    let mousemove_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut state = state_clone.borrow_mut();
        state.update_drag(
            canvas_clone.width() as f32,
            canvas_clone.height() as f32,
            event.client_x() as f32,
            event.client_y() as f32,
        );
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas.add_event_listener_with_callback(
        "mousedown",
        mousedown_callback.as_ref().unchecked_ref(),
    )?;
    canvas.add_event_listener_with_callback(
        "mouseup",
        mouseup_callback.as_ref().unchecked_ref(),
    )?;
    canvas.add_event_listener_with_callback(
        "mousemove",
        mousemove_callback.as_ref().unchecked_ref(),
    )?;

    // Prevent memory leaks by forgetting the callbacks
    // (they'll be cleaned up when the page is unloaded)
    mousedown_callback.forget();
    mouseup_callback.forget();
    mousemove_callback.forget();

    Ok(())
}

pub fn setup_resize_events(
    window: &Window,
    canvas: &HtmlCanvasElement,
    context: &WebGl2RenderingContext,
) -> Result<(), JsValue> {
    let canvas_clone = canvas.clone();
    let context_clone = context.clone();
    
    let resize_closure = Closure::wrap(Box::new(move || {
        let display_width = canvas_clone.client_width() as u32;
        let display_height = canvas_clone.client_height() as u32;

        canvas_clone.set_width(display_width);
        canvas_clone.set_height(display_height);
        context_clone.viewport(0, 0, display_width as i32, display_height as i32);
    }) as Box<dyn FnMut()>);

    window.add_event_listener_with_callback(
        "resize",
        resize_closure.as_ref().unchecked_ref(),
    )?;

    resize_closure.forget();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Most event-related functionality requires a DOM environment
    // and would typically be tested using wasm-bindgen-test in an actual browser environment
    // Here we just demonstrate the structure for future integration tests
    
    #[test]
    fn test_event_setup_structure() {
        // This is a placeholder to show where integration tests would go
        assert!(true);
    }
}