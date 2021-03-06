#![no_std]

#[cfg(feature = "dualshock")]
pub mod dualshock;
#[cfg(feature = "joycon")]
pub mod joycon;

use bitflags::bitflags;

/////////////////////////////////////////////////////////////////////////
// Definitions
/////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Represents the set of currently pushed buttons.
    #[derive(Default)]
    pub struct Buttons: u32 {
        /// D-pad up
        const UP = 0x00001;
        /// D-pad down
        const DOWN = 0x00002;
        /// D-pad left
        const LEFT = 0x00004;
        /// D-pad right
        const RIGHT = 0x00008;
        /// Minus (-) on Joycon, Share on DS4
        const MINUS = 0x00010;
        /// Plus (+) on Joycon, Options on DS4
        const PLUS = 0x00020;
        /// L on Joycon, L1 on DS4
        const L1 = 0x00040;
        /// R on Joycon, R1 on DS4
        const R1 = 0x00080;
        /// ZL on Joycon, L2 on DS4
        const L2 = 0x00100;
        /// ZR on Joycon, R2 on DS4
        const R2 = 0x00200;
        /// Left-stick click on Joycon, L3 on DS4
        const L3 = 0x00400;
        /// Right-stick click on Joycon, R3 on DS4
        const R3 = 0x00800;
        /// Y on Joycon, ▢ on DS4
        const WEST = 0x01000;
        /// B on Joycon, X on DS4
        const SOUTH = 0x02000;
        /// A on Joycon, O on DS4
        const EAST = 0x04000;
        /// X on Joycon, △ on DS4
        const NORTH = 0x08000;
        /// ⌂ on Joycon, PS on DS4
        const HOME = 0x10000;
        /// O on Joycon, touchpad click on DS4
        const CAPTURE = 0x20000;
    }
}

/// Joy stick state.
#[derive(Debug)]
pub struct Stick {
    x: u8,
    y: u8,
}

/// Accelerometer state. X, Y, and Z axis in g (g-force).
#[derive(Debug, Default)]
pub struct Acceleration {
    x: i16,
    y: i16,
    z: i16,
}

/// Gyroscope state. Angular velocity X, Y, and Z axis in degrees per second.
#[derive(Debug, Default)]
pub struct Orientation {
    x: i16,
    y: i16,
    z: i16,
}

/// The current state.
#[derive(Debug)]
pub struct Status {
    /// The states of all the buttons.
    buttons: Buttons,
    /// The right joy stick analog values.
    left_stick: Stick,
    /// The left joy stick analog values.
    right_stick: Stick,
    /// The value of the left trigger: value between 0 and 255.
    left_trigger: u8,
    /// The value of the right trigger: value between 0 and 255.
    right_trigger: u8,
    /// Accelerometer state.
    acceleration: Acceleration,
    /// Gyroscope state.
    orientation: Orientation,
}

/// A generic controller HID `Report`.
#[derive(Debug, Default)]
pub struct Report {
    /// Counts up 1 per report.
    counter: Option<u8>,
    /// The state of the controller.
    status: Status,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

impl Default for Stick {
    fn default() -> Self {
        Self { x: 128, y: 128 }
    }
}

impl Stick {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Acceleration {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

impl Orientation {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self {
            buttons: Default::default(),
            left_stick: Default::default(),
            right_stick: Default::default(),
            left_trigger: 128,
            right_trigger: 128,
            acceleration: Default::default(),
            orientation: Default::default(),
        }
    }
}

impl Report {
    pub fn counter(&self) -> Option<u8> {
        self.counter
    }
}
