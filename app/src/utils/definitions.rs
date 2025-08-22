pub enum State {
    Normal,
    Resizing,
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