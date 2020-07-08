use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use gamepad::{dualshock, joycon};

type Device = usb::Device<usb::GlobalContext>;
type DeviceHandle = usb::DeviceHandle<usb::GlobalContext>;

const VENDOR_ID: u16 = 0x54c;
const PRODUCT_ID: u16 = 0x9cc;
const USB_INTERFACE: u8 = 0x03;
const USB_ADDRESS: u8 = 0x84;

fn get_first_controller() -> Result<Device> {
    for device in usb::devices()?.iter() {
        let descriptor = device.device_descriptor()?;
        if descriptor.vendor_id() == VENDOR_ID && descriptor.product_id() == PRODUCT_ID {
            return Ok(device);
        }
    }
    Err(anyhow!("no controller found"))
}

fn read(handle: &DeviceHandle) -> Result<Option<Vec<u8>>> {
    let mut buf = vec![0; 64];
    let timeout = Duration::from_secs(1);
    match handle.read_interrupt(USB_ADDRESS, &mut buf, timeout) {
        Ok(64) => Ok(Some(buf)),
        Ok(count) => Err(anyhow!("unexpected number of bytes read: {}", count)),
        Err(usb::Error::Timeout) => Ok(None),
        Err(err) => Err(err).context("failed to read data"),
    }
}

fn main() -> Result<()> {
    // USB
    let device = get_first_controller()?;
    println!("Found DualShock 4 controller:\n  {:?}\n", device);
    let mut handle = device.open().context("failed to open device")?;
    handle.set_auto_detach_kernel_driver(true)?;
    handle
        .claim_interface(USB_INTERFACE)
        .context("failed to claim interface")?;

    // UART
    let mut port = serial::open("/dev/ttyUSB0").context("failed to open serial port")?;

    loop {
        thread::sleep(Duration::from_millis(100));
        let report = dualshock::from_bytes(&read(&handle)?.unwrap());
        let mut buf = joycon::into_input_report_bytes(report).to_vec();
        buf.push(b'\n');
        port.write(&buf)?;
    }
}
