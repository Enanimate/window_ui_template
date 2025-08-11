use crate::definitions::GeometryType;

pub trait Element {
    fn get_id(&self) -> u32;
    fn get_geometry_type(&self) -> GeometryType;
    fn get_position(&self, window_size: [u32; 2]) -> [f32; 2];
    fn get_color(&self) -> [f32; 4];
    fn get_scale(&self, window_size: [u32; 2]) -> [f32; 2];
    fn get_texture_name(&self) -> Option<String>;
    fn get_text(&self) -> Option<&str>;
    fn get_bounds(&self) -> Option<f32>;

    fn set_id(&mut self, id: u32);

    fn handle_click(&self);
    fn is_cursor_within_bounds(&self, cursor_position: [f32; 2], element_pos: [f32; 2], element_scale: [f32;2]) -> bool;
}

pub struct Button {
    id: u32,
    pub geometry_type: GeometryType,
    relative_position: [f32; 2],
    color: [f32; 4],
    relative_scale: [f32; 2],
    on_click: Box<dyn Fn() + Send + Sync>,
    texture_name: String,
}

impl Button {
    pub fn new(relative_position: [f32; 2], color: [f32; 4], relative_scale: [f32; 2], on_click: Box<dyn Fn() + Send + Sync>, texture_name: &str) -> Self {
        Self {
            id: 0,
            geometry_type: GeometryType::Quad,
            relative_position,
            color,
            relative_scale,
            on_click,
            texture_name: texture_name.to_string(),
        }
    }
}

impl Element for Button {
    fn get_bounds(&self) -> Option<f32> {
        None
    }

    fn get_id(&self) -> u32 {
        self.id
    }

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

    fn set_id(&mut self, id: u32) {
        self.id = id;
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
    
    fn get_texture_name(&self) -> Option<String> {
        Some(self.texture_name.clone())
    }
    
    fn get_text(&self) -> Option<&str> {
        None
    }
}

pub struct Label {
    id: u32,
    pub geometry_type: GeometryType,
    text: String,
    color: [f32; 4],
    relative_position: [f32; 2],
    relative_scale: [f32; 2],
    relative_bounds: Option<f32>,
}

impl Label {
    pub fn new(text: &str, relative_position: [f32; 2], relative_scale: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            id: 0,
            geometry_type: GeometryType::Label,
            text: text.to_string(),
            color,
            relative_position,
            relative_scale,
            relative_bounds: None,
        }
    }

    pub fn with_bounds(mut self, relative_bounds: f32) -> Self {
        self.relative_bounds = Some(relative_bounds);
        self
    }
}

impl Element for Label {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_geometry_type(&self) -> GeometryType {
        self.geometry_type
    }

    fn get_position(&self, window_size: [u32; 2]) -> [f32; 2] {
        [self.relative_position[0] * window_size[0] as f32, self.relative_position[1] * window_size[1] as f32]
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn get_scale(&self, window_size: [u32; 2]) -> [f32; 2] {
        [self.relative_scale[0] * window_size[0] as f32, self.relative_scale[1] * window_size[1] as f32]
    }

    fn get_texture_name(&self) -> Option<String> {
        None
    }

    fn get_text(&self) -> Option<&str> {
        Some(self.text.as_str())
    }

    fn get_bounds(&self) -> Option<f32> {
        self.relative_bounds
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn handle_click(&self) {
        ()
    }

    fn is_cursor_within_bounds(&self, _cursor_position: [f32; 2], _element_pos: [f32; 2], _element_scale: [f32;2]) -> bool {
        false
    }
}