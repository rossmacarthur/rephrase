//! Implement a USB device class to emulate a Nintendo Switch controller.
//!
//! Some useful links:
//! - https://www.usb.org/sites/default/files/hid1_11.pdf
//! - https://www.usb.org/document-library/hid-usage-tables-112
//! - https://gist.github.com/ToadKing/b883a8ccfa26adcc6ba9905e75aeb4f2
//! - https://github.com/agalakhov/usbd-hid-device
//! - https://github.com/agalakhov/usbd-hid-device-example

use core::cell::RefCell;

use cortex_m::interrupt::{free, Mutex};
use stm32f4xx_hal::otg_fs::UsbBusType;
use usb_device::class_prelude::*;
use usb_device::prelude::*;

#[rustfmt::skip]
const HID_DESCRIPTOR: &[u8] = &[
    0x05, 0x01,                   // Usage Page (Generic Desktop Ctrls)
    0x15, 0x00,                   // Logical Minimum (0)
    0x09, 0x04,                   // Usage (Joystick)
    0xA1, 0x01,                   // Collection (Application)
    0x85, 0x30,                   //   Report ID (48)
    0x05, 0x01,                   //   Usage Page (Generic Desktop Ctrls)
    0x05, 0x09,                   //   Usage Page (Button)
    0x19, 0x01,                   //   Usage Minimum (0x01)
    0x29, 0x0A,                   //   Usage Maximum (0x0A)
    0x15, 0x00,                   //   Logical Minimum (0)
    0x25, 0x01,                   //   Logical Maximum (1)
    0x75, 0x01,                   //   Report Size (1)
    0x95, 0x0A,                   //   Report Count (10)
    0x55, 0x00,                   //   Unit Exponent (0)
    0x65, 0x00,                   //   Unit (None)
    0x81, 0x02,                   //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x09,                   //   Usage Page (Button)
    0x19, 0x0B,                   //   Usage Minimum (0x0B)
    0x29, 0x0E,                   //   Usage Maximum (0x0E)
    0x15, 0x00,                   //   Logical Minimum (0)
    0x25, 0x01,                   //   Logical Maximum (1)
    0x75, 0x01,                   //   Report Size (1)
    0x95, 0x04,                   //   Report Count (4)
    0x81, 0x02,                   //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x75, 0x01,                   //   Report Size (1)
    0x95, 0x02,                   //   Report Count (2)
    0x81, 0x03,                   //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x0B, 0x01, 0x00, 0x01, 0x00, //   Usage (0x010001)
    0xA1, 0x00,                   //   Collection (Physical)
    0x0B, 0x30, 0x00, 0x01, 0x00, //     Usage (0x010030)
    0x0B, 0x31, 0x00, 0x01, 0x00, //     Usage (0x010031)
    0x0B, 0x32, 0x00, 0x01, 0x00, //     Usage (0x010032)
    0x0B, 0x35, 0x00, 0x01, 0x00, //     Usage (0x010035)
    0x15, 0x00,                   //     Logical Minimum (0)
    0x27, 0xFF, 0xFF, 0x00, 0x00, //     Logical Maximum (65534)
    0x75, 0x10,                   //     Report Size (16)
    0x95, 0x04,                   //     Report Count (4)
    0x81, 0x02,                   //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0,                         //   End Collection
    0x0B, 0x39, 0x00, 0x01, 0x00, //   Usage (0x010039)
    0x15, 0x00,                   //   Logical Minimum (0)
    0x25, 0x07,                   //   Logical Maximum (7)
    0x35, 0x00,                   //   Physical Minimum (0)
    0x46, 0x3B, 0x01,             //   Physical Maximum (315)
    0x65, 0x14,                   //   Unit (System: English Rotation, Length: Centimeter)
    0x75, 0x04,                   //   Report Size (4)
    0x95, 0x01,                   //   Report Count (1)
    0x81, 0x02,                   //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x09,                   //   Usage Page (Button)
    0x19, 0x0F,                   //   Usage Minimum (0x0F)
    0x29, 0x12,                   //   Usage Maximum (0x12)
    0x15, 0x00,                   //   Logical Minimum (0)
    0x25, 0x01,                   //   Logical Maximum (1)
    0x75, 0x01,                   //   Report Size (1)
    0x95, 0x04,                   //   Report Count (4)
    0x81, 0x02,                   //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x75, 0x08,                   //   Report Size (8)
    0x95, 0x34,                   //   Report Count (52)
    0x81, 0x03,                   //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x06, 0x00, 0xFF,             //   Usage Page (Vendor Defined 0xFF00)
    0x85, 0x21,                   //   Report ID (33)
    0x09, 0x01,                   //   Usage (0x01)
    0x75, 0x08,                   //   Report Size (8)
    0x95, 0x3F,                   //   Report Count (63)
    0x81, 0x03,                   //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x85, 0x81,                   //   Report ID (-127)
    0x09, 0x02,                   //   Usage (0x02)
    0x75, 0x08,                   //   Report Size (8)
    0x95, 0x3F,                   //   Report Count (63)
    0x81, 0x03,                   //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x85, 0x01,                   //   Report ID (1)
    0x09, 0x03,                   //   Usage (0x03)
    0x75, 0x08,                   //   Report Size (8)
    0x95, 0x3F,                   //   Report Count (63)
    0x91, 0x83,                   //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
    0x85, 0x10,                   //   Report ID (16)
    0x09, 0x04,                   //   Usage (0x04)
    0x75, 0x08,                   //   Report Size (8)
    0x95, 0x3F,                   //   Report Count (63)
    0x91, 0x83,                   //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
    0x85, 0x80,                   //   Report ID (-128)
    0x09, 0x05,                   //   Usage (0x05)
    0x75, 0x08,                   //   Report Size (8)
    0x95, 0x3F,                   //   Report Count (63)
    0x91, 0x83,                   //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
    0x85, 0x82,                   //   Report ID (-126)
    0x09, 0x06,                   //   Usage (0x06)
    0x75, 0x08,                   //   Report Size (8)
    0x95, 0x3F,                   //   Report Count (63)
    0x91, 0x83,                   //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
    0xC0,                         // End Collection
];

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
        #[rustfmt::skip]
        writer.write(
            0x21, // bDescriptorType, Human Interface Device (HID)
            &[
                0x11, 0x01, // bcdHID, Version 1.11
                0x00, // bCountryCode
                0x01, // bNumDescriptors
                0x22, // bDescriptorType, HID Report
                HID_DESCRIPTOR.len() as u8, (HID_DESCRIPTOR.len() >> 8) as u8, // wDescriptorLength
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
                    xfer.accept_with_static(HID_DESCRIPTOR).ok();
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
