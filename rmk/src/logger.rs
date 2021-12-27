use crate::c_hal;

use alloc::format;
use alloc::string::String;
// Platform specific
use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::Mutex;

use core::cell::RefCell;
use log::LevelFilter;

type SerialPort =
    usbd_serial::SerialPort<'static, c_hal::usbd::Usbd<c_hal::usbd::UsbPeripheral<'static>>>;
pub struct SerialLogger {
    default_level: LevelFilter,
    serial: Mutex<RefCell<SerialPort>>,
}
impl SerialLogger {
    #[must_use = "You must call init() to begin logging"]
    pub const fn new(serial: SerialPort) -> Self {
        Self {
            default_level: LevelFilter::Trace,
            serial: Mutex::new(RefCell::new(serial)),
        }
    }

    #[must_use = "You must call init() to begin logging"]
    pub const fn with_level(mut self, level: log::LevelFilter) -> Self {
        self.default_level = level;
        self
    }

    pub fn init(&'static self) -> Result<(), log::SetLoggerError> {
        log::set_logger(self)
    }
}

impl log::Log for SerialLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level().to_level_filter() <= self.default_level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let s = format!(
            "{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()
        );

        let mut slice = s.as_bytes();

        while slice.len() > 0 {
            use usbd_serial::UsbError as E;

            let nbytes = interrupt_free(|cs| self.serial.borrow(cs).borrow_mut().write(slice));
            match nbytes {
                Ok(nbytes) => slice = &slice[nbytes..],
                Err(E::WouldBlock) => self.flush(),
                Err(_) => (), // infinite loop until hopefully we stop erroring
            }
        }
    }

    fn flush(&self) {
        use usbd_serial::UsbError as E;
        while interrupt_free(|cs| match self.serial.borrow(cs).borrow_mut().flush() {
            Ok(()) => false,                     // break loop
            Err(E::WouldBlock) | Err(_) => true, // continue looping
        }) {
            core::hint::spin_loop();
        }
    }
}
