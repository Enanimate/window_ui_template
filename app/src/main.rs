use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::{elements::{ElementType, InteractionResult, UiEvent}, interface::Interface}, RenderState};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, MouseButton, WindowEvent}, event_loop::{ControlFlow, EventLoop}, keyboard::{Key, NamedKey}, platform::modifier_supplement::KeyEventExtModifierSupplement, window::Window};

use crate::utils::{atlas_generation::generate_texture_atlas, components::header_componenet};

mod utils;

fn main() {
    let atlas = generate_texture_atlas();
    App::new(atlas);
}

struct App {
    render_state: Option<RenderState>,
    window_ref: Option<Arc<Window>>,
    interface: Arc<Mutex<Interface>>,
    window_size: PhysicalSize<u32>,
    cursor_position: [f32; 2],
    selected_element: Option<(u32, ElementType)>,
    hovered: Option<u32>,
    last_hovered: u32,
    atlas: UiAtlas,
    text_buffer: String,
}

impl App {
    fn new(atlas: UiAtlas) {
        let mut app = Self {
            render_state: None,
            window_ref: None,
            interface: Arc::new(Mutex::new(Interface::new(atlas.clone()))),
            window_size: PhysicalSize::new(0, 0),
            cursor_position: [0.0, 0.0],
            selected_element: None,
            hovered: None,
            last_hovered: 0,
            atlas,
            text_buffer: String::new(),
        };

        env_logger::init();
        
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }

    fn handle_click(&self, cursor_position: [f32; 2]) -> InteractionResult {
        let interface_guard = self.interface.lock().unwrap();
        let window_size = [self.window_size.width, self.window_size.height];
        let mut result = InteractionResult::None;
        let mut smallest_element = [0.5, 0.5, 1.0, 1.0];

        for element in &interface_guard.elements {
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

    fn handle_hover(&self, cursor_position: [f32; 2]) -> Option<u32> {
        let interface_guard = self.interface.lock().unwrap();
        let window_size = [self.window_size.width, self.window_size.height];
        let mut result = None;
        let mut smallest_element = [0.5, 0.5, 1.0, 1.0];

        for element in &interface_guard.elements {
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

    fn highlight(&self, alpha: f32) -> bool {
        let mut interface_guard = self.interface.lock().unwrap();
        
        for element in &mut interface_guard.elements {
            if element.get_id() == self.last_hovered {
                return element.set_highlight(alpha);
            }
        }
        false
    }

    fn rebuild_interface(&mut self) {
        let new_interface_data = Self::build_project_view(self.atlas.clone());

        if let Some(rs) = self.render_state.as_mut() {
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;

            interface_guard.initalize_text_brush(&rs.device, &rs.config, &rs.queue);
            interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height]);
        } else {
            log::warn!("Attempted to rebuild interface but render_state was None. Cannot initialize GPU buffers.");
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;
        }
    }

    fn build_project_view(atlas: UiAtlas) -> Interface {
        println!("Building Project-View...");
        let mut interface = Interface::new(atlas);

        interface.show(|ui| {
            header_componenet(ui);
            ui.add_textbox("", [0.5, 0.5], [0.5, 0.5], "#ffffffff");
        });

        return interface;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("Resuming...");
        event_loop.set_control_flow(ControlFlow::Poll);
        let window_attributes = Window::default_attributes().with_maximized(true).with_decorations(false);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let interface_arc = Arc::clone(&self.interface);
        
        self.window_ref = Some(window.clone());
        self.render_state = Some(pollster::block_on(RenderState::new(window, interface_arc)).unwrap());

        self.rebuild_interface();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let current_window_size = self.window_ref.as_ref().unwrap().inner_size();

        let mut needs_rebuild = false;
        let mut needs_update = false;
        let mut needs_text_update = false;
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.window_size = size;
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
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = [position.x as f32, position.y as f32];
                self.hovered = self.handle_hover(self.cursor_position);
                if let Some(hovered) = self.hovered {
                    if hovered != self.last_hovered {
                        if self.highlight(0.0) {
                            needs_update = true
                        }
                        self.last_hovered = hovered;
                    } else {
                        if self.highlight(1.0) {
                            needs_update = true
                        }
                    }
                } else {
                    if self.highlight(0.0) {
                        needs_update = true
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state.is_pressed() {
                    self.selected_element = None;
                    let window_ref = self.window_ref.clone().unwrap();
                    match self.handle_click(self.cursor_position) {
                        InteractionResult::Success => (),
                        InteractionResult::Propogate(ui_event) => {
                            match ui_event {
                                UiEvent::CloseRequested => event_loop.exit(),
                                UiEvent::SetMinimized => window_ref.set_minimized(true),
                                UiEvent::ResizeRequested => window_ref.set_maximized(!window_ref.is_maximized()),
                                UiEvent::TitleBar => {let _ = window_ref.drag_window();}
                                UiEvent::SetSelected(id, element_type) => {
                                    println!("SetSelected: {}", id);
                                    self.selected_element = Some((id, element_type))
                                }
                            }
                        },
                        InteractionResult::None => (),
                    }
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    match event.key_without_modifiers() {
                        Key::Named(named_key) => match named_key {
                            NamedKey::Enter => {
                                println!("{}", self.text_buffer)
                            }
                            _ => ()
                        }
                        Key::Character(char) => {
                            if let Some((selected_id, element_type)) = &self.selected_element {
                                let mut interface_guard = self.interface.lock().unwrap();
                                for element in &mut interface_guard.elements {
                                    if element.get_id() == *selected_id && *element_type == ElementType::TextBox{
                                        element.set_text(&char);
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
            if let Some(rs) = &self.render_state {
                let mut interface_guard = self.interface.lock().unwrap();
                interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height]);
            }
        }

        if needs_text_update || self.selected_element.is_some() && self.selected_element.as_ref().unwrap().1 == ElementType::TextBox {
            if let Some(rs) = &self.render_state {
                let mut interface_guard = self.interface.lock().unwrap();
                interface_guard.update_text(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height]);
            }
        }

        if needs_rebuild {
            self.rebuild_interface();
        }
    }
}