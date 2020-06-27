# rephrase-spoofer

Emulates a Nintendo Switch controller.

## Development

Start OpenOCD to connect it to the STM32F4DISCOVERY and leave this running in
another terminal.

```
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg
```

Run the following to build this crate and open GDB where it will automatically
run the following for you (in `.gdbinit`).

```
(gdb) target remote :3333
(gdb) load
```

You can then debug the code using GDB.
