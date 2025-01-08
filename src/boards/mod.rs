use crate::define_board;

pub mod macros;

define_board! {
    name: dz60,
    dimensions: [5, 15],
    layers: 2,
    usb: {
        vid: 0x445A,
        pid: 0x2260,
        manufacturer: "KBDFans",
        product: "DZ60"
    },
    matrix: {
        rows: [ pd0, pd1, pd2, pd3, pd5 ],
        cols: [ pf0, pf1, pe6, pc7, pc6, pb7, pd4, pb1, pb0, pb5, pb4, pd7, pd6, pb3, pf4 ]
    },
    keymap: [
        // Layer 0 - Base QWERTY layer
        [
            // Row 0
            [ Escape, N1, N2, N3, N4, N5, N6, N7, N8, N9, N0, Minus, Equal, Backspace, Backspace ],
            [ Tab, No,    Q, W, E, R, T, Y, U, I, O, P,     LeftBracket, RightBracket, Backslash ],
            [ CapsLock, No, A, S, D, F, G, H, J, K, L,        Semicolon, Apostrofe, Enter, Enter ],
            [ LShift,          Z, X, C, V, B, N, M,        Comma, Dot, Slash, RShift, Up, No, No ],
            [ LCtrl, LGui, LAlt, Space, Space, Space, Space, RAlt, RGui, Application, No, No, No, No, No ],
        ],
        // Layer 1 - Function layer
        [
            [Grave; MATRIX_COLS], // Row 0 - all grave for example
            [No; MATRIX_COLS],    // Row 1
            [No; MATRIX_COLS],    // Row 2
            [No; MATRIX_COLS],    // Row 3
            [No; MATRIX_COLS],    // Row 4
        ],
    ]
}
