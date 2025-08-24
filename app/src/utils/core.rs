use std::sync::Arc;
use winit::window::CursorIcon;

use crate::utils::definitions::{AppWindow, Edge};

pub struct AppLogic<W: AppWindow> {
    pub window: Option<Arc<W>>,
    resizing: bool,
}

impl<W: AppWindow> AppLogic<W> {
    pub fn new(window: Option<Arc<W>>) -> Self {
        Self {
            window,
            resizing: false,
        }
    }

    pub fn handle_resizing(&self, cursor_position: [f32; 2], window_size: [f32; 2]) -> Edge {
        let mut resize_event_area = 2.0;
        if self.resizing {
            resize_event_area = 50.0;
        }

        let is_on_left_edge = cursor_position[0] <= resize_event_area;
        let is_on_right_edge = cursor_position[0] >= window_size[0] - resize_event_area;
        let is_on_bottom_edge = cursor_position[1] >= window_size[1] - resize_event_area;

        let (cursor_icon, side) = match (is_on_left_edge, is_on_right_edge, is_on_bottom_edge) {
            (true, false, false) => (CursorIcon::WResize, Edge::Left),
            (false, true, false) => (CursorIcon::EResize, Edge::Right),
            (false, false, true) => (CursorIcon::SResize, Edge::Bottom),

            (true, false, true) => (CursorIcon::SwResize, Edge::BottomLeft),
            (false, true, true) => (CursorIcon::SeResize, Edge::BottomRight), 
            _ => (CursorIcon::default(), Edge::None)
        };

        self.window.clone().unwrap().set_cursor(cursor_icon);
        return side;
    }
}