# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

### Building the firmware
```bash
cargo build --release
```

### Running/Flashing to device
The project is configured to use `elf2uf2-rs` for uploading:
```bash
cargo run --release
```

This converts the ELF binary to UF2 format and automatically uploads it to a connected Raspberry Pi Pico in bootloader mode.

Alternative runner (commented out in config):
```bash
probe-rs run --chip RP2040
```

### Build target
Target platform: `thumbv6m-none-eabi` (Cortex-M0/M0+)

## Architecture Overview

### Hardware Platform
- **Target**: Raspberry Pi Pico (RP2040 microcontroller)
- **Display**: ST7789V3 320x170 LCD connected via SPI
- **Framework**: Embassy async runtime for embedded Rust

### Code Structure

#### Core Application (`src/main.rs`)
- Embassy executor-based async main loop
- Initializes Raspberry Pi Pico peripherals
- Configures SPI interface for LCD communication
- Sets up ST7789 display driver with landscape orientation
- Displays text and embedded images
- Implements LED blinking loop

#### Pin Configuration (`src/config/pins.rs`)
- Centralizes all GPIO pin assignments
- **LED**: Pin 25 (onboard LED)
- **SPI Bus**: SPI1 on pins 10-12 (SCK, TX, RX)
- **LCD Control**: Pins 13-16 (DC, CS, Backlight, Reset)

#### Memory Layout (`memory.x`)
- RP2040-specific memory configuration
- Boot2 bootloader section at 0x10000000
- Main flash at 0x10000100 (2MB - 256 bytes)
- RAM at 0x20000000 (264KB unified layout)

### Key Dependencies
- **Embassy**: Async executor and hardware abstraction
- **ST7789**: Display driver for the LCD
- **Embedded Graphics**: Drawing primitives and text rendering
- **defmt**: Structured logging for embedded systems

### Image Assets
- Raw binary image format stored in `src/images/`
- Images converted from PNG using `tool/img_converter.py`
- Conversion tool handles RGBA → RGB565 format with transparency support

### Development Tools
- **Image Converter** (`tool/img_converter.py`): Converts PNG images to RGB565 raw format
  - Usage: Place PNG files in `tool/original/`, run script, output goes to `tool/output/`
  - Handles transparency by converting to black pixels

### Logging Configuration
- DEFMT_LOG level set to "debug" in cargo config
- Uses defmt-rtt for real-time transfer debugging