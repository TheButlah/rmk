#![no_std]
#![no_main]

use panic_semihosting as _;

use embedded_hal as hal;
use nrf52840_hal as c_hal;

use cortex_m_rt::entry;
use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

#[entry]
fn main() -> ! {
    let periph = c_hal::pac::Peripherals::take().unwrap();
    let core_periph = c_hal::pac::CorePeripherals::take().unwrap();
    let port0 = c_hal::gpio::p0::Parts::new(periph.P0);
    let mut bled = port0.p0_15.into_push_pull_output(c_hal::gpio::Level::High);

    let mut is_on = true;
    let mut delay = c_hal::delay::Delay::new(core_periph.SYST);
    loop {
        delay.delay_ms(1000u16);
        is_on = !is_on;
        if is_on {
            bled.set_high().unwrap();
        } else {
            bled.set_low().unwrap();
        }
    }
}
