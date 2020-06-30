//! Implement a USB device class to emulate a Nintendo Switch controller.
//!
//! Some useful links:
//! - https://www.usb.org/defined-class-codes
//! - https://www.usb.org/sites/default/files/hid1_11.pdf
//! - https://gist.github.com/ToadKing/b883a8ccfa26adcc6ba9905e75aeb4f2

use stm32f4xx_hal::otg_fs::{self, UsbBusType};
use stm32f4xx_hal::prelude::*;
use usb_device::class_prelude::*;
use usb_device::prelude::*;

pub struct Usb {
    device: UsbDevice<'static, UsbBusType>,
    hid: Hid,
}

/// Represents a USB human interface device.
pub struct Hid {
    /// The USB interface number.
    interface_number: InterfaceNumber,
    /// Traffic from the device to the host.
    endpoint: EndpointIn<'static, UsbBusType>,
}

impl Usb {
    pub fn new(usb_peripherals: otg_fs::USB) -> Self {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS_ALLOCATOR: Option<UsbBusAllocator<UsbBusType>> = None;

        let usb_bus_alloc = unsafe {
            USB_BUS_ALLOCATOR = Some(otg_fs::UsbBus::new(usb_peripherals, &mut EP_MEMORY));
            USB_BUS_ALLOCATOR.as_ref().expect("yikes")
        };

        let device = UsbDeviceBuilder::new(&usb_bus_alloc, UsbVidPid(0x057e, 0x2009))
            .device_class(0x00)
            .device_sub_class(0x00)
            .device_protocol(0x00)
            .max_packet_size_0(64)
            .manufacturer("Nintendo Co., Ltd")
            .product("Pro Controller")
            .serial_number("000000000001")
            .build();

        let hid = Hid {
            interface_number: usb_bus_alloc.interface(),
            endpoint: usb_bus_alloc.interrupt(64, 8),
        };

        Self { device, hid }
    }
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
