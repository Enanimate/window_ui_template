use std::sync::{Arc, Mutex};

use rendering::{user_interface::{elements::Button, interface::Interface}, RenderState};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window};


mod utils;

fn main() {
    App::new();
}

struct App {
    render_state: Option<RenderState>,
    window_ref: Option<Arc<Window>>,
    interface: Arc<Mutex<Interface>>,
}

impl App {
    fn new() {
        let mut app = Self {
            render_state: None,
            window_ref: None,
            interface: Arc::new(Mutex::new(Interface::new())),
        };

        env_logger::init();
        
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }

    fn rebuild_interface(&mut self) {
        let new_interface_data = Self::build_project_view();

        if let Some(rs) = self.render_state.as_mut() {
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;

            interface_guard.initialize_interface_buffers(&rs.device, &rs.queue);
        } else {
            log::warn!("Attempted to rebuild interface but render_state was None. Cannot initialize GPU buffers.");
            let mut interface_guard = self.interface.lock().unwrap();
            *interface_guard = new_interface_data;
        }
    }

    fn build_project_view() -> Interface {
        let mut interface = Interface::new();

        interface.show(|ui| {
            ui.add_button(Button::new([-100.0, -100.0], [1.0, 0.0, 0.0, 1.0], [100.0, 50.0]));
            ui.add_button(Button::new([100.0, 100.0], [0.0, 0.0, 1.0, 1.0], [50.0, 100.0]));
        });
        return interface;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

        let interface_arc = Arc::clone(&self.interface);

        self.window_ref = Some(window.clone());
        self.render_state = Some(pollster::block_on(RenderState::new(window, interface_arc)).unwrap());

        self.rebuild_interface();

        if let Some(rs) = self.render_state.as_mut() {
            let mut interface_guard = self.interface.lock().unwrap();
            interface_guard.initialize_interface_buffers(&rs.device, &rs.queue);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let current_window_size = self.window_ref.as_ref().unwrap().inner_size();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
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
            _ => ()
        }

        if let Some(window_arc) = self.window_ref.as_ref() {
                window_arc.request_redraw();
            }
    }
}