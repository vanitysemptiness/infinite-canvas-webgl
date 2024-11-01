use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct State {
    pub zoom: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub is_dragging: bool,
    pub last_mouse_x: f32,
    pub last_mouse_y: f32,
}

impl State {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            is_dragging: false,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
        }))
    }

    pub fn update_drag(&mut self, canvas_width: f32, canvas_height: f32, new_x: f32, new_y: f32) {
        if self.is_dragging {
            let dx = new_x - self.last_mouse_x;
            let dy = new_y - self.last_mouse_y;
            self.offset_x += dx / canvas_width * 2.0;
            self.offset_y -= dy / canvas_height * 2.0;
            self.last_mouse_x = new_x;
            self.last_mouse_y = new_y;
        }
    }

    pub fn start_drag(&mut self, x: f32, y: f32) {
        self.is_dragging = true;
        self.last_mouse_x = x;
        self.last_mouse_y = y;
    }

    pub fn stop_drag(&mut self) {
        self.is_dragging = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = State::new();
        let state = state.borrow();
        assert_eq!(state.zoom, 1.0);
        assert_eq!(state.offset_x, 0.0);
        assert_eq!(state.offset_y, 0.0);
        assert!(!state.is_dragging);
    }

    #[test]
    fn test_drag_operations() {
        let state = State::new();
        {
            let mut state = state.borrow_mut();
            state.start_drag(10.0, 10.0);
            assert!(state.is_dragging);
            assert_eq!(state.last_mouse_x, 10.0);
            assert_eq!(state.last_mouse_y, 10.0);
        }
        {
            let mut state = state.borrow_mut();
            state.update_drag(100.0, 100.0, 20.0, 20.0);
            assert!(state.offset_x > 0.0);
            assert!(state.offset_y < 0.0);
        }
        {
            let mut state = state.borrow_mut();
            state.stop_drag();
            assert!(!state.is_dragging);
        }
    }
}