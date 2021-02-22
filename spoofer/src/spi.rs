use core::cmp::min;

use crate::vec::Vec;

const ADDRESS_SERIAL_NUMBER: u16 = 0x6000;
const ADDRESS_CONTROLLER_COLOR: u16 = 0x6050;
const ADDRESS_FACTORY_PARAMETERS_1: u16 = 0x6080;
const ADDRESS_FACTORY_PARAMETERS_2: u16 = 0x6098;
const ADDRESS_FACTORY_CALIBRATION_1: u16 = 0x6020;
const ADDRESS_FACTORY_CALIBRATION_2: u16 = 0x603D;
const ADDRESS_STICKS_CALIBRATION: u16 = 0x8010;
const ADDRESS_IMU_CALIBRATION: u16 = 0x8028;

pub fn read(address: u16, buf: &mut Vec<u8>, size: usize) {
    buf.push((address & 0xff) as u8);
    buf.push(((address >> 8) & 0xff) as u8);
    buf.push(0x00);
    buf.push(0x00);
    buf.push(size as u8);

    for i in 0..size {
        buf.fake_push(0xff);
    }

    match address {
        ADDRESS_SERIAL_NUMBER => {
            // all 0xff
        }
        ADDRESS_CONTROLLER_COLOR => {
            if size >= 3 {
                buf.push_slice(&[0x0A, 0xB9, 0xE6]);
            }
            if size >= 6 {
                buf.push_slice(&[0xDD, 0xDD, 0xDD]);
            }
            if size >= 9 {
                buf.push_slice(&[0xAA, 0xAA, 0xAA]);
            }
            if size >= 12 {
                buf.push_slice(&[0xAA, 0xAA, 0xAA]);
            }
        }
        ADDRESS_FACTORY_PARAMETERS_1 => {
            let params = [
                0x50, 0xfd, 0x00, 0x00, 0xc6, 0x0f, 0x0f, 0x30, 0x61, 0x96, 0x30, 0xf3, 0xd4, 0x14,
                0x54, 0x41, 0x15, 0x54, 0xc7, 0x79, 0x9c, 0x33, 0x36, 0x63,
            ];
            buf.push_slice(&params[..min(params.len(), size)])
        }
        ADDRESS_FACTORY_PARAMETERS_2 => {
            let params = [
                0x0f, 0x30, 0x61, 0x96, 0x30, 0xf3, 0xd4, 0x14, 0x54, 0x41, 0x15, 0x54, 0xc7, 0x79,
                0x9c, 0x33, 0x36, 0x63,
            ];
            buf.push_slice(&params[..min(params.len(), size)])
        }
        ADDRESS_FACTORY_CALIBRATION_1 => {
            let calibration = [
                0xE6, 0xFF, 0x3A, 0x00, 0x39, 0x00, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0xF7, 0xFF,
                0xFC, 0xFF, 0x00, 0x00, 0xE7, 0x3B, 0xE7, 0x3B, 0xE7, 0x3B,
            ];
            buf.push_slice(&calibration[..min(calibration.len(), size)])
        }
        ADDRESS_FACTORY_CALIBRATION_2 => {
            let calibration = [
                0xba, 0x15, 0x62, 0x11, 0xb8, 0x7f, 0x29, 0x06, 0x5b, 0xff, 0xe7, 0x7e, 0x0e, 0x36,
                0x56, 0x9e, 0x85, 0x60, 0xff, 0x32, 0x32, 0x32, 0xff, 0xff, 0xff,
            ];
            buf.push_slice(&calibration[..min(calibration.len(), size)])
        }
        ADDRESS_STICKS_CALIBRATION => {
            if size > 22 {
                buf.inner[22] = 0xb2;
            }
            if size > 23 {
                buf.inner[23] = 0xa1;
            }
        }
        ADDRESS_IMU_CALIBRATION => {
            let calibration = [
                0xbe, 0xff, 0x3e, 0x00, 0xf0, 0x01, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0xfe, 0xff,
                0xfe, 0xff, 0x08, 0x00, 0xe7, 0x3b, 0xe7, 0x3b, 0xe7, 0x3b,
            ];
            buf.push_slice(&calibration[..min(calibration.len(), size)])
        }
        _ => {}
    }
}
