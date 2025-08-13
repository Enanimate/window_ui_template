use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::interface::Interface, RenderState};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{MouseButton, WindowEvent}, event_loop::EventLoop, window::Window};

use crate::utils::{atlas_generation::generate_texture_atlas, componenents::list};

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
            atlas,
        };

        env_logger::init();
        
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }

    fn handle_click(&self, cursor_position: [f32; 2]) {
        let interface_guard = self.interface.lock().unwrap();
        let window_size = [self.window_size.width, self.window_size.height];

        for element in &interface_guard.elements {
            let element_position = element.get_position(window_size);
            let element_scale = element.get_scale(window_size);

            if element.is_cursor_within_bounds(cursor_position, element_position, element_scale) {
                element.handle_click();
            }
        }
    }

    fn rebuild_interface(&mut self) {
        println!("window size: {:?}", self.window_size);
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
            
        });

        //interface = list(interface);

        return interface;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_maximized(true);
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
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state.is_pressed() {
                    self.handle_click(self.cursor_position);
                }
            }
            _ => ()
        }

        if let Some(window_arc) = self.window_ref.as_ref() {
            window_arc.request_redraw();
        }

        if needs_rebuild {
            self.rebuild_interface();
        }
    }
}