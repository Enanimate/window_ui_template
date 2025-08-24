use std::sync::{Arc, Mutex};
use rendering::{definitions::UiAtlas, user_interface::{elements::{InteractionResult, UiEvent}, interface::Interface}, RenderState};
use winit::{dpi::PhysicalPosition, event::{MouseButton, WindowEvent}, window::CursorIcon};

use crate::utils::{components::header_componenet, definitions::{AppWindow, Edge}};

#[derive(PartialEq)]
pub enum AppState {
    Default,
    Resizing,
}

pub struct AppData {
    pub last_logged_cursor_position: PhysicalPosition<f32>,
    pub curr_hover: Option<u32>,
    pub prev_hover: u32,

    pub state: AppState,

    selected: bool,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            last_logged_cursor_position: PhysicalPosition::new(0.0, 0.0),
            curr_hover: None,
            prev_hover: 0,
            state: AppState::Default,
            selected: false,
        }
    }
}

pub struct AppLogic<W: AppWindow> {
    pub render_state: Option<RenderState>,

    pub window: Option<Arc<W>>,
    pub interface: Arc<Mutex<Interface>>,
    atlas: UiAtlas,

    pub app_data: AppData,
}

impl<W: AppWindow> AppLogic<W> {
    pub fn new(window: Option<Arc<W>>, interface: Arc<Mutex<Interface>>, atlas: UiAtlas) -> Self {
        Self {
            render_state: None,

            window,
            interface,
            atlas,

            app_data: AppData::new(),
        }
    }



    pub fn rebuild_interface(&mut self) {
        let window_size = self.window.as_ref().unwrap().get_inner_size();
        let new_interface_data = Self::build_project_view(self.atlas.clone());

        if let Some(rs) = self.render_state.as_mut() {
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;

            interface_guard.initalize_text_brush(&rs.device, &rs.config, &rs.queue);
            interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [window_size.width, window_size.height]);
        } else {
            log::warn!("Attempted to rebuild interface but render_state was None. Cannot initialize GPU buffers.");
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;
        }
    }

    fn build_project_view(atlas: UiAtlas) -> Interface {
        println!("Building Project-View...");
        let mut interface = Interface::new(atlas);

        // TODO: Implement manual window sizing. Should be straight forward, similar to panel with id: 0.

        interface.show(|ui| {
            header_componenet(ui);
            ui.add_textbox("placeholder", [0.5, 0.5], [0.5, 0.5], "#ffffffff");
        });

        return interface;
    }



    pub fn handle_resizing(&self, cursor_position: [f32; 2], window_size: [f32; 2]) -> Edge {
        // Needs unit tests | is being tested
        let mut resize_event_area = 2.0;
        if self.app_data.state == AppState::Resizing {
            resize_event_area = 50.0;
        }

        let is_on_left_edge = cursor_position[0] <= resize_event_area;
        let is_on_right_edge = cursor_position[0] >= window_size[0] - resize_event_area;
        let is_on_bottom_edge = cursor_position[1] >= window_size[1] - resize_event_area;

        let (cursor_icon, side) = match (is_on_left_edge, is_on_right_edge, is_on_bottom_edge) {
            (true, false, false) => (CursorIcon::WResize, Edge::Left),
            (false, true, false) => (CursorIcon::EResize, Edge::Right),
            (false, false, true) => (CursorIcon::SResize, Edge::Bottom),

            (true, false, true) => (CursorIcon::SwResize, Edge::BottomLeft),
            (false, true, true) => (CursorIcon::SeResize, Edge::BottomRight), 
            _ => (CursorIcon::default(), Edge::None)
        };

        self.window.clone().unwrap().set_cursor(cursor_icon);
        return side;
    }

    pub fn handle_hover(&self, cursor_position: [f32; 2]) -> Option<u32> {
        let window_inner_size = self.window.as_ref().unwrap().get_inner_size();
        let window_size = [window_inner_size.width, window_inner_size.height];

        let mut interface_guard = self.interface.lock().unwrap();
        let mut result = None;
        let mut smallest_element = [0.5, 0.5, 1.0, 1.0];

        for element in &mut interface_guard.elements {
            let element_position = element.get_position(window_size);
            let element_scale = element.get_scale(window_size);

            if element.is_cursor_within_bounds(cursor_position, element_position, element_scale) {
                if element.get_layer(smallest_element, window_size) {
                    smallest_element = [element_position[0], element_position[1], element_scale[0], element_scale[1]];
                    result = Some(element.get_id());
                }
            }
        }
        return result;
    }

    pub fn handle_click(&self, cursor_position: [f32; 2]) -> InteractionResult {
        let window_inner_size = self.window.as_ref().unwrap().get_inner_size();
        let window_size = [window_inner_size.width, window_inner_size.height];

        let mut interface_guard = self.interface.lock().unwrap();
        let mut result = InteractionResult::None;
        let mut smallest_element = [0.5, 0.5, 1.0, 1.0];

        for element in &mut interface_guard.elements {
            let element_position = element.get_position(window_size);
            let element_scale = element.get_scale(window_size);

            if element.is_cursor_within_bounds(cursor_position, element_position, element_scale) {
                if element.get_layer(smallest_element, window_size) {
                    smallest_element = [element_position[0], element_position[1], element_scale[0], element_scale[1]];
                    result = element.handle_click();
                }
            }
        }
        return result;
    }

    pub fn highlight(&self, alpha: f32) -> bool {
        let mut interface_guard = self.interface.lock().unwrap();
        
        for element in &mut interface_guard.elements {
            if element.get_id() == self.app_data.prev_hover {
                return element.set_highlight(alpha);
            }
        }
        false
    }



    pub fn default_state_event_handler(&self, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::WindowEvent) {
        let current_window_size = self.window.as_ref().unwrap().get_inner_size();

        let mut needs_rebuild = false;
        let mut needs_update = false;
        let mut needs_text_update = false;

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                needs_rebuild = true;
                if let Some(rs) = self.render_state.as_mut() {
                    rs.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(rs) = self.render_state.as_mut() {
                    match rs.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            rs.resize(current_window_size.width, current_window_size.height);
                        }
                        Err(e) => {
                            log::error!("Unable to render {}", e);
                        }
                    }
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state.is_pressed() {
                    self.selected_element = None;
                    let window_ref = self.window.clone().unwrap();
                    if self.handle_resizing( self.cursor_position, [current_window_size.width as f32, current_window_size.height as f32]) != Edge::None {
                        self.app_data.state = AppState::Resizing;
                    }
                    match self.handle_click(self.cursor_position) {
                        InteractionResult::Success => (),
                        InteractionResult::Propogate(ui_event) => {
                            match ui_event {
                                UiEvent::CloseRequested => event_loop.exit(),
                                UiEvent::SetMinimized => window_ref.set_minimized(true),
                                UiEvent::ResizeRequested => window_ref.set_maximized(!window_ref.is_maximized()),
                                UiEvent::TitleBar => {let _ = window_ref.drag_window();}
                                UiEvent::SetSelected(id, element_type) => {
                                    self.selected_element = Some((id, element_type))
                                }
                            }
                        },
                        InteractionResult::None => (),
                    }
                } else if button == MouseButton::Left && !state.is_pressed() {
                    if self.logic.app_data.state == AppState::Resizing {
                        self.logic.app_data.state = AppState::Default;
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                let side = self.logic.handle_resizing( self.cursor_position, [current_window_size.width as f32, current_window_size.height as f32]);
                if self.logic.app_data.state == AppState::Resizing {
                    let delta_x = position.x as f32 - self.cursor_position[0];
                    let delta_y = position.y as f32 - self.cursor_position[1];

                    let window = self.window_ref.clone().unwrap();

                    let current_position = window.outer_position().unwrap_or_default();

                    let (mut new_width, mut new_height) = (current_window_size.width as f32, current_window_size.height as f32);
                    let mut new_position_x = current_position.x;

                    println!("{:?}", side);

                    match side {
                        Edge::None => (),

                        Edge::Left => {
                            new_width = (current_window_size.width as f32 - delta_x).max(100.0);
                            new_position_x = (current_position.x as f32 + delta_x) as i32;
                        },

                        Edge::Right => new_width = (current_window_size.width as f32 + delta_x).max(100.0),
                        Edge::Bottom => new_height = (current_window_size.height as f32 + delta_y).max(100.0),
                        Edge::BottomLeft => {
                            new_width = (current_window_size.width as f32 - delta_x).max(100.0);
                            new_height = (current_window_size.height as f32 + delta_y).max(100.0);

                            new_position_x = (current_position.x as f32 + delta_x) as i32;
                        },
                        Edge::BottomRight => {
                            new_width = (current_window_size.width as f32 + delta_x).max(100.0);
                            new_height = (current_window_size.height as f32 + delta_y).max(100.0);
                        },
                    }

                    window.set_outer_position(PhysicalPosition::new(new_position_x, current_position.y));
                    let _ = window.request_inner_size(PhysicalSize::new(new_width as u32, new_height as u32));
                    window.request_redraw();
                }

                
                self.cursor_position = [position.x as f32, position.y as f32];
                self.logic.app_data.curr_hover = self.logic.handle_hover(self.cursor_position);
                if let Some(curr_hover) = self.logic.app_data.curr_hover {
                    if curr_hover != self.logic.app_data.prev_hover {
                        if self.logic.highlight(0.0) {
                            needs_update = true
                        }
                        self.logic.app_data.prev_hover = curr_hover;
                    } else {
                        if self.logic.highlight(1.0) {
                            needs_update = true
                        }
                    }
                } else {
                    if self.logic.highlight(0.0) {
                        needs_update = true
                    }
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    match event.key_without_modifiers() {
                        Key::Named(named_key) => match named_key {
                            NamedKey::Space => {
                                if let Some((selected_id, element_type)) = &self.selected_element {
                                    let mut interface_guard = self.logic.interface.lock().unwrap();
                                    for element in &mut interface_guard.elements {
                                        if element.get_id() == *selected_id && *element_type == ElementType::TextBox{
                                            element.set_text(" ", [self.window_size.width, self.window_size.height]);
                                            needs_text_update = true;
                                        }
                                    }
                                }
                            }
                            NamedKey::Enter => {
                                if let Some((selected_id, element_type)) = &self.selected_element {
                                    let mut interface_guard = self.logic.interface.lock().unwrap();
                                    for element in &mut interface_guard.elements {
                                        if element.get_id() == *selected_id && *element_type == ElementType::TextBox{
                                            element.set_text("\n", [self.window_size.width, self.window_size.height]);
                                            needs_text_update = true;
                                        }
                                    }
                                }
                            }
                            _ => ()
                        }
                        Key::Character(char) => {
                            if let Some((selected_id, element_type)) = &self.selected_element {
                                let mut interface_guard = self.logic.interface.lock().unwrap();
                                for element in &mut interface_guard.elements {
                                    if element.get_id() == *selected_id && *element_type == ElementType::TextBox{
                                        element.set_text(&char, [self.window_size.width, self.window_size.height]);
                                        needs_text_update = true;
                                    }
                                }
                            }
                        },
                        Key::Unidentified(native_key) => println!("native: {:?}", native_key),
                        Key::Dead(_) => println!("DEAD"),
                    }
                }
            }
            _ => ()
        }

        if let Some(window_arc) = self.window_ref.as_ref() {
            window_arc.request_redraw();
        }

        if needs_update {
            if let Some(rs) = &self.logic.render_state {
                let mut interface_guard = self.logic.interface.lock().unwrap();
                interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height]);
            }
        }

        if needs_text_update || self.selected_element.is_some() && self.selected_element.as_ref().unwrap().1 == ElementType::TextBox {
            if let Some(rs) = &self.logic.render_state {
                let mut interface_guard = self.logic.interface.lock().unwrap();
                interface_guard.update_text(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height]);
            }
        }

        if needs_rebuild {
            self.logic.rebuild_interface();
        }
    }

    pub fn resizing_state_event_handler() {

    }
}