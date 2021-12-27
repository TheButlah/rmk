#![no_std]
extern crate alloc;

pub(crate) mod action;
pub(crate) mod keymap;
pub(crate) mod logger;

/// The hardware-specific/concrete HAL crate to use
pub use nrf52840_hal as c_hal;
