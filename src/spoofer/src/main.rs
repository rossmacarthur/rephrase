#![no_main]
#![no_std]

mod vec;

use core::fmt::Write;

use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f4xx_hal::nb;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::serial;
use stm32f4xx_hal::stm32;
use usb_device::prelude::*;

use vec::Vec;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

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
    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });
    let mut usb_serial = usbd_serial::SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x057e, 0x2009))
        .manufacturer("Nintendo Co., Ltd.")
        .product("Pro Controller")
        .serial_number("000000000001")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

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
        usb_dev.poll(&mut [&mut usb_serial]);

        match rx.read() {
            Err(nb::Error::WouldBlock) => {}
            result => {
                let byte = result.unwrap();
                buf.push(byte);
                tx.write(byte).unwrap();
                if byte == b'\n' {
                    writeln!(tx, "echo: {}", buf.as_str().unwrap()).unwrap();
                    buf.clear();
                }
            }
        }
    }
}
