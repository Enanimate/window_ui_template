use crate::user_interface::{elements::{Button, Element, Label}, interface::Interface};

pub mod interface;
pub mod elements;

pub struct UserInterface<'a> {
    interface: &'a mut Interface,
}

impl<'a> UserInterface<'a> {
    pub fn add_element(&mut self, element: impl Element + 'static) {
        self.interface.add_elements(element);
    }

    pub fn add_button(
        &mut self, 
        relative_position: [f32; 2], 
        color: [f32; 4], 
        relative_scale: [f32; 2], 
        on_click: Box<dyn Fn() + Send + Sync>, texture_name: &str) 
    {
        let element = Button::new(relative_position, color, relative_scale, on_click, texture_name);
        self.interface.add_elements(element);
    }

    pub fn add_label(
        &mut self, 
        text: &str, 
        relative_position: [f32; 2], 
        text_scale: [f32; 2], 
        color: [f32; 4]) 
    {
        let element = Label::new(text, relative_position, text_scale, color);
        self.interface.add_elements(element);
    }
}