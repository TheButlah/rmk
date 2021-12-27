#![no_std]
#![no_main]

// ---- Global Handlers and platform-specific crate selection
use cortex_m_rt::entry;
use embedded_hal as hal;
use nrf52840_hal as c_hal;

use rmk_hid::{report::BootKeyboardReport, Keycode};

use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use lazy_static::lazy_static;
use usb_device::UsbError;
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

type SerialPort<'a> =
    usbd_serial::SerialPort<'a, c_hal::usbd::Usbd<c_hal::usbd::UsbPeripheral<'a>>>;

type Clocks = c_hal::Clocks<
    c_hal::clocks::ExternalOscillator,
    c_hal::clocks::Internal,
    c_hal::clocks::LfOscStopped,
>;

static SERIAL: Option<SerialPort> = None;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{}", info);
    log::error!("Halting program via busy loop...");
    log::logger().flush();
    loop {}
}

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

    let mut serial = SerialPort::new(&usb_alloc);
    let mut usb_dev = {
        use usb_device::prelude::*;
        UsbDeviceBuilder::new(&usb_alloc, UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(usbd_serial::USB_CLASS_CDC)
            .max_packet_size_0(64) // (makes control transfers 8x faster)
            .build()
    };

    let mut is_on = true;
    let mut delay = c_hal::delay::Delay::new(core_periph.SYST);
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        match serial.write("hello\n".as_bytes()) {
            Ok(count) => {
                // count bytes were written
                ()
            }
            Err(UsbError::WouldBlock) => (), // No data could be written (buffers full)
            Err(err) => (),                  // An error occurred
        }
        serial.flush().ok();

        // is_on = !is_on;
        // if is_on {
        //     bled.set_high().unwrap();
        //     key_report.keycodes[0] = Keycode::O as u8;
        //     key_report.keycodes[1] = Keycode::N as u8;
        //     // hid_class
        //     //     .push_input(&key_report)
        //     //     .expect("Failed to push report");
        // } else {
        //     bled.set_low().unwrap();
        //     key_report.keycodes[0] = 0;
        //     key_report.keycodes[1] = 0;
        //     // hid_class
        //     //     .push_input(&key_report)
        //     //     .expect("Failed to push report");
        // }
    }
}
