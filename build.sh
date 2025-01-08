#!/usr/bin/env bash

rm -f krust.hex
cargo build --release
avr-objcopy -O ihex -R .eeprom target/avr-atmega32u4/release/krust.elf krust.hex
dfu-programmer atmega32u4 flash krust.hex --erase-first
dfu-programmer atmega32u4 reset
