#![no_main]
#![no_std]

mod usb_hid;
mod vec;

use core::cell::RefCell;
use core::fmt::{self, Write};

use cortex_m::interrupt::{free, CriticalSection, Mutex};
use cortex_m_rt::entry;
use lazy_static::lazy_static;
use panic_semihosting as _;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::{self, interrupt};
use stm32f4xx_hal::{block, serial};

use vec::Vec;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];
static UART_TX: Mutex<RefCell<Option<serial::Tx<stm32::USART2>>>> = Mutex::new(RefCell::new(None));

fn send_over_uart(cs: &CriticalSection, args: fmt::Arguments<'_>) {
    let mut borrow = crate::UART_TX.borrow(cs).borrow_mut();
    if let Some(tx) = borrow.as_mut() {
        tx.write_fmt(args).unwrap()
    }
}

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
    writeln!(tx, "=== spoofer started ===").unwrap();
    free(move |cs| {
        UART_TX.borrow(&cs).replace(Some(tx));
    });
    // Loop forever
    // usb_hid::send();

    fn send_report(counter: u8) {
        let mut report = [0; 64];

        let header = [0x19, 0x01, 0x03, 0x07, 0x00, 0x00, 0x92, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        for (index, byte) in header.iter().enumerate() {
            report[index] = *byte;
        }

        report[13] = 0x30;
        report[14] = counter;
        report[15] = 0x81;

        report[16] = 0x40;
        report[17] = 0x00;
        report[18] = 0x40;

        usb_hid::send(&report);
    }

    let mut counter = 0;
    for _ in 0..40 {
        counter += 1;
        send_report(counter);
    }

    loop {
        send_report(counter);
    }
}
