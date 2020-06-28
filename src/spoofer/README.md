# rephrase-spoofer

Emulates a Nintendo Switch controller.

## Development

Start OpenOCD to connect it to the STM32F4DISCOVERY and leave this running in
another terminal.

```
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg
```

Run the following to build this crate and open GDB where it will automatically
connect to OpenOCD and load the binary (see `openocd.gdb`).

```
cargo run
```

You can then debug the code using GDB.

## USB Descriptor

See https://gist.github.com/ToadKing/b883a8ccfa26adcc6ba9905e75aeb4f2

### Device descriptor

| Offset | Field              | Size | Value  | Description                   |
| ------ | ------------------ | ---- | ------ | ----------------------------- |
| 0      | bLength            | 1    | 0x12   |                               |
| 1      | bDescriptorType    | 1    | 0x01   | Device                        |
| 2      | bcdUSB             | 2    | 0x0200 | USB Spec 2.0                  |
| 4      | bDeviceClass       | 1    | 0x00   | Class info in Ifc Descriptors |
| 5      | bDeviceSubClass    | 1    | 0x00   |                               |
| 6      | bDeviceProtocol    | 1    | 0x00   |                               |
| 7      | bMaxPacketSize0    | 1    | 0x40   | 64 bytes                      |
| 8      | idVendor           | 2    | 0x057e | Nintendo Co., Ltd             |
| 10     | idProduct          | 2    | 0x2009 |                               |
| 12     | bcdDevice          | 2    | 0x0200 | 2.00                          |
| 14     | iManufacturer      | 1    | 0x01   | "Nintendo Co., Ltd."          |
| 15     | iProduct           | 1    | 0x02   | "Pro Controller"              |
| 16     | iSerialNumber      | 1    | 0x03   | "000000000001"                |
| 17     | bNumConfigurations | 1    | 0x01   |                               |

### Configuration descriptor

| Offset | Field               | Size | Value  | Description                |
| ------ | ------------------- | ---- | ------ | -------------------------- |
| 0      | bLength             | 1    | 0x09   |                            |
| 1      | bDescriptorType     | 1    | 0x02   | Configuration              |
| 2      | wTotalLength        | 2    | 0x0029 |                            |
| 4      | bNumInterfaces      | 1    | 0x01   |                            |
| 5      | bConfigurationValue | 1    | 0x01   |                            |
| 6      | iConfiguration      | 1    | 0x00   |                            |
| 7      | bmAttributes        | 1    | 0xa0   | Bus Powered, Remote Wakeup |
| 8      | bMaxPower           | 1    | 0xfa   | 500 mA                     |

### Interface descriptor

| Offset | Field             | Size | Value  | Description                  |
| ------ | ----------------- | ---- | ------ | ---------------------------- |
| 0      | bLength           | 1    | 0x09   |                              |
| 1      | bDescriptorType   | 1    | 0x21   | Human Interface Device (HID) |
| 2      | bcdHID            | 2    | 0x0111 | Version 1.11                 |
| 4      | bCountryCode      | 1    | 0x00   |                              |
| 5      | bNumDescriptors   | 1    | 0x01   |                              |
| 6      | bDescriptorType   | 1    | 0x22   | Report                       |
| 7      | wDescriptorLength | 2    | 0x00CB | 203 bytes                    |

### Endpoint Descriptor (In), Interrupt, 8 ms

| Offset | Field            | Size | Value | Description |
| ------ | ---------------- | ---- | ----- | ----------- |
| 0      | bLength          | 1    | 07h   |             |
| 1      | bDescriptorType  | 1    | 05h   | Endpoint    |
| 2      | bEndpointAddress | 1    | 81h   | 1 In        |
| 3      | bmAttributes     | 1    | 03h   | Interrupt   |
| 4      | wMaxPacketSize   | 2    | 0040h | 64 bytes    |
| 6      | bInterval        | 1    | 08h   | 8 ms        |

### Endpoint Descriptor (Out), Interrupt, 8 ms

| Offset | Field            | Size | Value | Description |
| ------ | ---------------- | ---- | ----- | ----------- |
| 0      | bLength          | 1    | 07h   |             |
| 1      | bDescriptorType  | 1    | 05h   | Endpoint    |
| 2      | bEndpointAddress | 1    | 01h   | 1 Out       |
| 3      | bmAttributes     | 1    | 03h   | Interrupt   |
| 4      | wMaxPacketSize   | 2    | 0040h | 64 bytes    |
| 6      | bInterval        | 1    | 08h   | 8 ms        |

### HID Report


```c
0x05, 0x01,        // Usage Page (Generic Desktop Ctrls)
0x15, 0x00,        // Logical Minimum (0)
0x09, 0x04,        // Usage (Joystick)
0xA1, 0x01,        // Collection (Application)
0x85, 0x30,        //   Report ID (48)
0x05, 0x01,        //   Usage Page (Generic Desktop Ctrls)
0x05, 0x09,        //   Usage Page (Button)
0x19, 0x01,        //   Usage Minimum (0x01)
0x29, 0x0A,        //   Usage Maximum (0x0A)
0x15, 0x00,        //   Logical Minimum (0)
0x25, 0x01,        //   Logical Maximum (1)
0x75, 0x01,        //   Report Size (1)
0x95, 0x0A,        //   Report Count (10)
0x55, 0x00,        //   Unit Exponent (0)
0x65, 0x00,        //   Unit (None)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x05, 0x09,        //   Usage Page (Button)
0x19, 0x0B,        //   Usage Minimum (0x0B)
0x29, 0x0E,        //   Usage Maximum (0x0E)
0x15, 0x00,        //   Logical Minimum (0)
0x25, 0x01,        //   Logical Maximum (1)
0x75, 0x01,        //   Report Size (1)
0x95, 0x04,        //   Report Count (4)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x75, 0x01,        //   Report Size (1)
0x95, 0x02,        //   Report Count (2)
0x81, 0x03,        //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x0B, 0x01, 0x00, 0x01, 0x00,  //   Usage (0x010001)
0xA1, 0x00,        //   Collection (Physical)
0x0B, 0x30, 0x00, 0x01, 0x00,  //     Usage (0x010030)
0x0B, 0x31, 0x00, 0x01, 0x00,  //     Usage (0x010031)
0x0B, 0x32, 0x00, 0x01, 0x00,  //     Usage (0x010032)
0x0B, 0x35, 0x00, 0x01, 0x00,  //     Usage (0x010035)
0x15, 0x00,        //     Logical Minimum (0)
0x27, 0xFF, 0xFF, 0x00, 0x00,  //     Logical Maximum (65534)
0x75, 0x10,        //     Report Size (16)
0x95, 0x04,        //     Report Count (4)
0x81, 0x02,        //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0xC0,              //   End Collection
0x0B, 0x39, 0x00, 0x01, 0x00,  //   Usage (0x010039)
0x15, 0x00,        //   Logical Minimum (0)
0x25, 0x07,        //   Logical Maximum (7)
0x35, 0x00,        //   Physical Minimum (0)
0x46, 0x3B, 0x01,  //   Physical Maximum (315)
0x65, 0x14,        //   Unit (System: English Rotation, Length: Centimeter)
0x75, 0x04,        //   Report Size (4)
0x95, 0x01,        //   Report Count (1)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x05, 0x09,        //   Usage Page (Button)
0x19, 0x0F,        //   Usage Minimum (0x0F)
0x29, 0x12,        //   Usage Maximum (0x12)
0x15, 0x00,        //   Logical Minimum (0)
0x25, 0x01,        //   Logical Maximum (1)
0x75, 0x01,        //   Report Size (1)
0x95, 0x04,        //   Report Count (4)
0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x75, 0x08,        //   Report Size (8)
0x95, 0x34,        //   Report Count (52)
0x81, 0x03,        //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x06, 0x00, 0xFF,  //   Usage Page (Vendor Defined 0xFF00)
0x85, 0x21,        //   Report ID (33)
0x09, 0x01,        //   Usage (0x01)
0x75, 0x08,        //   Report Size (8)
0x95, 0x3F,        //   Report Count (63)
0x81, 0x03,        //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x85, 0x81,        //   Report ID (-127)
0x09, 0x02,        //   Usage (0x02)
0x75, 0x08,        //   Report Size (8)
0x95, 0x3F,        //   Report Count (63)
0x81, 0x03,        //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
0x85, 0x01,        //   Report ID (1)
0x09, 0x03,        //   Usage (0x03)
0x75, 0x08,        //   Report Size (8)
0x95, 0x3F,        //   Report Count (63)
0x91, 0x83,        //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
0x85, 0x10,        //   Report ID (16)
0x09, 0x04,        //   Usage (0x04)
0x75, 0x08,        //   Report Size (8)
0x95, 0x3F,        //   Report Count (63)
0x91, 0x83,        //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
0x85, 0x80,        //   Report ID (-128)
0x09, 0x05,        //   Usage (0x05)
0x75, 0x08,        //   Report Size (8)
0x95, 0x3F,        //   Report Count (63)
0x91, 0x83,        //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
0x85, 0x82,        //   Report ID (-126)
0x09, 0x06,        //   Usage (0x06)
0x75, 0x08,        //   Report Size (8)
0x95, 0x3F,        //   Report Count (63)
0x91, 0x83,        //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Volatile)
0xC0,              // End Collection
```
