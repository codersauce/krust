//! Replacement for avr-std-stub with a custom panic handler.

use core::panic::PanicInfo;

use avr_device::atmega32u4::Peripherals;

use crate::{pins, delay::delay_ms};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let dp = unsafe { Peripherals::steal() };
    let pins = pins!(dp);
    let mut status = pins.pb7.into_output();
    loop {
        status.set_high();
        delay_ms(100);
        status.set_low();
        delay_ms(100);
        status.set_high();
        delay_ms(300);
        status.set_low();
        delay_ms(500);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}
