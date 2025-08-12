use rendering::user_interface::interface::Interface;

/// Generates elements in a list format
macro_rules! list {
    ($ui:ident, $total_size:expr, $total_unique:expr, $token_count:expr, ) => {};

    ($ui:ident, $total_size:expr, $total_unique:expr, $token_count:expr, button $color:expr, $($rest:tt)*) => {
        let unit_space = $total_size[1] / $total_unique as f32;

        let position = [
            $total_size[0] as f32 / 2.0,
            (unit_space * $token_count as f32) + (unit_space / 2.0),
        ];

        let scale = [
            $total_size[0] as f32,
            unit_space
        ];

        println!("button: {:?}", position);
        $ui.add_button(
            position,
            $color,
            scale,
            Box::new(|| {println!("Clicked")}),
            "solid",
        );
        list!($ui, $total_size, $total_unique, $token_count + 1, $($rest)*);
    };

    ($ui:ident, $total_size:expr, $total_unique:expr, $token_count:expr, button_label $t:expr, $($rest:tt)*) => {
        let unit_space = $total_size[1] / $total_unique as f32;

        let position = [
            $total_size[0] as f32 / 2.0,
            (unit_space * ($token_count as f32 - 1.0)) + (unit_space / 2.0),
        ];
        
        let scale = [
            $total_size[0] as f32,
            unit_space
        ];

        println!("label: {:?}", position);
        $ui.add_label(
            $t,
            position,
            scale,
            [1.0, 1.0, 1.0, 1.0],
        );
        list!($ui, $total_size, $total_unique, $token_count, $($rest)*);
    };

    ($ui:ident, $total_size:expr, $total_unique:expr, $token_count:expr, label $t:expr, $($rest:tt)*) => {
        let unit_space = $total_size[1] / $total_unique as f32;

        let position = [
            $total_size[0] as f32 / 2.0,
            (unit_space * $token_count as f32),
        ];

        let scale = [
            $total_size[0] as f32,
            unit_space
        ];

        println!("label: {:?}", position);
        $ui.add_label(
            $t,
            position,
            scale,
            [1.0, 1.0, 1.0, 1.0],
        );
        list!($ui, $total_size, $total_unique, $token_count + 1, $($rest)*);
    };
}

pub fn list(mut interface: Interface) -> Interface {
    interface.show(|ui| {
        // This is the correct way to call a recursive macro
        //ui.add_label(text, relative_position, relative_scale, color);
        //ui.add_button([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], [0.1, 0.1], Box::new(|| {println!("Clicked")}), "solid");
        //ui.add_label("flag", [0.25, 0.005], [0.1, 0.1], [0.0, 0.0, 1.0, 1.0]);
        list!(
            ui, [1.0, 1.0], 4, 0,
            button [0.0, 0.0, 1.0, 1.0],
            button_label "Button1",
            button [0.0, 0.0, 1.0, 1.0],
            button_label "Button2",
            button [0.0, 0.0, 1.0, 1.0],
            button_label "Button3",
            button [0.0, 0.0, 1.0, 1.0],
            button_label "Button4",
            //button [1.0, 0.0, 0.0, 1.0],
        );
    });
    interface
}