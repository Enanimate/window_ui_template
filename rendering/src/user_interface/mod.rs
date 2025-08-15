use crate::{definitions::{Color, ColorExt}, user_interface::{elements::{Button, Element, Icon, Label, Panel, UiEvent}, interface::Interface}};

pub mod interface;
pub mod elements;

pub struct UserInterface<'a> {
    interface: &'a mut Interface,
}

impl<'a> UserInterface<'a> {
    pub fn add_element(&mut self, element: impl Element + 'static, id: Option<u32>) {
        self.interface.add_elements(element, id);
    }

    pub fn add_panel(
        &mut self, 
        relative_position: [f32; 2], 
        color: &str, 
        relative_scale: [f32; 2], 
        texture_name: &str,
        id: Option<u32>
    ) -> &mut Self
    {
        let element = Panel::new(relative_position, Color::from_hex(color).into_vec4(), relative_scale, texture_name);
        self.interface.add_elements(element, id);
        self
    }

    pub fn add_button(
        &mut self, 
        relative_position: [f32; 2], 
        color: &str, 
        relative_scale: [f32; 2], 
        on_click: Box<dyn Fn() + Send + Sync>, 
        texture_name: &str
    )
    {
        let element = Button::new(relative_position, Color::from_hex(color).into_vec4(), relative_scale, texture_name)
            .with_fn(on_click);
        self.interface.add_elements(element, None);
    }

    pub fn add_prop_button(
        &mut self, 
        relative_position: [f32; 2], 
        color: &str, 
        relative_scale: [f32; 2], 
        on_click: impl Fn() -> UiEvent + 'static, 
        texture_name: &str
    )
    {
        let element = Button::new(relative_position, Color::from_hex(color).into_vec4(), relative_scale, texture_name)
            .with_prop_fn(on_click);
        self.interface.add_elements(element, None);
    }

    pub fn add_label(
        &mut self, 
        text: &str, 
        relative_position: [f32; 2], 
        text_scale: [f32; 2], 
        color: &str, 
    ) 
    {
        let element = Label::new(text, relative_position, text_scale, Color::from_hex(color).into_vec4());
        self.interface.add_elements(element, None);
    }

    pub fn add_icon(
        &mut self, 
        relative_position: [f32; 2], 
        color: &str, 
        relative_scale: [f32; 2], 
        texture_name: &str
    )
    {
        let element = Icon::new(relative_position, Color::from_hex(color).into_vec4(), relative_scale, texture_name);
        self.interface.add_elements(element, None);
    }
}