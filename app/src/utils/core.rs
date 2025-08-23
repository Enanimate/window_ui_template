use std::sync::{Arc, Mutex};
use rendering::user_interface::interface::Interface;

use crate::utils::definitions::AppWindow;

pub struct AppLogic<W: AppWindow> {
    pub window: Option<Arc<W>>,
    pub interface: Arc<Mutex<Interface>>
}