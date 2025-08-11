use rendering::user_interface::{elements::Label, interface::Interface};

pub fn list(mut interface: Interface) -> Interface {
    interface.show(|ui| {
        ui.add_label(Label::new("test", [0.0, 0.0], [0.0, 0.0], [0.0, 0.0, 1.0, 1.0]));
    });

    interface
}