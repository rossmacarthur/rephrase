//! PlayStation Dualshock 4 controller.

use std::convert::TryInto;

use crate::model::*;

/////////////////////////////////////////////////////////////////////////
// Utility traits
/////////////////////////////////////////////////////////////////////////

trait IsBitSet {
    fn is_bit_set(&self, bit: u8) -> bool;
}

impl IsBitSet for u8 {
    fn is_bit_set(&self, bit: u8) -> bool {
        (self & (0b1 << bit)) > 0
    }
}

trait FromLeSlice {
    fn from_le_slice(bytes: &[u8]) -> Self;
}

impl FromLeSlice for i16 {
    fn from_le_slice(bytes: &[u8]) -> Self {
        Self::from_le_bytes(bytes.try_into().expect("slice of length 2"))
    }
}

/////////////////////////////////////////////////////////////////////////
// Report implementation
/////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Report {
    id: u8,
    counter: u8,
    state: State,
}

impl Report {
    pub fn from_bytes(buf: &[u8]) -> Self {
        // https://www.psdevwiki.com/ps4/DS4-USB
        // https://github.com/chrippa/ds4drv/blob/be7327fc3f5abb8717815f2a1a2ad3d335535d8a/ds4drv/device.py#L150-L211
        // https://github.com/JibbSmart/JoyShockLibrary/blob/959d41b7339421d5a135d43a112c27138e33f2ff/JoyShockLibrary/InputHelpers.cpp

        let id = buf[0];
        let mut buttons = Buttons::empty();

        let left_stick = Stick::new(buf[1], buf[2]);
        let right_stick = Stick::new(buf[3], buf[4]);

        buttons |= match buf[5] & 0x0f {
            0b0000 => Buttons::UP,
            0b0001 => Buttons::UP | Buttons::RIGHT,
            0b0010 => Buttons::RIGHT,
            0b0011 => Buttons::RIGHT | Buttons::DOWN,
            0b0100 => Buttons::DOWN,
            0b0101 => Buttons::DOWN | Buttons::LEFT,
            0b0110 => Buttons::LEFT,
            0b0111 => Buttons::LEFT | Buttons::UP,
            0b1000 => Buttons::empty(),
            _ => unreachable!(),
        };

        let bit_flags: &[(usize, u8, Buttons)] = &[
            // Byte 5
            (5, 4, Buttons::WEST),
            (5, 5, Buttons::SOUTH),
            (5, 6, Buttons::EAST),
            (5, 7, Buttons::NORTH),
            // Byte 6
            (6, 0, Buttons::L1),
            (6, 1, Buttons::R1),
            (6, 2, Buttons::L2),
            (6, 3, Buttons::R2),
            (6, 4, Buttons::SHARE),
            (6, 5, Buttons::OPTIONS),
            (6, 6, Buttons::L3),
            (6, 7, Buttons::R3),
            // Byte 7
            (7, 0, Buttons::PS),
            (7, 1, Buttons::TPAD),
        ];

        for (byte, bit, button) in bit_flags {
            // simply check's if the bit `bit` is set
            if (buf[*byte] & (0b1 << bit)) > 0 {
                buttons |= *button;
            }
        }

        let counter = buf[7] >> 2;
        let left_trigger = buf[8];
        let right_trigger = buf[9];

        let acceleration = Acceleration::new(
            i16::from_le_slice(&buf[13..=14]),
            i16::from_le_slice(&buf[15..=16]),
            i16::from_le_slice(&buf[17..=18]),
        );

        let orientation = Orientation::new(
            i16::from_le_slice(&buf[19..=20]),
            i16::from_le_slice(&buf[21..=22]),
            i16::from_le_slice(&buf[23..=24]),
        );

        let state = State {
            buttons,
            left_stick,
            right_stick,
            left_trigger,
            right_trigger,
            acceleration,
            orientation,
        };

        // TODO: extract more information

        Self { id, counter, state }
    }
}
