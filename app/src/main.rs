use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::{elements::{InteractionResult, UiEvent}, interface::Interface}, RenderState};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, MouseButton, WindowEvent}, event_loop::EventLoop, keyboard::Key, platform::modifier_supplement::KeyEventExtModifierSupplement, window::Window};

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
    hovered: Option<u32>,
    last_hovered: u32,
    atlas: UiAtlas,
}

impl App {
    fn new(atlas: UiAtlas) {
        let mut app = Self {
            render_state: None,
            window_ref: None,
            interface: Arc::new(Mutex::new(Interface::new(atlas.clone()))),
            window_size: PhysicalSize::new(0, 0),
            cursor_position: [0.0, 0.0],
            hovered: None,
            last_hovered: 0,
            atlas,
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

    fn highlight(&self, alpha: f32) {
        let mut interface_guard = self.interface.lock().unwrap();
        
        for element in &mut interface_guard.elements {
            if element.get_id() == self.last_hovered {
                element.set_highlight(alpha);
            }
        }
    }

    fn rebuild_interface(&mut self) {
        let new_interface_data = Self::build_project_view(self.atlas.clone());

        if let Some(rs) = self.render_state.as_mut() {
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;

            interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height], &rs.config);
        } else {
            log::warn!("Attempted to rebuild interface but render_state was None. Cannot initialize GPU buffers.");
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;
        }
    }

    fn build_project_view(atlas: UiAtlas) -> Interface {
        let mut interface = Interface::new(atlas);

        interface.show(|ui| {
            header_componenet(ui);
        });

        return interface;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_maximized(true).with_decorations(false);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let interface_arc = Arc::clone(&self.interface);
        
        self.window_ref = Some(window.clone());
        self.render_state = Some(pollster::block_on(RenderState::new(window, interface_arc)).unwrap());

        self.rebuild_interface();

        if let Some(rs) = self.render_state.as_mut() {
            let mut interface_guard = self.interface.lock().unwrap();
            interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height], &rs.config);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let current_window_size = self.window_ref.as_ref().unwrap().inner_size();

        let mut needs_rebuild = false;
        let mut needs_update = true;
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
                        self.highlight(0.0);
                        self.last_hovered = hovered;
                    } else {
                        self.highlight(1.0);
                        needs_update = true;
                    }
                } else {
                    self.highlight(0.0);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state.is_pressed() {
                    let window_ref = self.window_ref.clone().unwrap();
                    match self.handle_click(self.cursor_position) {
                        InteractionResult::Success => (),
                        InteractionResult::Propogate(ui_event) => {
                            match ui_event {
                                UiEvent::CloseRequested => event_loop.exit(),
                                UiEvent::SetMinimized => window_ref.set_minimized(true),
                                UiEvent::ResizeRequested => window_ref.set_maximized(!window_ref.is_maximized()),
                                UiEvent::TitleBar => {
                                    println!("HERE");
                                    let _ = window_ref.drag_window();
                                }
                                //_ => unimplemented!()
                            }
                        },
                        InteractionResult::None => (),
                    }
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    match event.key_without_modifiers() {
                        // TODO: Implement match keys, try to figure out a better layer approach...
                        Key::Named(named_key) => println!("named: {:?}", named_key),
                        Key::Character(char) => match char.parse::<String>().unwrap().as_str() {
                            "f" => {println!("F: {:#?}", self.cursor_position)}
                            _ => ()
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
                interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [self.window_size.width, self.window_size.height], &rs.config);
            }
        }

        if needs_rebuild {
            self.rebuild_interface();
        }
    }
}