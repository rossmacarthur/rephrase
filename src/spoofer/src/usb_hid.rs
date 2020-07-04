//! Implement a USB device class to emulate a Nintendo Switch controller.
//!
//! Some useful links:
//! - https://www.usb.org/sites/default/files/hid1_11.pdf
//! - https://gist.github.com/ToadKing/b883a8ccfa26adcc6ba9905e75aeb4f2

use core::cell::RefCell;

use cortex_m::interrupt::{free, Mutex};
use stm32f4xx_hal::otg_fs::UsbBusType;
use usb_device::class_prelude::*;
use usb_device::prelude::*;

// A fake USB HID report.
pub struct Report([u8; 64]);

const REPORT: Report = Report::new();

/// Represents a USB human interface device.
pub struct Hid {
    /// The USB interface number.
    interface_number: InterfaceNumber,
    /// Traffic from the device to the host.
    endpoint: EndpointIn<'static, UsbBusType>,
}

/// Represents the USB port on the microcontroller.
pub struct Usb {
    /// Represents a USB human interface device.
    hid: Hid,
    /// The STM32F4 USB device.
    device: UsbDevice<'static, UsbBusType>,
}

static mut USB_ALLOC: Option<UsbBusAllocator<UsbBusType>> = None;
static USB: Mutex<RefCell<Option<Usb>>> = Mutex::new(RefCell::new(None));

impl Report {
    /// Create a new empty `Report`.
    const fn new() -> Self {
        Self([0; 64])
    }
}

impl AsRef<[u8]> for Report {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

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
        )?;
        writer.endpoint(&self.endpoint)?;
        Ok(())
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        use usb_device::control::*;

        let request = xfer.request();

        if let Request {
            request_type: RequestType::Standard,
            recipient: Recipient::Interface,
            request: Request::GET_DESCRIPTOR,
            index,
            ..
        } = request
        {
            if *index == u8::from(self.interface_number).into() {
                let (desc_ty, desc_index) = request.descriptor_type_index();
                // Report type is 0x22
                if desc_ty == 0x22 && desc_index == 0 {
                    xfer.accept_with_static(REPORT.as_ref()).ok();
                }
            }
        }
    }
}

impl Hid {
    /// Send a USB HID report.
    pub fn send_report(&mut self, report: Report) -> usb_device::Result<usize> {
        self.endpoint.write(report.as_ref())
    }
}

pub fn interrupt() {
    free(move |cs| {
        let mut borrow = USB.borrow(&cs).borrow_mut();
        let usb = &mut borrow.as_mut().unwrap();
        usb.device.poll(&mut [&mut usb.hid]);
    });
}

pub fn send() {
    free(move |cs| {
        let mut borrow = USB.borrow(&cs).borrow_mut();
        let usb = &mut borrow.as_mut().unwrap();
        usb.hid.send_report(Report::new()).ok();
    });
}
