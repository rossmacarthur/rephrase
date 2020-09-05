# rephrase-spoofer

Emulates a Nintendo Switch controller.

## Development

## Installation

Install GDB and OpenOCD using Homebrew.

```sh
# GDB
brew install armmbed/formulae/arm-none-eabi-gcc

# OpenOCD
brew install openocd
```

Install the `thumbv7em-none-eabihf` Rust target.

```sh
rustup target add thumbv7em-none-eabihf
```

### Running

Start OpenOCD to connect it to the STM32F4DISCOVERY and leave this running in
another terminal.

```sh
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg
```

Run the following to build this crate and open GDB where it will automatically
connect to OpenOCD and load the binary (see `openocd.gdb`).

```sh
cargo run
```

You can then [debug the code](http://sourceware.org/gdb/current/onlinedocs/gdb/)
using GDB.
