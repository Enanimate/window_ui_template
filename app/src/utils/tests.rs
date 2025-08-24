#![allow(unused_imports, dead_code)]
use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::interface::Interface};
use winit::{dpi::{PhysicalPosition, PhysicalSize}, window::CursorIcon};

use crate::utils::{core::AppLogic, definitions::{AppWindow, Edge}};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MockWindow {
    pub cursor_icon: Arc<Mutex<CursorIcon>>,
    pub inner_size: PhysicalSize<u32>,
}

impl AppWindow for MockWindow {
    fn set_cursor(&self, cursor: CursorIcon) {
        *self.cursor_icon.lock().unwrap() = cursor;
    }

    fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.inner_size
    }
}

#[test]
fn resizing_check() {
    // Initialize mock data
    let mock_window = MockWindow {
        cursor_icon: Arc::new(Mutex::new(CursorIcon::Default)),
        inner_size: PhysicalSize::new(800, 800)
    };

    let atlas = UiAtlas::new(0, 0);
    let mock_window_ref = Some(Arc::new(mock_window.clone()));
    let mock_interface = Arc::new(Mutex::new(Interface::new(atlas.clone())));
    
    let logic = AppLogic::<MockWindow>::new(mock_window_ref, mock_interface, atlas);

    let window_size = [mock_window.inner_size.width as f32, mock_window.inner_size.height as f32];

    check_resize_edge(&logic, window_size, Edge::Left);
    check_resize_edge(&logic, window_size, Edge::BottomLeft);
    check_resize_edge(&logic, window_size, Edge::Bottom);
    check_resize_edge(&logic, window_size, Edge::BottomRight);
    check_resize_edge(&logic, window_size, Edge::Right);
}

fn check_resize_edge(logic: &AppLogic<MockWindow>, window_size: [f32; 2], edge_to_test: Edge) {
    let cursor_position = match edge_to_test {
        Edge::Left => [0.0, window_size[1] / 2.0],

        Edge::BottomLeft => [0.0, window_size[1]],
        Edge::Bottom => [window_size[0] / 2.0, window_size[1]],
        Edge::BottomRight => [window_size[0], window_size[1]],

        Edge::Right => [window_size[0], window_size[1] / 2.0],

        Edge::None => [window_size[0] / 2.0, window_size[1] / 2.0]
    };
    let result = logic.handle_resizing(cursor_position, window_size);
    assert_eq!(edge_to_test, result, "We failed testing resize_edges, edge: {:#?} | returned: {:#?}", edge_to_test, result);
}