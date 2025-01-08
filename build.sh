#!/usr/bin/env bash

set -e

# takes the board name as first param
BOARD=$1

# provide help if not given
if [ -z "$BOARD" ]; then
    echo "Usage: $0 <board>"
    echo "Available boards: "
    ls -1 src/boards | sed 's/\.rs//' | grep -Ev "mod|macros"
    exit 1
fi

rm -f krust.hex
cargo build --release --features $BOARD
avr-objcopy -O ihex -R .eeprom target/avr-atmega32u4/release/krust.elf $BOARD.hex
dfu-programmer atmega32u4 flash $BOARD.hex --erase-first
dfu-programmer atmega32u4 reset
