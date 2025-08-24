use std::sync::{Arc, Mutex};

use rendering::{definitions::UiAtlas, user_interface::interface::Interface, RenderState};
use tracing_flame::FlameLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[allow(unused_imports)]
use winit::{application::ApplicationHandler, dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, MouseButton, WindowEvent}, event_loop::{ControlFlow, EventLoop}, keyboard::{Key, NamedKey}, platform::modifier_supplement::KeyEventExtModifierSupplement, window::{CursorIcon, Window}};

use crate::utils::{atlas_generation::generate_texture_atlas, core::{AppLogic, AppState}};

mod utils;

fn main() {
    let atlas = generate_texture_atlas();
    let mut app = App::new(atlas);

    let fmt_layer = tracing_subscriber::fmt::Layer::default().pretty();
    let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(flame_layer)
        .init();
    //let subscriber = Registry::default().with(flame_layer);
    //tracing::subscriber::set_global_default(subscriber).expect("Could not set global subscriber");
    //env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    window_ref: Option<Arc<Window>>,
    logic: AppLogic<Window>
}

impl App {
    fn new(atlas: UiAtlas) -> Self {
        Self {
            window_ref: None,
            logic: AppLogic::new(None, Arc::new(Mutex::new(Interface::new(atlas.clone()))), atlas)
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
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
        match self.logic.app_data.state {
            AppState::Default => self.logic.default_state_event_handler(event_loop, event),
            AppState::Resizing => self.logic.resizing_state_event_handler(event_loop, event),
        }
    }
}