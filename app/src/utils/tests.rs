#![allow(unused_imports, dead_code)]
use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::interface::Interface};
use winit::{dpi::{PhysicalPosition, PhysicalSize}, window::CursorIcon};

use crate::utils::{core::AppLogic, definitions::{AppWindow, Edge}};

#[derive(Debug, Clone)]
struct MockWindowData {
    inner_size: PhysicalSize<u32>,
    outer_position: PhysicalPosition<i32>,

    minimized: bool,
    maximized: bool,
}

impl MockWindowData {
    fn default() -> Self {
        Self { 
            inner_size: PhysicalSize::new(800, 800),
            outer_position: PhysicalPosition::new(0, 0),

            minimized: false, 
            maximized: false 
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MockWindow {
    pub cursor_icon: Arc<Mutex<CursorIcon>>,

    window_data: Arc<Mutex<MockWindowData>>,
}

impl AppWindow for MockWindow {
    fn set_cursor(&self, cursor: CursorIcon) {
        *self.cursor_icon.lock().unwrap() = cursor;
    }

    fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.window_data.lock().unwrap().inner_size
    }

    fn set_window_minimized(&self, minimized: bool) {
        self.window_data.lock().unwrap().minimized = minimized;
    }

    fn set_window_maximized(&self, maximized: bool) {
        self.window_data.lock().unwrap().maximized = maximized;
    }

    fn is_window_maximized(&self) -> bool {
        self.window_data.lock().unwrap().maximized
    }

    fn drag_place_window(&self) -> Result<(), winit::error::ExternalError> {
        Ok(())
    }

    fn outer_window_position(&self) -> Result<PhysicalPosition<i32>, winit::error::NotSupportedError> {
        Ok(self.window_data.lock().unwrap().outer_position)
    }

    fn set_outer_window_position(&self, position: PhysicalPosition<i32>) {
        self.window_data.lock().unwrap().outer_position = position
    }

    fn request_inner_window_size(&self, size: PhysicalSize<u32>) -> Option<PhysicalSize<u32>> {
        let mut guard = self.window_data.lock().unwrap();
        guard.inner_size = size;
        Some(guard.inner_size)
    }

    fn request_window_redraw(&self) {
        ()
    }
}

#[test]
fn resizing_check() {
    // Initialize mock data
    let mock_window = MockWindow {
        cursor_icon: Arc::new(Mutex::new(CursorIcon::Default)),

        window_data: Arc::new(Mutex::new(MockWindowData::default())),
    };

    let atlas = UiAtlas::new(0, 0);
    let mock_window_ref = Some(Arc::new(mock_window.clone()));
    let mock_interface = Arc::new(Mutex::new(Interface::new(atlas.clone())));
    
    let logic = AppLogic::<MockWindow>::new(mock_window_ref, mock_interface, atlas);

    let guard = mock_window.window_data.lock().unwrap();
    let window_size = [guard.inner_size.width as f32, guard.inner_size.height as f32];

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