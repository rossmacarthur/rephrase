//! Implement a USB device class to emulate a Nintendo Switch controller.
//!
//! Some useful links:
//! - https://www.usb.org/defined-class-codes
//! - https://www.usb.org/sites/default/files/hid1_11.pdf
//! - https://gist.github.com/ToadKing/b883a8ccfa26adcc6ba9905e75aeb4f2

use core::cell::RefCell;

use cortex_m::interrupt::{Mutex, free};
use stm32f4xx_hal::otg_fs::{self, UsbBusType};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32;
use usb_device::class_prelude::*;
use usb_device::prelude::*;


static mut USB_ALLOC: Option<UsbBusAllocator<UsbBusType>> = None;
static USB: Mutex<RefCell<Option<Usb>>> = Mutex::new(RefCell::new(None));

pub fn init(usb_alloc: UsbBusAllocator<UsbBusType>) {
    free(move |cs| {
        let usb_alloc = unsafe {
            USB_ALLOC = Some(usb_alloc);
            USB_ALLOC.as_ref().unwrap()
        };
        let hid = Hid {
            interface_number: usb_alloc.interface(),
            endpoint: usb_alloc.interrupt(64, 8),
        };
        let device = UsbDeviceBuilder::new(usb_alloc, UsbVidPid(0x057e, 0x2009))
            .device_class(0x00)
            .device_sub_class(0x00)
            .device_protocol(0x00)
            .max_packet_size_0(64)
            .manufacturer("Nintendo Co., Ltd")
            .product("Pro Controller")
            .serial_number("000000000001")
            .build();
        let usb = Usb { device, hid };
        USB.borrow(&cs).replace(Some(usb));
    });
}

/// Represents the USB port on the microcontroller.
pub struct Usb {
    hid: Hid,
    device: UsbDevice<'static, UsbBusType>,
}

/// Represents a USB human interface device.
pub struct Hid {
    /// The USB interface number.
    interface_number: InterfaceNumber,
    /// Traffic from the device to the host.
    endpoint: EndpointIn<'static, UsbBusType>,
}

impl<B> UsbClass<B> for Hid
where
    B: UsbBus,
{
    fn get_configuration_descriptors(
        &self,
        writer: &mut DescriptorWriter,
    ) -> usb_device::Result<()> {
        writer.interface(
            self.interface_number,
            0x03, // bInterfaceClass
            0x00, // bInterfaceSubClass
            0x00, // bInterfaceProtocol
        )?;

        writer.write(
            0x21, // bDescriptorType, Human Interface Device (HID)
            &[
                0x11, 0x01, // bcdHID, Version 1.11
                0x00, // bCountryCode
                0x01, // bNumDescriptors
                0x22, // bDescriptorType, HID Report
                0x40, 0x00, // wMaxPacketSize, 64 bytes
            ],
        )
    }
}

// https://github.com/agalakhov/usbd-hid-device/blob/master/src/hidclass.rs
// https://github.com/agalakhov/usbd-hid-device/blob/master/src/lib.rs
// https://github.com/agalakhov/usbd-hid-device-example/blob/master/src/main.rs
// https://github.com/agalakhov/usbd-hid-device-example/blob/master/src/report.rs
