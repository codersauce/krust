use crate::port::Dynamic;
use avr_hal_generic::port::{
    mode::{Input, Output, PullUp},
    Pin,
};

use crate::keyboard_config::{MATRIX_COLS, MATRIX_ROWS};
use crate::port::Pins;

pub struct Matrix {
    pub last_state: [[bool; MATRIX_COLS]; MATRIX_ROWS],
    rows: [Pin<Output, Dynamic>; MATRIX_ROWS],
    cols: [Pin<Input<PullUp>, Dynamic>; MATRIX_COLS],
}

impl Matrix {
    pub fn new(pins: Pins) -> Self {
        Matrix {
            last_state: [[false; MATRIX_COLS]; MATRIX_ROWS],
            rows: [
                pins.pd0.into_output().downgrade(), // PD0
                pins.pd1.into_output().downgrade(), // PD1
                pins.pd2.into_output().downgrade(), // PD2
                pins.pd3.into_output().downgrade(), // PD3
                pins.pd5.into_output().downgrade(), // PD5
            ],
            cols: [
                pins.pf0.into_pull_up_input().downgrade(), // PF0
                pins.pf1.into_pull_up_input().downgrade(), // PF1
                pins.pe6.into_pull_up_input().downgrade(), // PE6
                pins.pc7.into_pull_up_input().downgrade(), // PC7
                pins.pc6.into_pull_up_input().downgrade(), // PC6
                pins.pb7.into_pull_up_input().downgrade(), // PB7
                pins.pd4.into_pull_up_input().downgrade(), // PD4
                pins.pb1.into_pull_up_input().downgrade(), // PB1
                pins.pb0.into_pull_up_input().downgrade(), // PB0
                pins.pb5.into_pull_up_input().downgrade(), // PB5
                pins.pb4.into_pull_up_input().downgrade(), // PB4
                pins.pd7.into_pull_up_input().downgrade(), // PD7
                pins.pd6.into_pull_up_input().downgrade(), // PD6
                pins.pb3.into_pull_up_input().downgrade(), // PB3
                pins.pf4.into_pull_up_input().downgrade(), // PF4
            ],
        }
    }

    pub fn scan(&mut self) -> [[bool; MATRIX_COLS]; MATRIX_ROWS] {
        let mut new_state = [[false; MATRIX_COLS]; MATRIX_ROWS];

        for (row_idx, row_pin) in self.rows.iter_mut().enumerate() {
            row_pin.set_low();

            for (col_idx, col_pin) in self.cols.iter().enumerate() {
                new_state[row_idx][col_idx] = col_pin.is_low();
            }

            row_pin.set_high();
        }

        new_state
    }
}
