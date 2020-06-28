# rephrase

Rephrase translates PS4 DualShock 4 controller commands into a Nintendo Joycon
controller commands.

## Why?

I have a DS4 controller and I want it to work with my Nintendo Switch console.

## Architecture

This project contains two main Rust crates that implement this translation:

- [reader](src/reader): Reads DS4 input over USB (and in the future Bluetooth).
- [spoofer](src/spoofer): Turns a STM32F4 Discovery microcontroller into a
  Nintendo Joycon emulator using the micro USB port.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.