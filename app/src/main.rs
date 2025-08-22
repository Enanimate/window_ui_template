use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::{elements::{ElementType, InteractionResult}, interface::Interface}, RenderState};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event_loop::{ControlFlow, EventLoop}, window::{CursorIcon, Window}};

use crate::utils::{atlas_generation::generate_texture_atlas, components::header_componenet, definitions::{Edge, State}, state_events::{state_normal, state_resizing}};

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
    resizing: bool,
    state: State,
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
            resizing: false,
            state: State::Normal,
        };

        env_logger::init();
        
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }

    fn handle_click(&self, cursor_position: [f32; 2]) -> InteractionResult {
        let mut interface_guard = self.interface.lock().unwrap();
        let window_size = [self.window_size.width, self.window_size.height];
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

    fn old_handle_hover(&self, cursor_position: [f32; 2]) -> Option<u32> {
        let mut interface_guard = self.interface.lock().unwrap();
        let window_size = [self.window_size.width, self.window_size.height];
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

    fn handle_hover(&mut self, cursor_position: [f32; 2]) -> Option<u32> {
        let mut interface_guard = self.interface.lock().unwrap();
        let window_size = [self.window_size.width, self.window_size.height];
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

        if result.is_some() {
            return result;
        }

        let resize_event_area = match self.state {
            State::Normal => {
                println!("Entering Normal State");
                4.0
            }
            State::Resizing => {
                println!("Entering Resizing State");
                50.0
            },
        };

        let is_on_left_edge = cursor_position[0] <= resize_event_area;
        let is_on_right_edge = cursor_position[0] >= window_size[0] as f32 - resize_event_area;
        let is_on_bottom_edge = cursor_position[1] >= window_size[1] as f32 - resize_event_area;

        if is_on_left_edge || is_on_right_edge || is_on_bottom_edge {
            self.state = State::Resizing
        } else {
            self.state = State::Normal
        }

        return None;
    }

    fn handle_resizing(&self, cursor_position: [f32; 2], window_size: [f32; 2]) -> Edge {
        let mut resize_event_area = 2.0;
        if self.resizing {
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

        self.window_ref.clone().unwrap().set_cursor(cursor_icon);
        return side;
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

        // TODO: Implement manual window sizing. Should be straight forward, similar to panel with id: 0.

        interface.show(|ui| {
            header_componenet(ui);
            ui.add_textbox("placeholder", [0.5, 0.5], [0.5, 0.5], "#ffffffff");
        });

        return interface;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let window_attributes = Window::default_attributes().with_maximized(true).with_decorations(false);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let interface_arc = Arc::clone(&self.interface);
        
        self.window_ref = Some(window.clone());
        self.render_state = Some(pollster::block_on(RenderState::new(window.clone(), interface_arc)).unwrap());

        self.rebuild_interface();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {

        match self.state {
            State::Normal => state_normal(self, &event, event_loop),
            State::Resizing => state_resizing(self, &event),
        }

        if let Some(window_arc) = self.window_ref.as_ref() {
            window_arc.request_redraw();
        }
    }
}