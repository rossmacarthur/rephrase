// TODO: https://gist.github.com/mzyy94/60ae253a45e2759451789a117c59acf9#file-simulate_procon-py
use core::cell::{Cell, RefCell};
use core::mem;

use cortex_m::interrupt::{free, CriticalSection, Mutex};

static REPORT: Mutex<RefCell<Report>> = Mutex::new(RefCell::new(Report::new()));
static COUNTER: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));

const SUBCOMMAND_CONTROLLER_STATE_ONLY: u8 = 0x00;
const SUBCOMMAND_BLUETOOTH_MANUAL_PAIRING: u8 = 0x01;
const SUBCOMMAND_REQUEST_DEVICE_INFO: u8 = 0x02;
const SUBCOMMAND_SET_INPUT_REPORT_MODE: u8 = 0x03;
const SUBCOMMAND_TRIGGER_BUTTONS_ELAPSED_TIME: u8 = 0x04;
const SUBCOMMAND_GET_PAGE_LIST_STATE: u8 = 0x05;
const SUBCOMMAND_SET_HCI_STATE: u8 = 0x06;
const SUBCOMMAND_RESET_PAIRING_INFO: u8 = 0x07;
const SUBCOMMAND_SET_SHIPMENT_LOW_POWER_STATE: u8 = 0x08;
const SUBCOMMAND_SPI_FLASH_READ: u8 = 0x10;
const SUBCOMMAND_SPI_FLASH_WRITE: u8 = 0x11;
const SUBCOMMAND_SPI_SECTOR_ERASE: u8 = 0x12;
const SUBCOMMAND_RESET_NFC_IR_MCU: u8 = 0x20;
const SUBCOMMAND_SET_NFC_IR_MCU_CONFIG: u8 = 0x21;
const SUBCOMMAND_SET_NFC_IR_MCU_STATE: u8 = 0x22;
const SUBCOMMAND_SET_PLAYER_LIGHTS: u8 = 0x30;
const SUBCOMMAND_GET_PLAYER_LIGHTS: u8 = 0x31;
const SUBCOMMAND_SET_HOME_LIGHTS: u8 = 0x38;
const SUBCOMMAND_ENABLE_IMU: u8 = 0x40;
const SUBCOMMAND_SET_IMU_SENSITIVITY: u8 = 0x41;
const SUBCOMMAND_WRITE_IMU_REGISTERS: u8 = 0x42;
const SUBCOMMAND_READ_IMU_REGISTERS: u8 = 0x43;
const SUBCOMMAND_ENABLE_VIBRATION: u8 = 0x48;
const SUBCOMMAND_GET_REGULATED_VOLTAGE: u8 = 0x50;

fn count() -> u8 {
    free(|cs| {
        let counter = COUNTER.borrow(&cs);
        let counter = COUNTER.borrow(&cs);
        let value = counter.get();
        counter.set(value.wrapping_add(3));
        value
    })
}

pub fn increment_count() {
    free(|cs| {
        let counter = COUNTER.borrow(&cs);
        let value = counter.get();
        counter.set(value.wrapping_add(3));
    })
}

fn report() -> [u8; 11] {
    free(|cs| {
        let report = REPORT.borrow(&cs).borrow();
        report.to_array()
    })
}

pub fn report_toggle() {
    free(|cs| {
        let mut report = REPORT.borrow(&cs).borrow_mut();
        report.toggle_lr();
    })
}

// A Joycon report.
#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct Report {
    info: u8,
    buttons: [u8; 3],
    analog: [u8; 6],
    vibrator: u8,
}

pub struct Response {
    buf: [u8; 64],
    len: usize,
}

struct Reply<'a> {
    code: u8,
    command: u8,
    data: Option<&'a [u8]>,
}

struct UartReply<'a> {
    code: u8,
    subcommand: u8,
    data: Option<&'a [u8]>,
}

impl Report {
    /// Create a new empty `Report`.
    const fn new() -> Self {
        let lx: u16 = 0x0800;
        let ly: u16 = 0x0800;
        let rx: u16 = 0x0800;
        let ry: u16 = 0x0800;

        Self {
            info: 0b0000_0001 // Pro controller + USB connected
                    | 0b0001_0000 // Battery charging
                    | 0b1000_0000, // Battery full
            buttons: [0; 3], // All unset
            analog: [
                (ly & 0xFF) as u8,
                (((ly & 0x0F) << 4) | ((lx & 0xF00) >> 8)) as u8,
                ((ly & 0xFF0) >> 4) as u8,
                (rx & 0xFF) as u8,
                (((ry & 0x0F) << 4) | ((rx & 0xF00) >> 8)) as u8,
                ((ry & 0xFF0) >> 4) as u8,
            ],
            vibrator: 0x0C,
        }
    }

    fn toggle_lr(&mut self) {
        self.buttons[0] ^= 0x80;
        self.buttons[2] ^= 0x80;
    }

    fn to_array(&self) -> [u8; 11] {
        unsafe { mem::transmute_copy::<Self, [u8; 11]>(self) }
    }
}

impl<'a> Reply<'a> {
    fn new(code: u8, command: u8) -> Self {
        Self {
            code,
            command,
            data: None,
        }
    }

    fn data(mut self, data: &'a [u8]) -> Self {
        self.data = Some(data);
        self
    }

    fn build(self) -> Response {
        let mut buf = [0; 64];
        buf[0] = self.code;
        buf[1] = self.command;
        let len = 2 + self
            .data
            .map(|data| {
                buf[2..2 + data.len()].copy_from_slice(data);
                data.len()
            })
            .unwrap_or(0);
        Response { buf, len }
    }
}

impl<'a> UartReply<'a> {
    fn new(code: u8, subcommand: u8) -> Self {
        Self {
            code,
            subcommand,
            data: None,
        }
    }

    fn data(mut self, data: &'a [u8]) -> Self {
        self.data = Some(data);
        self
    }

    fn build(self) -> Response {
        let mut buf = [0; 64];
        buf[0] = 0x21;
        buf[1] = count();
        let rep = report();
        let n = rep.len();
        buf[2..2 + n].copy_from_slice(&rep);
        buf[n + 2] = self.code;
        buf[n + 3] = self.subcommand;

        let len = n
            + 4
            + self
                .data
                .map(|data| {
                    buf[n + 4..n + 4 + data.len()].copy_from_slice(data);
                    data.len()
                })
                .unwrap_or(0);

        Response { buf, len }
    }
}

impl AsRef<[u8]> for Response {
    fn as_ref(&self) -> &[u8] {
        &self.buf
    }
}

impl Response {
    pub fn new(req: &[u8]) -> Option<Self> {
        if req.len() >= 2 && req[0] == 0x80 {
            Some(match req[1] {
                0x01 => Reply::new(0x81, 0x01)
                    .data(&[
                        0x00, // TODO, what is this byte?
                        0x03, // pro controller
                        0x57, 0x30, 0xea, 0x8a, 0xbb, 0x7c, // mac address (BE),
                    ])
                    .build(),
                0x04 => Reply::new(0x30, count()).data(&report()).build(),
                0x02 | 0x03 => Reply::new(0x81, req[1]).build(),
                _ => return None,
            })
        } else if req.len() > 16 && req[0] == 0x01 {
            let subcommand = req[10];
            Some(match subcommand {
                SUBCOMMAND_BLUETOOTH_MANUAL_PAIRING => {
                    UartReply::new(0x81, subcommand).data(&[0x03]).build()
                }
                SUBCOMMAND_REQUEST_DEVICE_INFO => {
                    UartReply::new(0x82, subcommand)
                        .data(&[
                            0x03, 0x48, // firmware version
                            0x03, // pro controller
                            0x02, // unknown
                            0x57, 0x30, 0xef, 0x8a, 0xbb, 0x7c, // mac address (BE),
                            0x03, // unknown
                            0x02,
                        ])
                        .build()
                }
                SUBCOMMAND_SET_INPUT_REPORT_MODE
                | SUBCOMMAND_SET_SHIPMENT_LOW_POWER_STATE
                | SUBCOMMAND_SET_PLAYER_LIGHTS
                | SUBCOMMAND_SET_HOME_LIGHTS
                | SUBCOMMAND_ENABLE_IMU
                | SUBCOMMAND_ENABLE_VIBRATION => UartReply::new(0x80, subcommand).build(),
                SUBCOMMAND_SPI_FLASH_READ => {
                    let address = u16::from_be_bytes([req[12], req[11]]);
                    let size = req[15] as usize;
                    let mut buf = [0u8; 64];
                    let mut vec = crate::vec::Vec::new(&mut buf);
                    crate::spi::read(address, &mut vec, size);
                    UartReply::new(0x90, subcommand).data(vec.as_ref()).build()
                }
                SUBCOMMAND_TRIGGER_BUTTONS_ELAPSED_TIME => UartReply::new(0x83, subcommand).build(),
                _ => UartReply::new(0x80, subcommand).build(),
            })
        } else {
            None
        }
    }

    pub fn idle() -> Self {
        Reply::new(0x30, count()).data(&report()).build()
    }
}
