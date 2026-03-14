use crate::define_board;

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
    keymap: include!("../keymaps/dz60_default.rs")
}
