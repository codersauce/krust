use avr_device::atmega32u4::PLL;
use usb_device::{
    device::{UsbDevice, UsbDeviceState},
    prelude::*,
};
use usbd_hid::{descriptor::KeyboardReport, hid_class::HIDClass};

use crate::{keycodes::Keycode, UsbBus};

pub struct UsbKeyboard<B: usb_device::bus::UsbBus + 'static> {
    usb_device: UsbDevice<'static, B>,
    hid_class: HIDClass<'static, B>,
    // status_led: Pin<Output>,
    last_report: MyKeyboardReport,
}

// Wrapper type for KeyboardReport
#[derive(Clone, Copy, Debug)]
pub struct MyKeyboardReport(pub KeyboardReport);

// Implement PartialEq for the wrapper
impl PartialEq for MyKeyboardReport {
    fn eq(&self, other: &Self) -> bool {
        self.0.modifier == other.0.modifier
            && self.0.reserved == other.0.reserved
            && self.0.leds == other.0.leds
            && self.0.keycodes == other.0.keycodes
    }
}

impl UsbKeyboard<UsbBus<PLL>> {
    pub fn new(
        usb_device: UsbDevice<'static, UsbBus<PLL>>,
        hid_class: HIDClass<'static, UsbBus<PLL>>,
        // status_led: Pin<Output>,
    ) -> UsbKeyboard<UsbBus<PLL>> {
        UsbKeyboard {
            usb_device,
            hid_class,
            // status_led,
            last_report: MyKeyboardReport(KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0; 6],
            }),
        }
    }

    pub fn handle_keypress(&mut self, keycode: Keycode, pressed: bool) {
        let report = self.update_report(keycode, pressed);
        self.write_report(&report);
    }

    fn update_report(&mut self, keycode: Keycode, pressed: bool) -> MyKeyboardReport {
        // Update the last report based on the new key press
        if pressed {
            if keycode.is_modifier() {
                self.last_report.0.modifier |= keycode.to_modifier_bitfield();
            } else {
                // Add keycode to the first available slot in keycodes array
                if let Some(slot) = self
                    .last_report
                    .0
                    .keycodes
                    .iter_mut()
                    .find(|slot| **slot == 0)
                {
                    *slot = keycode.into();
                }
            }
        } else {
            if keycode.is_modifier() {
                self.last_report.0.modifier &= !keycode.to_modifier_bitfield();
            } else {
                // Remove the keycode from the keycodes array
                for slot in self.last_report.0.keycodes.iter_mut() {
                    if *slot == keycode.into() {
                        *slot = 0;
                        break;
                    }
                }
            }
        }
        self.last_report
    }

    fn write_report(&mut self, report: &MyKeyboardReport) {
        // Send the report
        match self.hid_class.push_input(&report.0) {
            Ok(_) => (),
            Err(UsbError::WouldBlock) => {
                // Handle WouldBlock error (non-blocking mode)
            }
            Err(err) => {
                // Handle other errors
                panic!("Failed to write keyboard report: {:?}", err);
            }
        }
    }

    pub fn poll(&mut self) {
        if self.usb_device.poll(&mut [&mut self.hid_class]) {
            let mut report_buf = [0u8; 1];
            if self.hid_class.pull_raw_output(&mut report_buf).is_ok() {
                // Process received data if needed, e.g., check for Num Lock status
                if report_buf[0] & 2 != 0 {
                    // self.status_led.set_high();
                } else {
                    // self.status_led.set_low();
                }
            }
        }
        if self.usb_device.state() == UsbDeviceState::Configured {
            let blank_report = MyKeyboardReport(KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0; 6],
            });
            if self.last_report != blank_report {
                self.write_report(&blank_report);
                self.last_report = blank_report;
            }
        }
    }
}
