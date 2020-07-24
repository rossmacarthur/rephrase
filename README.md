# rephrase

Rephrase translates Sony DualShock 4 controller commands into a Nintendo Joycon
controller commands.

## Why?

I have a DS4 controller and I want it to work with my Nintendo Switch console.

## Architecture

This project contains three main Rust crates that implement this translation:

- [gamepad](gamepad): Defines a generic gamepad interface for use in the below
  crates.
- [reader](reader): Reads Sony DualShock 4 input over USB (and in the future
  Bluetooth).
- [spoofer](spoofer): Turns a STM32F4 Discovery microcontroller into a Nintendo
  Joycon emulator using the micro USB port.

## This project is still a work in progress!

- [x] Implement DualShock 4 input parsing (currently over USB on Linux only).
- [x] Emulate a skeleton USB HID device on the STM32F4 Discovery.
- [ ] Emulate a wired Switch Pro Controller and correctly interface with the
  Switch console.
- [ ] Implement DualShock 4 Bluetooth receiver on STM32F4.
- [ ] Forward all reports to the Switch console.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
