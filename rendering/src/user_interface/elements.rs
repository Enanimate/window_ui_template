use crate::definitions::GeometryType;

pub trait Element {
    fn get_geometry_type(&self) -> GeometryType;
    fn get_position(&self) -> [f32; 2];
    fn get_color(&self) -> [f32; 4];
    fn get_scale(&self) -> [f32; 2];
}

pub struct Button {
    pub geometry_type: GeometryType,
    position: [f32; 2],
    color: [f32; 4],
    scale: [f32; 2],
}

impl Button {
    pub fn new(position: [f32; 2], color: [f32; 4], scale: [f32; 2]) -> Self {
        Self {
            geometry_type: GeometryType::Quad,
            position,
            color,
            scale,
        }
    }
}

impl Element for Button {
    fn get_geometry_type(&self) -> GeometryType {
        self.geometry_type
    }

    fn get_position(&self) -> [f32; 2] {
        self.position
    }

    fn get_scale(&self) -> [f32; 2] {
        self.scale
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }
}