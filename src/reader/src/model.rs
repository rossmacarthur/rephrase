use bitflags::bitflags;

bitflags! {
    pub struct Buttons: u32 {
        const UP = 0x00001; // d-pad up
        const DOWN = 0x00002; // d-pad down
        const LEFT = 0x00004; // d-pad left
        const RIGHT = 0x00008; // d-pad right

        const SHARE = 0x00010; // - on Nintendo devices, Share on DS4
        const OPTIONS = 0x00020; // + on Nintendo devices, Options on DS4

        const L1 = 0x00040; // L on Nintendo devices, L1 on DS4
        const R1 = 0x00080; // R on Nintendo devices, R1 on DS4

        const L2 = 0x00100; // ZL on Nintendo devices, L2 on DS4
        const R2 = 0x00200; // ZR on Nintendo devices, R2 on DS4

        const L3 = 0x00400; // left-stick click on Nintendo devices, L3 on DS4
        const R3 = 0x00800; // right-stick click on Nintendo devices, R3 on DS4

        const WEST = 0x01000; // the West action button: Y on Nintendo devices, □ on DS4
        const SOUTH = 0x02000; // the South action button: B on Nintendo devices, ⨉ on DS4
        const EAST = 0x04000; // the East action button: A on Nintendo devices, ○ on DS4
        const NORTH = 0x08000; // the North action button: X on Nintendo devices, △ on DS4

        const PS = 0x10000; // Home on Nintendo devices, PS on DS4
        const TPAD = 0x20000; // Capture on Nintendo devices, touchpad click on DS4
    }
}

#[derive(Debug)]
pub struct Stick {
    x: u8,
    y: u8,
}

/// Accelerometer state. X, Y, and Z axis in g (g-force).
#[derive(Debug)]
pub struct Acceleration {
    x: i16,
    y: i16,
    z: i16,
}

/// Gyroscope state. Angular velocity in degrees per second.
#[derive(Debug)]
pub struct Orientation {
    x: i16,
    y: i16,
    z: i16,
}

/// The current state.
#[derive(Debug)]
pub struct State {
    /// The states of all the buttons.
    pub(crate) buttons: Buttons,
    /// The right joy stick analog values.
    pub(crate) left_stick: Stick,
    /// The left joy stick analog values.
    pub(crate) right_stick: Stick,
    /// The value of the left trigger: value between 0.0 and 1.0.
    pub(crate) left_trigger: u8,
    /// The value of the right trigger: value between 0.0 and 1.0.
    pub(crate) right_trigger: u8,
    /// Accelerometer state.
    pub(crate) acceleration: Acceleration,
    /// Gyroscope state.
    pub(crate) orientation: Orientation,
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
