use winit::window::CursorIcon;

pub trait AppWindow {
    fn set_cursor(&self, cursor: CursorIcon);
}

impl AppWindow for winit::window::Window {
    fn set_cursor(&self, cursor: CursorIcon) {
        self.set_cursor(cursor);
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