#![no_std]
#![cfg_attr(not(test), no_main)]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
// #![deny(unsafe_op_in_unsafe_fn)]

use avr_device::{
    asm::sleep,
    atmega32u4::{Peripherals, PLL, TC1},
};
use avr_device_macros::{entry, interrupt};
use krust::{delay::delay_ms, keyboard_config, layers, matrix, pins, usb_keyboard, UsbBus};
use usb_device::{
    class_prelude::UsbBusAllocator,
    descriptor::lang_id::LangID,
    device::{UsbDeviceBuilder, UsbVidPid},
    prelude::StringDescriptors,
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

use keyboard_config::{MATRIX_COLS, MATRIX_ROWS};
use layers::Layers;
use matrix::Matrix;
use usb_keyboard::UsbKeyboard;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let pll = dp.PLL;
    let usb = dp.USB_DEVICE;

    // Configure PLL interface
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());

    // Check PLL lock with timeout
    let mut timeout = 0;
    while pll.pllcsr.read().plock().bit_is_clear() {
        timeout += 1;
        if timeout > 1000 {
            // PLL lock failed - blink rapidly
            loop {
                // led.toggle();
                delay_ms(100);
            }
        }
    }

    // Initialize USB
    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus<PLL>>> = None;
        &*USB_BUS.insert(UsbBus::with_suspend_notifier(usb, pll))
    };

    let hid_class = HIDClass::new(usb_bus, KeyboardReport::desc(), 1);
    let strings = StringDescriptors::new(LangID::EN)
        .manufacturer("Custom Keyboard")
        .product("DZ60");

    let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x445A, 0x2260))
        .strings(&[strings])
        .unwrap()
        .build();

    let matrix = Matrix::new(pins);
    let layers = Layers::new();
    let usb_keyboard = UsbKeyboard::new(usb_device, hid_class);

    unsafe {
        USB_CTX = Some(usb_keyboard);
        MATRIX_CTX = Some(matrix);
        LAYERS_CTX = Some(layers);
    }

    // Configure Timer1 for scanning
    unsafe {
        let tc1 = &*TC1::ptr();
        tc1.tccr1a.write(|w| w.wgm1().bits(0b00));
        tc1.tccr1b
            .write(|w| w.cs1().prescale_64().wgm1().bits(0b01));
        tc1.ocr1a.write(|w| w.bits(1000)); // ~1kHz scanning rate
        tc1.timsk1.write(|w| w.ocie1a().set_bit());

        avr_device::interrupt::enable();
    }

    loop {
        sleep();
    }
}

static mut USB_CTX: Option<UsbKeyboard<UsbBus<PLL>>> = None;
static mut MATRIX_CTX: Option<Matrix> = None;
static mut LAYERS_CTX: Option<Layers> = None;

#[interrupt(atmega32u4)]
fn USB_GEN() {
    unsafe {
        #[allow(static_mut_refs)]
        if let Some(usb_keyboard) = &mut USB_CTX {
            usb_keyboard.poll();
        }
    }
}

#[interrupt(atmega32u4)]
fn USB_COM() {
    unsafe {
        #[allow(static_mut_refs)]
        if let Some(usb_keyboard) = &mut USB_CTX {
            usb_keyboard.poll();
        }
    }
}

// Timer1 Compare A interrupt
#[interrupt(atmega32u4)]
fn TIMER1_COMPA() {
    unsafe {
        #[allow(static_mut_refs)]
        if let Some(usb_keyboard) = &mut USB_CTX {
            if let Some(matrix) = &mut MATRIX_CTX {
                if let Some(layers) = &mut LAYERS_CTX {
                    let new_state = matrix.scan();
                    let layer = layers.get_current_layer();
                    for row in 0..MATRIX_ROWS {
                        for col in 0..MATRIX_COLS {
                            if new_state[row][col] != matrix.last_state[row][col] {
                                let keycode = layers.get_keycode(layer, row, col);
                                let is_pressed = new_state[row][col];
                                usb_keyboard.handle_keypress(keycode, is_pressed);
                            }
                        }
                    }
                    matrix.last_state = new_state;
                }
            }
        }
    }
}
