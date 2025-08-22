use std::any;

use rendering::user_interface::elements::{ElementType, InteractionResult, UiEvent};
use winit::{dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, MouseButton, WindowEvent}, keyboard::{Key, NamedKey}, platform::modifier_supplement::KeyEventExtModifierSupplement};

use crate::{utils::definitions::Edge, App};


pub fn state_normal(app: &mut App, event: &winit::event::WindowEvent, event_loop: &winit::event_loop::ActiveEventLoop) {

    let mut needs_rebuild = false;
    let mut needs_update = false;
    let mut needs_text_update = false;
    let window = app.window_ref.clone().unwrap();
    let current_window_size = window.inner_size();
    match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                app.window_size = *size;
                needs_rebuild = true;
                if let Some(rs) = app.render_state.as_mut() {
                    rs.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(rs) = app.render_state.as_mut() {
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
                if button == &MouseButton::Left && state.is_pressed() {
                    app.selected_element = None;
                    let window_ref = app.window_ref.clone().unwrap();
                    if app.handle_resizing( app.cursor_position, [current_window_size.width as f32, current_window_size.height as f32]) != Edge::None {
                        app.resizing = true;
                    }
                    match app.handle_click(app.cursor_position) {
                        InteractionResult::Success => (),
                        InteractionResult::Propogate(ui_event) => {
                            match ui_event {
                                UiEvent::CloseRequested => event_loop.exit(),
                                UiEvent::SetMinimized => window_ref.set_minimized(true),
                                UiEvent::ResizeRequested => window_ref.set_maximized(!window_ref.is_maximized()),
                                UiEvent::TitleBar => {let _ = window_ref.drag_window();}
                                UiEvent::SetSelected(id, element_type) => {
                                    app.selected_element = Some((id, element_type))
                                }
                            }
                        },
                        InteractionResult::None => (),
                    }
                } else if button == &MouseButton::Left && !state.is_pressed() {
                    if app.resizing {
                        app.resizing = false;
                    }
                }
            }
            
            WindowEvent::CursorMoved { position, .. } => {
                app.cursor_position = [position.x as f32, position.y as f32];
                app.hovered = app.handle_hover(app.cursor_position);
                if let Some(hovered) = app.hovered {
                    if hovered != app.last_hovered {
                        if app.highlight(0.0) {
                            needs_update = true
                        }
                        app.last_hovered = hovered;
                    } else {
                        if app.highlight(1.0) {
                            needs_update = true
                        }
                    }
                } else {
                    if app.highlight(0.0) {
                        needs_update = true
                    }
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    match event.key_without_modifiers() {
                        Key::Named(named_key) => match named_key {
                            NamedKey::Space => {
                                if let Some((selected_id, element_type)) = &app.selected_element {
                                    let mut interface_guard = app.interface.lock().unwrap();
                                    for element in &mut interface_guard.elements {
                                        if *element_type == ElementType::TextBox(*selected_id) {
                                        //if element.get_id() == *selected_id && *element_type == ElementType::TextBox{
                                            element.set_text(" ", [app.window_size.width, app.window_size.height]);
                                            needs_text_update = true;
                                        }
                                    }
                                }
                            }
                            NamedKey::Enter => {
                                if let Some((selected_id, element_type)) = &app.selected_element {
                                    let mut interface_guard = app.interface.lock().unwrap();
                                    for element in &mut interface_guard.elements {
                                        if *element_type == ElementType::TextBox(*selected_id) {
                                            element.set_text("\n", [app.window_size.width, app.window_size.height]);
                                            needs_text_update = true;
                                        }
                                    }
                                }
                            }
                            _ => ()
                        }
                        Key::Character(char) => {
                            if let Some((selected_id, element_type)) = &app.selected_element {
                                let mut interface_guard = app.interface.lock().unwrap();
                                for element in &mut interface_guard.elements {
                                    if *element_type == ElementType::TextBox(*selected_id) {
                                        element.set_text(&char, [app.window_size.width, app.window_size.height]);
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

    if needs_update {
        if let Some(rs) = &app.render_state {
            let mut interface_guard = app.interface.lock().unwrap();
            interface_guard.initialize_interface_buffers(&rs.device, &rs.queue, [app.window_size.width, app.window_size.height]);
        }
    }

    let selected_data = app.selected_element.as_ref();

    if needs_text_update || app.selected_element.is_some() && selected_data.clone().unwrap().1 == ElementType::TextBox(selected_data.unwrap().0) {
        if let Some(rs) = &app.render_state {
            let mut interface_guard = app.interface.lock().unwrap();
            interface_guard.update_text(&rs.device, &rs.queue, [app.window_size.width, app.window_size.height]);
        }
    }

    if needs_rebuild {
        app.rebuild_interface();
    }
}
// state_resizing is the limited-state entered when manually resizing the window.
// This state should heavily limit other possible types of interaction.
pub fn state_resizing(app: &mut App, event: &winit::event::WindowEvent) {

    match event {
        WindowEvent::CursorMoved { position, .. } => {
            let window = app.window_ref.clone().unwrap();

            let current_window_size = window.inner_size();
            let current_window_position = window.outer_position().expect("Failed to get window position.");

            let side = app.handle_resizing( app.cursor_position, [current_window_size.width as f32, current_window_size.height as f32]);
            let delta_x = position.x as f32 - app.cursor_position[0];
            let delta_y = position.y as f32 - app.cursor_position[1];

            let (mut new_width, mut new_height) = (current_window_size.width as f32, current_window_size.height as f32);
            let mut new_position_x = current_window_position.x;

            println!("{:?}", side);

            match side {
                Edge::None => (),

                Edge::Left => {
                    new_width = (current_window_size.width as f32 - delta_x).max(100.0);
                    new_position_x = (current_window_position.x as f32 + delta_x) as i32;
                },

                Edge::Right => new_width = (current_window_size.width as f32 + delta_x).max(100.0),
                Edge::Bottom => new_height = (current_window_size.height as f32 + delta_y).max(100.0),
                Edge::BottomLeft => {
                    new_width = (current_window_size.width as f32 - delta_x).max(100.0);
                    new_height = (current_window_size.height as f32 + delta_y).max(100.0);

                    new_position_x = (current_window_position.x as f32 + delta_x) as i32;
                },
                Edge::BottomRight => {
                    new_width = (current_window_size.width as f32 + delta_x).max(100.0);
                    new_height = (current_window_size.height as f32 + delta_y).max(100.0);
                },
            }

            window.set_outer_position(PhysicalPosition::new(new_position_x, current_window_position.y));
            let _ = window.request_inner_size(PhysicalSize::new(new_width as u32, new_height as u32));
            window.request_redraw();
        }
        _ => ()
    }
}