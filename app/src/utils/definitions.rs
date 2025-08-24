use winit::{dpi::{PhysicalPosition, PhysicalSize}, error::{ExternalError, NotSupportedError}, window::CursorIcon};

pub trait AppWindow {
    fn set_cursor(&self, cursor: CursorIcon);
    fn get_inner_size(&self) -> PhysicalSize<u32>;
    fn set_window_minimized(&self, minimized: bool);
    fn set_window_maximized(&self, maximized: bool);
    fn is_window_maximized(&self) -> bool;
    fn drag_place_window(&self) -> Result<(), ExternalError>;
    fn outer_window_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError>;
    fn set_outer_window_position(&self, position: PhysicalPosition<i32>);
    fn request_inner_window_size(&self, size: PhysicalSize<u32>) -> Option<PhysicalSize<u32>>;
    fn request_window_redraw(&self);
}

impl AppWindow for winit::window::Window {
    fn set_cursor(&self, cursor: CursorIcon) {
        self.set_cursor(cursor);
    }
    
    fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.inner_size()
    }

    fn set_window_minimized(&self, minimized: bool) {
        self.set_minimized(minimized);
    }

    fn set_window_maximized(&self, maximized: bool) {
        self.set_maximized(maximized);
    }

    fn is_window_maximized(&self) -> bool {
        self.is_maximized()
    }

    fn drag_place_window(&self) -> Result<(), ExternalError> {
        self.drag_window()
    }

    fn outer_window_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        self.outer_position()
    }

    fn set_outer_window_position(&self, position: PhysicalPosition<i32>) {
        self.set_outer_position(position);
    }

    fn request_inner_window_size(&self, size: PhysicalSize<u32>) -> Option<PhysicalSize<u32>> {
        self.request_inner_size(size)
    }

    fn request_window_redraw(&self) {
        self.request_redraw();
    }
}

#[derive(PartialEq, Debug)]
pub enum Edge {
    None, 

    Left,
    Right,
    Bottom,

    BottomLeft,
    BottomRight,
}