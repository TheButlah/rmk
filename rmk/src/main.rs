#![no_std]
#![no_main]

use panic_semihosting as _;

use embedded_hal as hal;
use nrf52840_hal as c_hal;

use cortex_m_rt::entry;
use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use rmk_hid::{report::BootKeyboardReport, Keycode};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

#[entry]
fn main() -> ! {
    let periph = c_hal::pac::Peripherals::take().unwrap();
    let core_periph = c_hal::pac::CorePeripherals::take().unwrap();
    let port0 = c_hal::gpio::p0::Parts::new(periph.P0);
    let mut bled = port0.p0_15.into_push_pull_output(c_hal::gpio::Level::High);

    let clocks = c_hal::Clocks::new(periph.CLOCK).enable_ext_hfosc();
    let usb_alloc = c_hal::usbd::Usbd::new(c_hal::usbd::UsbPeripheral::new(periph.USBD, &clocks));

    let hid_class = HIDClass::new(&usb_alloc, BootKeyboardReport::desc(), 10);
    let mut key_report = BootKeyboardReport::default();

    let mut is_on = true;
    let mut delay = c_hal::delay::Delay::new(core_periph.SYST);
    loop {
        delay.delay_ms(1000u16);
        is_on = !is_on;
        if is_on {
            bled.set_high().unwrap();
            key_report.keycodes[0] = Keycode::O as u8;
            key_report.keycodes[1] = Keycode::N as u8;
            hid_class
                .push_input(&key_report)
                .expect("Failed to push report");
        } else {
            bled.set_low().unwrap();
            key_report.keycodes[0] = 0;
            key_report.keycodes[1] = 0;
            hid_class
                .push_input(&key_report)
                .expect("Failed to push report");
        }
    }
}
