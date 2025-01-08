# Krust - Your Keyboard using Rust

Open-source keyboard firmware for AVR devices written in Rust.

## Features

- **Matrix Scanning**: Supports a configurable matrix for key detection.
- **USB HID Keyboard**: Implements a USB HID keyboard device, allowing the keyboard to be recognized by any modern operating system.
- **Layers**: Supports multiple keymap layers, enabling complex keybindings and macros.
- **Custom Keycodes**: Defines a comprehensive set of keycodes, including modifiers and special keys.
- **Power Management**: Implements USB suspend/resume functionality with optional PLL power-down.

## Hardware Requirements

- **Microcontroller**: ATmega32u4
- **USB**: USB 2.0 for communication with the host device

## Building the Firmware

To build the firmware, you'll need the following tools:

- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **AVR Toolchain**: Install the AVR toolchain for your platform. On Linux, you can use `avr-gcc`.

### Steps to Build

1. Clone the repository:

   ```bash
   git clone https://github.com/codersauce/krust.git
   cd krust
   ```

2. Build the firmware:

   ```bash
   cargo build --release
   ```

3. Prepare the hex file for avr:

   ```bash
   avr-objcopy -O ihex -R .eeprom target/avr-atmega32u4/release/examples/dz60.elf dz60.hex
   ```

4. Flash the firmware to the microcontroller using `dfu-programmer`:

   ```bash
   dfu-programmer atmega32u4 flash dz60.hex --erase-first
   dfu-programmer atmega32u4 reset
   ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

## Acknowledgments

- **avr-hal**: For providing a robust hardware abstraction layer for AVR microcontrollers.
- **usb-device**: For enabling USB communication in Rust.
- **QMK Firmware**: For inspiration and reference in keyboard firmware design.
