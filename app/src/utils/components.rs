use rendering::user_interface::{elements::UiEvent, UserInterface};

pub fn header_componenet(ui: &mut UserInterface) {
    let header_y = 0.01;
    ui.add_panel(
        [0.5, header_y], 
        "#0d1117ff", 
        [1.0, header_y * 2.0], 
        "solid",
        Some(0)
    );

    // Close Button
    ui.add_prop_button(
        [0.99, header_y], 
        "#5c030300", 
        [0.02, header_y * 2.0], 
        Box::new(|| {UiEvent::CloseRequested}), 
        "solid"
    );
    ui.add_icon(
        [0.99, header_y], 
        "#ffffffff", 
        [10.0, 10.0], 
        "close"
    );

    // Maximize Button
    ui.add_prop_button(
        [0.97, header_y], 
        "#30363d00", 
        [0.02, header_y * 2.0], 
        Box::new(|| {UiEvent::ResizeRequested}), 
        "solid"
    );

    ui.add_icon(
        [0.97, header_y], 
        "#ffffffff", 
        [12.0, 12.0], 
        "maximize"
    );

    // Minimize Button
    ui.add_prop_button(
        [0.95, header_y], 
        "#30363d00", 
        [0.02, header_y * 2.0], 
        Box::new(|| {UiEvent::SetMinimized}), 
        "solid"
    );

    ui.add_icon(
        [0.95, header_y], 
        "#ffffffff", 
        [12.0, 12.0], 
        "minimize"
    );
}