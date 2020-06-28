#![no_main]
#![no_std]

mod vec;

use core::fmt::Write;

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;

use hal::{block, prelude::*, serial, stm32};
use vec::Vec;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let gpioa = dp.GPIOA.split();

    let tx = gpioa.pa2.into_alternate_af7();
    let rx = gpioa.pa3.into_alternate_af7();

    let config = serial::config::Config::default().baudrate(115_200.bps());
    let usart = serial::Serial::usart2(dp.USART2, (tx, rx), config, clocks).unwrap();

    let (mut tx, mut rx) = usart.split();

    let mut _buf = [0; 1024];
    let mut buf = Vec::new(&mut _buf);
    loop {
        let byte = block!(rx.read()).unwrap();
        buf.push(byte);
        tx.write(byte).unwrap();
        if byte == b'\n' {
            writeln!(tx, "echo: {}", buf.as_str().unwrap()).unwrap();
            buf.clear();
        }
    }
}
