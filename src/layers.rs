use crate::boards::current_board::config::{MATRIX_COLS, MATRIX_ROWS, NUM_LAYERS};
use crate::keycodes::Keycode;

pub struct Layers {
    keymaps: [[[Keycode; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS],
    current_layer: usize,
    pressed_keycodes: [[Keycode; MATRIX_COLS]; MATRIX_ROWS],
}

impl Layers {
    pub fn new() -> Self {
        Layers {
            keymaps: crate::boards::current_board::keymap::KEYMAPS,
            current_layer: 0,
            pressed_keycodes: [[Keycode::No; MATRIX_COLS]; MATRIX_ROWS],
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

    pub fn keycode_for_event(&mut self, row: usize, col: usize, pressed: bool) -> Keycode {
        if pressed {
            let keycode = self.get_keycode(self.current_layer, row, col);
            self.pressed_keycodes[row][col] = keycode;
            keycode
        } else {
            let keycode = self.pressed_keycodes[row][col];
            self.pressed_keycodes[row][col] = Keycode::No;
            keycode
        }
    }

    pub fn handle_layer_key(&mut self, keycode: Keycode, pressed: bool) -> bool {
        match keycode.layer_target() {
            Some(layer) => {
                if pressed {
                    self.set_layer(layer);
                } else {
                    self.set_layer(0);
                }
                true
            }
            None => false,
        }
    }
}
