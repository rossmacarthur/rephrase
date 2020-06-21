use bitflags::bitflags;

bitflags! {
    pub struct Buttons: u32 {
        const D_UP = 0x00001; // d-pad up
        const D_DOWN = 0x00002; // d-pad down
        const D_LEFT = 0x00004; // d-pad left
        const D_RIGHT = 0x00008; // d-pad right
        const SELECT = 0x00010; // - on Nintendo devices, Share on DS4
        const START = 0x00020; // + on Nintendo devices, Options on DS4
        const LEFT_STICK = 0x00040; // left-stick click on Nintendo devices, L3 on DS4
        const RIGHT_STICK = 0x00080; // right-stick click on Nintendo devices, R3 on DS4
        const LEFT = 0x00100; // L on Nintendo devices, L1 on DS4
        const RIGHT = 0x00200; // R on Nintendo devices, R1 on DS4
        const LEFT_TRIGGER = 0x00400; // ZL on Nintendo devices, L2 on DS4
        const RIGHT_TRIGGER = 0x00800; // ZR on Nintendo devices, R2 on DS4
        const SOUTH = 0x01000; // the South face-button: B on Nintendo devices, ⨉ on DS4
        const EAST = 0x02000; // the East face-button: A on Nintendo devices, ○ on DS4
        const NORTH = 0x04000; // the North face-button: X on Nintendo devices, △ on DS4
        const WEST = 0x08000; // the West face-button: Y on Nintendo devices, □ on DS4
        const MODE = 0x10000; // Home on Nintendo devices, PS on DS4
        const CAPTURE = 0x20000; // Capture on Nintendo devices, touchpad click on DS4
        const LEFT_SIDE = 0x40000; // SL on Nintendo JoyCons
        const RIGHT_SIDE = 0x80000; // SR on Nintendo JoyCons
    }
}

#[derive(Debug)]
struct Stick {
    /// Values between -1.0 and 1.0 inclusive.
    x: f64,
    /// Values between -1.0 and 1.0 inclusive.
    y: f64,
}

/// Accelerometer state. X, Y, and Z axis in g (g-force).
struct Accelerometer {
    x: f64,
    y: f64,
    z: f64,
}

/// Gyroscope state. Angular velocity in degrees per second.
struct Gyroscope {
    x: f64,
    y: f64,
    z: f64,
}

/// The current input state.
#[derive(Debug)]
struct InputState {
    /// The states of all the buttons.
    buttons: Buttons,
    /// The value of the left trigger: value between 0.0 and 1.0.
    left_trigger: f64,
    /// The value of the right trigger: value between 0.0 and 1.0.
    right_trigger: f64,
    /// The right joy stick analog values.
    left_stick: Stick,
    /// The left joy stick analog values.
    right_stick: Stick,
}

/// The current Inertial measurement unit (IMU) state.
#[derive(Debug)]
struct InertialState {
    /// Accelerometer state.
    accel: Accelerometer,
    /// Gyroscope state.
    gyro: Gyroscope,
}
