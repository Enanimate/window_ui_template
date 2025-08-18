use std::time::{Duration, SystemTime};

use crate::definitions::GeometryType;
pub trait Element {
    /// Returns an elements id
    fn get_id(&self) -> u32;

    /// Returns an elements GeometryType
    /// This is used for instancing similar
    /// geometries together to reduce load on
    /// on the CPU.
    fn get_geometry_type(&self) -> GeometryType;

    /// Returns the elements center-point relative
    /// to the window size. An element at `[0.5, 0.5]`
    /// would have a center point at half the window's 
    /// height and half the window's width.
    fn get_position(&self, window_size: [u32; 2]) -> [f32; 2];

    /// Returns the color the texture mask is
    /// tinted by, for colord textures this
    /// should typically be white with an alpha of 1.0.
    fn get_color(&self) -> [f32; 4];

    /// Returns the scale applied to the element.
    /// An element with a center-point at `[0.5, 0.5]`
    /// and a scale of `[0.5, 0.5]` would have a width of
    /// half the window's width and a height of 
    /// half the window's height.
    fn get_scale(&self, window_size: [u32; 2]) -> [f32; 2];

    /// Returns the name of the texture given to this element,
    /// the name is directly related to the name of the equivalent
    /// file stored in the assets file.
    fn get_texture_name(&self) -> Option<String>;

    /// Returns an option, if called on a label element this
    /// would be the text to be rendered.
    fn get_text(&mut self) -> Option<&String>;

    /// Returns the bounds of an element, used for detecting
    /// whether the user's mouse is within an element.
    fn get_bounds(&self) -> Option<[f32; 2]>;

    /// Returns whether the element this was called on 
    /// has bounds within the input elements bounds.
    fn get_layer(&self, input: [f32; 4], window_size: [u32; 2]) -> bool;

    /// Returns the element type to determine what 
    /// type of element the user is interacting with.
    fn get_element_type(&self) -> ElementType;





    /// Sets an elements id-number.
    fn set_id(&mut self, id: u32);

    /// If an element can be highlighted this will set
    /// the element's alpha transparency value.
    fn set_highlight(&mut self, a_value: f32) -> bool;

    fn set_text(&mut self, text: &str);





    /// Returns a custom result-type, if an element is
    /// non-interactable this should return
    /// InteractionResult::None, if an element is interactable
    /// this can return one of two possible results.
    /// Either Success which means that the interaction was accepted
    /// and successful, or Propogate(UiEvent) which expects the calling function
    /// to handle the returned UiEvent.
    fn handle_click(&self) -> InteractionResult;

    /// Returns whether the cursor's position is within the elements bounds.
    fn is_cursor_within_bounds(&self, cursor_position: [f32; 2], element_pos: [f32; 2], element_scale: [f32;2]) -> bool;
}

pub struct Panel {
    id: u32,
    pub geometry_type: GeometryType,
    relative_position: [f32; 2],
    color: [f32; 4],
    relative_scale: [f32; 2],
    texture_name: String,
}

impl Panel {
    pub fn new(relative_position: [f32; 2], color: [f32; 4], relative_scale: [f32; 2], texture_name: &str) -> Self {
        Self {
            id: 0,
            geometry_type: GeometryType::Quad,
            relative_position,
            color,
            relative_scale,
            texture_name: texture_name.to_string(),
        }
    }
}

impl Element for Panel {
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
        Some(self.texture_name.clone())
    }

    fn get_text(&mut self) -> Option<&String> {
        None
    }

    fn get_bounds(&self) -> Option<[f32; 2]> {
        None
    }

    fn get_layer(&self, input: [f32; 4], window_size: [u32; 2]) -> bool {
        let position_self = [self.relative_position[0] * window_size[0] as f32, self.relative_position[1] * window_size[1] as f32];
        let scale_self = [(self.relative_scale[0] * window_size[0] as f32) / 2.0, (self.relative_scale[1] * window_size[1] as f32) / 2.0];
        let position_input = [input[0] * window_size[0] as f32, input[1] * window_size[1] as f32];
        let scale_input = [(input[2] * window_size[0] as f32) / 2.0, (input[3] * window_size[1] as f32) / 2.0];
        if position_self[0] - scale_self[0] > position_input[0] - scale_input[0]
            || position_self[0] + scale_self[0] < position_input[0] + scale_input[0] 
            || position_self[1] - scale_self[1] > position_input[1] - scale_input[1] 
            || position_self[1] + scale_self[1] < position_input[1] + scale_input[1] {
                return true;
            }
        return false;
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Panel
    }

    fn set_id(&mut self, id: u32) {
        self.id = id
    }

    fn set_highlight(&mut self, _a_value: f32) -> bool {
        false
    }

    fn set_text(&mut self, _text: &str) {
        unimplemented!()
    }

    fn handle_click(&self) -> InteractionResult {
        if self.id == 0 {
            println!("flag1");
            return InteractionResult::Propogate(UiEvent::TitleBar)
        } else {
            println!("flag2");
            InteractionResult::None
        }
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

pub enum UiEvent {
    CloseRequested,
    SetMinimized,
    ResizeRequested,
    TitleBar,
    SetSelected(u32, ElementType),
}
pub enum InteractionResult {
    Success,
    Propogate(UiEvent),
    None
}

#[derive(PartialEq)]
pub enum ElementType {
    Panel,
    Button,
    Label,
    Icon,
    TextBox
}

pub struct Button {
    id: u32,
    pub geometry_type: GeometryType,
    relative_position: [f32; 2],
    color: [f32; 4],
    relative_scale: [f32; 2],
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    on_click_propogate: Option<Box<dyn Fn() -> UiEvent + 'static>>,
    texture_name: String,
}

impl Button {
    pub fn new(relative_position: [f32; 2], color: [f32; 4], relative_scale: [f32; 2], texture_name: &str) -> Self {
        Self {
            id: 0,
            geometry_type: GeometryType::Quad,
            relative_position,
            color,
            relative_scale,
            on_click: None,
            on_click_propogate: None,
            texture_name: texture_name.to_string(),
        }
    }

    pub fn with_prop_fn(mut self, function: impl Fn() -> UiEvent + 'static) -> Self {
        self.on_click_propogate = Some(Box::new(function));
        self
    }

    pub fn with_fn(mut self, function: Box<dyn Fn() + Send + Sync>) -> Self {
        self.on_click = Some(function);
        self
    }
}

impl Element for Button {
    fn get_bounds(&self) -> Option<[f32; 2]> {
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

    fn get_layer(&self, input: [f32; 4], window_size: [u32; 2]) -> bool {
        let position_self = [self.relative_position[0] * window_size[0] as f32, self.relative_position[1] * window_size[1] as f32];
        let scale_self = [(self.relative_scale[0] * window_size[0] as f32) / 2.0, (self.relative_scale[1] * window_size[1] as f32) / 2.0];
        let position_input = [input[0] * window_size[0] as f32, input[1] * window_size[1] as f32];
        let scale_input = [(input[2] * window_size[0] as f32) / 2.0, (input[3] * window_size[1] as f32) / 2.0];
        if position_self[0] - scale_self[0] > position_input[0] - scale_input[0]
            || position_self[0] + scale_self[0] < position_input[0] + scale_input[0] 
            || position_self[1] - scale_self[1] > position_input[1] - scale_input[1] 
            || position_self[1] + scale_self[1] < position_input[1] + scale_input[1] {
                return true;
            }
        return false;
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Button
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_highlight(&mut self, a_value: f32) -> bool {
        self.color[3] = a_value;
        true
    }

    fn set_text(&mut self, _text: &str) {
        unimplemented!()
    }

    fn handle_click(&self) -> InteractionResult {
        if let Some(function) = &self.on_click {
            (function)();
            InteractionResult::Success
        } else if let Some(function) = &self.on_click_propogate {
            let prop = function();
            InteractionResult::Propogate(prop)
        } else {
            InteractionResult::None
        }
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
    
    fn get_text(&mut self) -> Option<&String> {
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
    relative_bounds: Option<[f32; 2]>,
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
        self.relative_bounds = Some([relative_bounds, 30.0]);
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
        let text_length = (self.text.chars().count() as f32 * 15.0) / 2.0;
        let text_height = 30.0 / 2.0;
        [self.relative_position[0] * window_size[0] as f32 - text_length, self.relative_position[1] * window_size[1] as f32 - text_height]
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

    fn get_text(&mut self) -> Option<&String> {
        Some(&self.text)
    }

    fn get_bounds(&self) -> Option<[f32; 2]> {
        self.relative_bounds
    }

    fn get_layer(&self, _input: [f32; 4], _window_size: [u32; 2]) -> bool {
        return false;
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Label
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_highlight(&mut self, _a_value: f32) -> bool {
        false
    }

    fn set_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn handle_click(&self) -> InteractionResult {
        InteractionResult::None
    }

    fn is_cursor_within_bounds(&self, _cursor_position: [f32; 2], _element_pos: [f32; 2], _element_scale: [f32;2]) -> bool {
        false
    }
}

pub struct Icon {
    id: u32,
    pub geometry_type: GeometryType,
    relative_position: [f32; 2],
    color: [f32; 4],
    relative_scale: [f32; 2],
    texture_name: String,
}

impl Icon {
    pub fn new(relative_position: [f32; 2], color: [f32; 4], relative_scale: [f32; 2], texture_name: &str) -> Self {
        Self {
            id: 0,
            geometry_type: GeometryType::Quad,
            relative_position,
            color,
            relative_scale,
            texture_name: texture_name.to_string(),
        }
    }
}

impl Element for Icon {
    fn get_bounds(&self) -> Option<[f32; 2]> {
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

    fn get_scale(&self, _window_size: [u32; 2]) -> [f32; 2] {
        [self.relative_scale[0], self.relative_scale[1]]
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn get_layer(&self, input: [f32; 4], window_size: [u32; 2]) -> bool {
        let position_self = [self.relative_position[0] * window_size[0] as f32, self.relative_position[1] * window_size[1] as f32];
        let scale_self = [(self.relative_scale[0] * window_size[0] as f32) / 2.0, (self.relative_scale[1] * window_size[1] as f32) / 2.0];
        let position_input = [input[0] * window_size[0] as f32, input[1] * window_size[1] as f32];
        let scale_input = [(input[2] * window_size[0] as f32) / 2.0, (input[3] * window_size[1] as f32) / 2.0];
        if position_self[0] - scale_self[0] > position_input[0] - scale_input[0]
            || position_self[0] + scale_self[0] < position_input[0] + scale_input[0] 
            || position_self[1] - scale_self[1] > position_input[1] - scale_input[1] 
            || position_self[1] + scale_self[1] < position_input[1] + scale_input[1] {
                return true;
            }
        return false;
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Icon
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_highlight(&mut self, _a_value: f32) -> bool {
        false
    }

    fn set_text(&mut self, _text: &str) {
        unimplemented!()
    }

    fn handle_click(&self) -> InteractionResult {
        InteractionResult::None
    }

    fn is_cursor_within_bounds(&self, _cursor_position: [f32; 2], _element_pos: [f32; 2], _element_scale: [f32;2]) -> bool {
        false
    }
    
    fn get_texture_name(&self) -> Option<String> {
        Some(self.texture_name.clone())
    }
    
    fn get_text(&mut self) -> Option<&String> {
        None
    }
}

pub struct TextBox {
    id: u32,
    pub geometry_type: GeometryType,
    text: String,
    placeholder: String,
    final_text: String,
    color: [f32; 4],
    relative_position: [f32; 2],
    relative_scale: [f32; 2],
    relative_bounds: Option<[f32; 2]>,
    timer: SystemTime,
    blink_rate: Duration,
    is_cursor_visible: bool,
}

impl TextBox {
    pub fn new(text: &str, relative_position: [f32; 2], relative_scale: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            id: 0,
            geometry_type: GeometryType::Label,
            text: text.to_string(),
            placeholder: "Enter Text...".to_string(),
            final_text: String::new(),
            color,
            relative_position,
            relative_scale,
            relative_bounds: None,
            timer: SystemTime::now(),
            blink_rate: Duration::from_millis(500),
            is_cursor_visible: false,
        }
    }

    pub fn with_bounds(mut self, relative_bounds: f32) -> Self {
        self.relative_bounds = Some([relative_bounds, 30.0]);
        self
    }
}

impl Element for TextBox {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_geometry_type(&self) -> GeometryType {
        self.geometry_type
    }

    fn get_position(&self, window_size: [u32; 2]) -> [f32; 2] {
        let text_length = (self.text.chars().count() as f32 * 15.0) / 2.0;
        let text_height = 30.0 / 2.0;
        [self.relative_position[0] * window_size[0] as f32 - text_length, self.relative_position[1] * window_size[1] as f32 - text_height]
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

    fn get_text(&mut self) -> Option<&String> {
        if self.is_cursor_visible {
            self.final_text = format!("{}{}", self.text.clone(), "|");
        } else {
            self.final_text = self.text.clone();
        }

        match self.timer.elapsed() {
            Ok(elapsed) => {
                if elapsed >= self.blink_rate && !self.text.is_empty() {
                    self.is_cursor_visible = !self.is_cursor_visible;
                    self.timer = SystemTime::now();

                    if self.is_cursor_visible {
                        self.final_text.push_str("|");
                    }
                }
            }

            Err(e) => println!("Timer Error: {e}")
        }

        if self.text.is_empty() {
            return Some(&self.placeholder)
        } else {
            return Some(&self.final_text)
        }
    }

    fn get_bounds(&self) -> Option<[f32; 2]> {
        self.relative_bounds
    }

    fn get_layer(&self, input: [f32; 4], window_size: [u32; 2]) -> bool {
        let position_self = [self.relative_position[0] * window_size[0] as f32, self.relative_position[1] * window_size[1] as f32];
        let scale_self = [(self.relative_scale[0] * window_size[0] as f32) / 2.0, (self.relative_scale[1] * window_size[1] as f32) / 2.0];
        let position_input = [input[0] * window_size[0] as f32, input[1] * window_size[1] as f32];
        let scale_input = [(input[2] * window_size[0] as f32) / 2.0, (input[3] * window_size[1] as f32) / 2.0];
        if position_self[0] - scale_self[0] > position_input[0] - scale_input[0]
            || position_self[0] + scale_self[0] < position_input[0] + scale_input[0] 
            || position_self[1] - scale_self[1] > position_input[1] - scale_input[1] 
            || position_self[1] + scale_self[1] < position_input[1] + scale_input[1] {
                return true;
            }
        return false;
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::TextBox
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_highlight(&mut self, _a_value: f32) -> bool {
        false
    }

    fn set_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn handle_click(&self) -> InteractionResult {
        println!("Label Handle Click");
        InteractionResult::Propogate(UiEvent::SetSelected(self.id, self.get_element_type()))
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