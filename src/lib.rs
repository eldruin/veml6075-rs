//! This is a platform agnostic Rust driver for the VEML6075 UVA and UVB
//! light sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the sensor. See: [`enable()`].
//! - Read the UVA measurement. See: [`read_uva()`].
//! - Read the UVB measurement. See: [`read_uvb()`].
//! - Read the UVcomp1 measurement. See: [`read_uvcomp1()`].
//! - Read the UVcomp2 measurement. See: [`read_uvcomp2()`].
//! - Read all sensor data at once. See: [`read_all()`].
//! - Set integration time. See: [`set_integration_time()`].
//! - Set dynamic setting. See: [`set_dynamic_setting()`].
//! - Change operating mode. See: [`set_mode()`].
//! - Trigger measurement when on active force mode. See: [`trigger_measurement()`].
//! - Read the device id. See: [`read_device_id()`].
//!
//! [`enable()`]: struct.Veml6075.html#method.enable
//! [`read_uva()`]: struct.Veml6075.html#method.read_uva
//! [`read_uvb()`]: struct.Veml6075.html#method.read_uvb
//! [`read_uvcomp1()`]: struct.Veml6075.html#method.read_uvcomp1
//! [`read_uvcomp2()`]: struct.Veml6075.html#method.read_uvcomp2
//! [`read_all()`]: struct.Veml6075.html#method.read_all
//! [`set_integration_time()`]: struct.Veml6075.html#method.set_integration_time
//! [`set_dynamic_setting()`]: struct.Veml6075.html#method.set_dynamic_setting
//! [`set_mode()`]: struct.Veml6075.html#method.set_mode
//! [`trigger_measurement()`]: struct.Veml6075.html#method.trigger_measurement
//! [`read_device_id()`]: struct.Veml6075.html#method.read_device_id
//!
//! ## The device
//! The VEML6075 senses UVA and UVB light and incorporates photodiode,
//! amplifiers,and analog / digital circuits into a single chip using a
//! CMOS process. When the UV sensor is applied, it is able to detect
//! UVA and UVB intensity to provide a measure of the signal strength
//! as well as allowing for UVI measurement.
//! The VEML6075 provides excellent temperature compensation capability
//! for keeping the output stable under changing temperature.
//! VEML6075's functionality is easilyoperated via the simple command
//! format of I2C (SMBus compatible) interface protocol.
//! VEML6075's operating voltage ranges from 1.7 V to 3.6 V.
//!
//! Datasheet:
//! - [VEML6075](https://www.vishay.com/docs/84304/veml6075.pdf)
//!
//! Application note:
//! - [VEML6075 AN](https://www.vishay.com/docs/84339/designingveml6075.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! ### Read UVA and UVB
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6075;
//! use veml6075::Veml6075;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6075::new(dev);
//! let uva = sensor.read_uva().unwrap();
//! let uvb = sensor.read_uvb().unwrap();
//! println!("Measurements UVA: {}, UVB: {}", uva, uvb);
//! # }
//! ```
//!
//! ### Read all channels at once
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6075;
//! use veml6075::Veml6075;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6075::new(dev);
//! let measurement = sensor.read_all().unwrap();
//! println!("Measurements UVA: {}, UVB: {}", measurement.uva, measurement.uvb);
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Result of measurement of all channels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// UVA sensor data
    pub uva: u16,
    /// UVB sensor data
    pub uvb: u16,
    /// UVcomp1 sensor data
    pub uvcomp1: u16,
    /// UVcomp2 sensor data
    pub uvcomp2: u16,
}

/// Integration time
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegrationTime {
    /// 50 ms
    Ms50,
    /// 100 ms
    Ms100,
    /// 200 ms
    Ms200,
    /// 400 ms
    Ms400,
    /// 800 ms
    Ms800,
}

/// Dynamic setting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DynamicSetting {
    /// Normal dynamic setting
    Normal,
    /// High dynamic setting
    High,
}

/// Operating mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    /// Continuous measurement (default)
    Continuous,
    /// Active force (one-shot)
    ActiveForce,
}

/// Veml6075 device driver.
#[derive(Debug, Default)]
pub struct Veml6075<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Configuration register status.
    config: u8,
}

mod device_impl;
