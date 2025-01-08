use crate::keyboard_config::{MATRIX_COLS, MATRIX_ROWS};
use crate::keycodes::Keycode;

pub const NUM_LAYERS: usize = 2;

pub struct Layers {
    keymaps: [[[Keycode; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS],
    current_layer: usize,
}

impl Layers {
    pub fn new() -> Self {
        let keymaps = [
            // Layer 0 - Base QWERTY layer
            [
                // Row 0
                [
                    Keycode::Escape,
                    Keycode::N1,
                    Keycode::N2,
                    Keycode::N3,
                    Keycode::N4,
                    Keycode::N5,
                    Keycode::N6,
                    Keycode::N7,
                    Keycode::N8,
                    Keycode::N9,
                    Keycode::N0,
                    Keycode::Minus,
                    Keycode::Equal,
                    Keycode::Backspace,
                    Keycode::Backspace,
                ],
                // Row 1
                [
                    Keycode::Tab,
                    Keycode::No,
                    Keycode::Q,
                    Keycode::W,
                    Keycode::E,
                    Keycode::R,
                    Keycode::T,
                    Keycode::Y,
                    Keycode::U,
                    Keycode::I,
                    Keycode::O,
                    Keycode::P,
                    Keycode::LeftBracket,
                    Keycode::RightBracket,
                    Keycode::Backslash,
                ],
                // Row 2
                [
                    Keycode::CapsLock,
                    Keycode::No,
                    Keycode::A,
                    Keycode::S,
                    Keycode::D,
                    Keycode::F,
                    Keycode::G,
                    Keycode::H,
                    Keycode::J,
                    Keycode::K,
                    Keycode::L,
                    Keycode::Semicolon,
                    Keycode::Apostrofe,
                    Keycode::Enter,
                    Keycode::Enter,
                ],
                // Row 3
                [
                    Keycode::LShift,
                    Keycode::Z,
                    Keycode::X,
                    Keycode::C,
                    Keycode::V,
                    Keycode::B,
                    Keycode::N,
                    Keycode::M,
                    Keycode::Comma,
                    Keycode::Dot,
                    Keycode::Slash,
                    Keycode::RShift,
                    Keycode::Up,
                    Keycode::No,
                    Keycode::No,
                ],
                // Row 4
                [
                    Keycode::LCtrl,
                    Keycode::LGui,
                    Keycode::LAlt,
                    Keycode::Space,
                    Keycode::Space,
                    Keycode::Space,
                    Keycode::Space,
                    Keycode::RAlt,
                    Keycode::RGui,
                    Keycode::Application,
                    Keycode::RCtrl,
                    Keycode::Left,
                    Keycode::Down,
                    Keycode::Right,
                    Keycode::No,
                ],
            ],
            // Layer 1 - Function layer
            [
                [Keycode::Grave; MATRIX_COLS], // Row 0 - all grave for example
                [Keycode::No; MATRIX_COLS],    // Row 1
                [Keycode::No; MATRIX_COLS],    // Row 2
                [Keycode::No; MATRIX_COLS],    // Row 3
                [Keycode::No; MATRIX_COLS],    // Row 4
            ],
        ];

        Layers {
            keymaps,
            current_layer: 0,
        }
    }

    pub fn get_keycode(&self, layer: usize, row: usize, col: usize) -> Keycode {
        self.keymaps[layer][row][col]
    }

    pub fn get_current_layer(&self) -> usize {
        self.current_layer
    }

    pub fn set_layer(&mut self, layer: usize) {
        if layer < NUM_LAYERS {
            self.current_layer = layer;
        }
    }
}
