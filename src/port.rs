pub use avr_hal_generic::port::{mode, PinMode, PinOps};

avr_hal_generic::impl_port_traditional! {
    enum Ports {
        B: avr_device::atmega32u4::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: avr_device::atmega32u4::PORTC = [6, 7],
        D: avr_device::atmega32u4::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
        E: avr_device::atmega32u4::PORTE = [2, 6],
        F: avr_device::atmega32u4::PORTF = [0, 1, 4, 5, 6, 7],
    }
}
