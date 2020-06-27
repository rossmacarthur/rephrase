#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};
use hal::gpio::{Output, PushPull};

#[entry]
fn main() -> ! {
    loop {}
}
