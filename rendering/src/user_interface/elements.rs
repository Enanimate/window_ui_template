use crate::definitions::GeometryType;

pub trait Element {
    fn get_geometry_type(&self) -> GeometryType;
    fn get_position(&self, window_size: [u32; 2]) -> [f32; 2];
    fn get_color(&self) -> [f32; 4];
    fn get_scale(&self, window_size: [u32; 2]) -> [f32; 2];

    fn handle_click(&self);
    fn is_cursor_within_bounds(&self, cursor_position: [f32; 2], element_pos: [f32; 2], element_scale: [f32;2]) -> bool;
}

pub struct Button {
    pub geometry_type: GeometryType,
    relative_position: [f32; 2],
    color: [f32; 4],
    relative_scale: [f32; 2],
    on_click: Box<dyn Fn() + Send + Sync>,
}

impl Button {
    pub fn new(relative_position: [f32; 2], color: [f32; 4], relative_scale: [f32; 2], on_click: Box<dyn Fn() + Send + Sync>) -> Self {
        Self {
            geometry_type: GeometryType::Quad,
            relative_position,
            color,
            relative_scale,
            on_click,
        }
    }
}

impl Element for Button {
    fn get_geometry_type(&self) -> GeometryType {
        self.geometry_type
    }

    fn get_position(&self, window_size: [u32; 2]) -> [f32; 2] {
        [self.relative_position[0] * window_size[0] as f32, self.relative_position[1] * window_size[1] as f32]
    }

    fn get_scale(&self, window_size: [u32; 2]) -> [f32; 2] {
        [self.relative_scale[0] * window_size[0] as f32, self.relative_scale[1] * window_size[1] as f32]
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn handle_click(&self) {
        (self.on_click)()
    }

    fn is_cursor_within_bounds(&self, cursor_position: [f32; 2], element_pos: [f32; 2], element_scale: [f32;2]) -> bool {
        if cursor_position[0] <= element_pos[0] + (element_scale[0] / 2.0) 
            && cursor_position[0] >= element_pos[0] - (element_scale[0] / 2.0)
            && cursor_position[1] <= element_pos[1] + (element_scale[1] / 2.0) 
            && cursor_position[1] >= element_pos[1] - (element_scale[1] / 2.0) 
        {
            return true;
        }
        false
    }
}