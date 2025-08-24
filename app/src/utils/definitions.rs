use winit::{dpi::PhysicalSize, window::CursorIcon};

pub trait AppWindow {
    fn set_cursor(&self, cursor: CursorIcon);
    fn get_inner_size(&self) -> PhysicalSize<u32>;
    fn set_minimized(&self, minimized: bool);
    fn set_maximized(&self, maximized: bool);
}

impl AppWindow for winit::window::Window {
    fn set_cursor(&self, cursor: CursorIcon) {
        self.set_cursor(cursor);
    }
    
    fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.inner_size()
    }

    fn set_minimized(&self, minimized: bool) {
        self.set_minimized(minimized);
    }

    fn set_maximized(&self, maximized: bool) {
        self.set_maximized(maximized);
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