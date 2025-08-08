use crate::definitions::Vertex;

pub trait Element {
    fn identify_as(&mut self, id: u32);
    fn draw(&self) -> (Vec<Vertex>, Vec<u16>);
}

pub struct Button {
    id: u32,
}

impl Button {
    pub fn new() -> Self {
        Self {
            id: 0,
        }
    }
}

impl Element for Button {
    fn identify_as(&mut self, id: u32) {
        self.id = id;
    }

    fn draw(&self) -> (Vec<Vertex>, Vec<u16>) {
        let indices = [0, 1, 2, 2, 3, 0].to_vec();
        let vertices = [
            Vertex {
                position: [-100.0, -100.0], // Bottom-left corner of a 200x200 area
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [100.0, -100.0], // Bottom-right corner
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [100.0, 100.0],   // Top-middle corner
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-100.0, 100.0],
                color: [1.0, 0.0, 0.0, 1.0],
            }
        ].to_vec();

        return (vertices, indices);
    }
}