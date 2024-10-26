use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Request animation frame wrapper for WebGL rendering loop
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

/// Convert a Result to a JsValue for wasm-bindgen compatibility
pub fn to_js_result<T, E: std::fmt::Display>(result: Result<T, E>) -> Result<T, JsValue> {
    result.map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_js_result() {
        let ok_result: Result<i32, String> = Ok(42);
        let err_result: Result<i32, String> = Err("test error".to_string());

        assert!(to_js_result(ok_result).is_ok());
        assert!(to_js_result(err_result).is_err());
    }
}