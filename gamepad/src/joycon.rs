//! Nintendo Switch Pro controller.

use crate::*;

/// Convert the `Report` into Joycon specific bytes.
pub fn into_input_report_bytes(report: Report) -> [u8; 13] {
    let Report {
        counter,
        status:
            Status {
                buttons,
                left_stick,
                right_stick,
                ..
            },
        ..
    } = report;

    let mut buf = [0; 13];

    buf[0] = 0x3f;
    buf[1] = counter.unwrap_or(0);
    buf[2] = 0x80 & 0x01; // high nibble battery, low nibble connection info

    // Digital buttons
    let bit_flags: [(usize, u8, Buttons); 12] = [
        (3, 0x01, Buttons::WEST),
        (3, 0x02, Buttons::NORTH),
        (3, 0x04, Buttons::SOUTH),
        (3, 0x08, Buttons::EAST),
        // (3, 0x10, Buttons::SR),
        // (3, 0x20, Buttons::SL),
        (3, 0x40, Buttons::R1),
        (3, 0x80, Buttons::R2),
        (4, 0x01, Buttons::MINUS),
        (4, 0x02, Buttons::PLUS),
        (4, 0x04, Buttons::R3),
        (4, 0x08, Buttons::L3),
        (4, 0x10, Buttons::HOME),
        (4, 0x20, Buttons::CAPTURE),
    ];

    for (byte, bit, button) in bit_flags.iter() {
        if buttons.contains(*button) {
            buf[*byte] |= bit;
        }
    }

    // Analog sticks
    buf[6] = left_stick.x;
    buf[7] = left_stick.y << 4;
    buf[8] = left_stick.y >> 4;

    buf[9] = right_stick.x;
    buf[10] = right_stick.y << 4;
    buf[11] = right_stick.y >> 4;

    buf
}
