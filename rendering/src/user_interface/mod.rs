use crate::{definitions::{Color, ColorExt}, user_interface::{elements::{Button, Element, Icon, Label, Panel, UiEvent}, interface::Interface}};

pub mod interface;
pub mod elements;

pub struct UserInterface<'a> {
    interface: &'a mut Interface,
}

impl<'a> UserInterface<'a> {
    /// Used for adding a manually constructed element to the [Interface].
    pub fn add_element(&mut self, element: impl Element + 'static, id: Option<u32>) {
        self.interface.add_elements(element, id);
    }

    /// Used to add a panel to the interface, the id
    /// field will usually be None, but can be Some(0)
    /// for allowing special interaction types (Window dragging 
    /// in the case of Some(0)).
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

    /// Used to add a basic button to the interface.
    /// The on_click field is provided a Boxed closure
    /// to be ran on click.
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

    /// Used to add a basic button to the interface.
    /// The on_click field is provided a Boxed closure
    /// that returns a [UiEvent] on click.
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

    /// Used to add a label containing text to 
    /// the [Interface].
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

    /// Used to add an icon, this is effectively
    /// a panel but rather than providing a relative scale
    /// the size of the icon is defined by real pixels in f32 format.
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