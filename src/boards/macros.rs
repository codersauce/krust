#[macro_export]
macro_rules! define_board {
    (
        name: $name:ident,
        dimensions: [$rows:expr, $cols:expr],
        layers: $num_layers:expr,
        usb: {
            vid: $vid:expr,
            pid: $pid:expr,
            manufacturer: $manufacturer:expr,
            product: $product:expr
        },
        matrix: {
            rows: [$($row_pin:ident),+ $(,)?],
            cols: [$($col_pin:ident),+ $(,)?]
        },
        keymap: $keymap:expr
    ) => {
        paste::paste! {
            #[cfg(feature = "" $name "")]
            pub mod $name {
                use atmega_hal::port::mode::{Input, Output, PullUp};

                use $crate::port::{Dynamic, Pin, Pins};

                pub mod config {
                    use super::*;

                    pub const MATRIX_ROWS: usize = $rows;
                    pub const MATRIX_COLS: usize = $cols;
                    pub const NUM_LAYERS: usize = $num_layers;

                    pub const VENDOR_ID: u16 = $vid;
                    pub const PRODUCT_ID: u16 = $pid;
                    pub const MANUFACTURER: &str = $manufacturer;
                    pub const PRODUCT: &str = $product;

                    pub fn matrix_pins(pins: Pins) -> ([Pin<Output, Dynamic>; MATRIX_ROWS], [Pin<Input<PullUp>, Dynamic>; MATRIX_COLS]) {
                        (
                            [
                                $(pins.$row_pin.into_output().downgrade()),*
                            ],
                            [
                                $(pins.$col_pin.into_pull_up_input().downgrade()),*
                            ]
                        )
                    }
                }

                pub mod keymap {
                    use $crate::keycodes::Keycode;
                    use super::config::{MATRIX_COLS, MATRIX_ROWS, NUM_LAYERS};
                    use $crate::keycodes::Keycode::*;

                    pub const KEYMAPS: [[[crate::keycodes::Keycode; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS] = $keymap;
                }

                pub use config::*;
                pub use keymap::*;
            }
        }
    };
}
