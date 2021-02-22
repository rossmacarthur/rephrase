#![no_main]
#![no_std]

mod response;
mod spi;
mod usb_hid;
mod vec;

use core::cell::{Cell, RefCell};
use core::fmt::{self, Write};

use cortex_m::interrupt::{free, CriticalSection, Mutex};
use cortex_m_rt::entry;
use lazy_static::lazy_static;
use panic_semihosting as _;
use stm32f4xx_hal::gpio::{self, ExtiPin, Input, Output, PullDown, PushPull};
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::{self, interrupt};
use stm32f4xx_hal::{block, delay, serial};

use vec::Vec;

static BTN: Mutex<RefCell<Option<gpio::gpioa::PA0<Input<PullDown>>>>> =
    Mutex::new(RefCell::new(None));

static LD3: Mutex<RefCell<Option<gpio::gpiod::PD13<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));

static UART_TX: Mutex<RefCell<Option<serial::Tx<stm32::USART2>>>> = Mutex::new(RefCell::new(None));

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

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
    let mut dp = stm32::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .require_pll48clk()
        .freeze();

    let mut delay = delay::Delay::new(cp.SYST, clocks);

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

    // Button stuff
    let mut btn = gpioa.pa0.into_pull_down_input();
    btn.make_interrupt_source(&mut dp.SYSCFG);
    btn.enable_interrupt(&mut dp.EXTI);
    btn.trigger_on_edge(&mut dp.EXTI, gpio::Edge::RISING_FALLING);

    // LED stuff
    let gpiod = dp.GPIOD.split();
    let ld3 = gpiod.pd13.into_push_pull_output();

    // Store these so they are available to the interrupt.
    free(move |cs| {
        BTN.borrow(&cs).replace(Some(btn));
        LD3.borrow(&cs).replace(Some(ld3));
    });

    let tx = gpioa.pa2.into_alternate_af7();
    let rx = gpioa.pa3.into_alternate_af7();
    let config = serial::config::Config::default().baudrate(115_200.bps());
    let usart = serial::Serial::usart2(dp.USART2, (tx, rx), config, clocks).unwrap();

    let (mut tx, mut rx) = usart.split();
    writeln!(tx, "\n=== spoofer started ===").unwrap();
    free(move |cs| {
        UART_TX.borrow(&cs).replace(Some(tx));
    });

    // Enable the interrupts!
    unsafe { stm32::NVIC::unmask(stm32::Interrupt::EXTI0) };
    unsafe {
        stm32::NVIC::unmask(stm32::Interrupt::OTG_FS);
    }

    loop {
        // crate::usb_hid::send();
        // delay.delay_ms(100u32);
    }
}

#[interrupt]
fn EXTI0() {
    free(|cs| {
        // get the button
        let mut borrow = BTN.borrow(&cs).borrow_mut();
        let btn = borrow.as_mut().unwrap();

        // toggle LD3
        let mut borrow = LD3.borrow(&cs).borrow_mut();
        let ld3 = borrow.as_mut().unwrap();
        ld3.toggle().unwrap();

        // toggle report butons
        crate::response::report_toggle();

        // finally clear the interrupt bit
        btn.clear_interrupt_pending_bit();
    });
}
