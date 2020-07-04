#![no_main]
#![no_std]

mod usb_hid;
mod vec;

use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::{self, interrupt};
use stm32f4xx_hal::{block, serial};

use vec::Vec;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[interrupt]
fn OTG_FS() {
    usb_hid::interrupt();
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .require_pll48clk()
        .freeze();

    // USB stuff
    let gpioa = dp.GPIOA.split();

    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate_af10(),
        pin_dp: gpioa.pa12.into_alternate_af10(),
    };
    let usb_alloc = UsbBus::new(usb, unsafe { &mut EP_MEMORY });
    usb_hid::init(usb_alloc);
    unsafe {
        stm32::NVIC::unmask(stm32::Interrupt::OTG_FS);
    }

    // UART stuff
    let tx = gpioa.pa2.into_alternate_af7();
    let rx = gpioa.pa3.into_alternate_af7();

    let config = serial::config::Config::default().baudrate(115_200.bps());
    let usart = serial::Serial::usart2(dp.USART2, (tx, rx), config, clocks).unwrap();

    let (mut tx, mut rx) = usart.split();

    let mut _buf = [0; 1024];
    let mut buf = Vec::new(&mut _buf);

    // Loop forever
    loop {
        let byte = block!(rx.read()).unwrap();
        buf.push(byte);
        tx.write(byte).unwrap(); // gives feedback to the sender
        if byte == b'\n' {
            if buf.as_str().unwrap() == "send\n" {
                usb_hid::send();
            }
            buf.clear();
        }
    }
}
