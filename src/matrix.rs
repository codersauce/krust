use crate::port::Dynamic;
use avr_hal_generic::port::{
    mode::{Input, Output, PullUp},
    Pin,
};

use crate::boards::current_board::{
    self,
    config::{MATRIX_COLS, MATRIX_ROWS},
};
use crate::port::Pins;

const DEBOUNCE_SCANS: u8 = 5;

pub struct Matrix {
    state: [[bool; MATRIX_COLS]; MATRIX_ROWS],
    debounce_counters: [[u8; MATRIX_COLS]; MATRIX_ROWS],
    rows: [Pin<Output, Dynamic>; MATRIX_ROWS],
    cols: [Pin<Input<PullUp>, Dynamic>; MATRIX_COLS],
}

impl Matrix {
    pub fn new(pins: Pins) -> Self {
        let (rows, cols) = current_board::matrix_pins(pins);
        let mut matrix = Matrix {
            state: [[false; MATRIX_COLS]; MATRIX_ROWS],
            debounce_counters: [[0; MATRIX_COLS]; MATRIX_ROWS],
            rows,
            cols,
        };

        matrix.set_all_rows_high();
        matrix
    }

    pub fn state(&self) -> [[bool; MATRIX_COLS]; MATRIX_ROWS] {
        self.state
    }

    pub fn scan(&mut self) -> Option<[[bool; MATRIX_COLS]; MATRIX_ROWS]> {
        let raw_state = self.scan_raw();
        let mut changed = false;

        for row in 0..MATRIX_ROWS {
            for col in 0..MATRIX_COLS {
                if raw_state[row][col] == self.state[row][col] {
                    self.debounce_counters[row][col] = 0;
                    continue;
                }

                let counter = &mut self.debounce_counters[row][col];
                *counter = counter.saturating_add(1);

                if *counter >= DEBOUNCE_SCANS {
                    self.state[row][col] = raw_state[row][col];
                    self.debounce_counters[row][col] = 0;
                    changed = true;
                }
            }
        }

        if changed {
            Some(self.state)
        } else {
            None
        }
    }

    fn scan_raw(&mut self) -> [[bool; MATRIX_COLS]; MATRIX_ROWS] {
        let mut new_state = [[false; MATRIX_COLS]; MATRIX_ROWS];

        self.set_all_rows_high();

        for (row_idx, row_pin) in self.rows.iter_mut().enumerate() {
            row_pin.set_low();

            for (col_idx, col_pin) in self.cols.iter().enumerate() {
                new_state[row_idx][col_idx] = col_pin.is_low();
            }

            row_pin.set_high();
        }

        new_state
    }

    fn set_all_rows_high(&mut self) {
        for row in self.rows.iter_mut() {
            row.set_high();
        }
    }
}
