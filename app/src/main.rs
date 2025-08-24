use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::{elements::{ElementType, InteractionResult, UiEvent}, interface::Interface}, RenderState};
#[allow(unused_imports)]
use winit::{application::ApplicationHandler, dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, MouseButton, WindowEvent}, event_loop::{ControlFlow, EventLoop}, keyboard::{Key, NamedKey}, platform::modifier_supplement::KeyEventExtModifierSupplement, window::{CursorIcon, Window}};

use crate::utils::{atlas_generation::generate_texture_atlas, core::{AppLogic, AppState}, definitions::Edge};

mod utils;

fn main() {
    let atlas = generate_texture_atlas();
    let mut app = App::new(atlas);

    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    //render_state: Option<RenderState>,
    window_ref: Option<Arc<Window>>,
    window_size: PhysicalSize<u32>,
    cursor_position: [f32; 2],
    selected_element: Option<(u32, ElementType)>,
    logic: AppLogic<Window>
}

impl App {
    fn new(atlas: UiAtlas) -> Self {
        Self {
            //render_state: None,
            window_ref: None,
            window_size: PhysicalSize::new(0, 0),
            cursor_position: [0.0, 0.0],
            selected_element: None,
            logic: AppLogic::new(None, Arc::new(Mutex::new(Interface::new(atlas.clone()))), atlas)
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let window_attributes = Window::default_attributes().with_maximized(true).with_decorations(false);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let interface_arc = Arc::clone(&self.logic.interface);
        
        self.window_ref = Some(window.clone());
        self.logic.window = Some(window.clone());
        self.logic.render_state = Some(pollster::block_on(RenderState::new(window.clone(), interface_arc)).unwrap());

        self.logic.rebuild_interface();
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
                println!("RESIZE");
                self.window_size = size;
                needs_rebuild = true;
                if let Some(rs) = self.logic.render_state.as_mut() {
                    rs.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(rs) = self.logic.render_state.as_mut() {
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
                    let window_ref = self.window_ref.clone().unwrap();
                    if self.logic.handle_resizing( self.cursor_position, [current_window_size.width as f32, current_window_size.height as f32]) != Edge::None {
                        self.logic.app_data.state = AppState::Resizing;
                    }
                    match self.logic.handle_click(self.cursor_position) {
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
}