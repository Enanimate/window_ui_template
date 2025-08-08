use crate::user_interface::{elements::Element, interface::Interface};

pub mod interface;
pub mod elements;

pub struct UserInterface<'a> {
    interface: &'a mut Interface
}

impl<'a> UserInterface<'a> {
    pub fn add_element(&mut self, element: impl Element + 'static) {
        self.interface.add_elements(element);
    }

    pub fn add_button(&mut self, element: impl Element + 'static) {
        self.interface.add_elements(element);
    }
}