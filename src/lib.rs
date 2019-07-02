//! This is a platform agnostic Rust driver for the VEML6075 UVA and UVB
//! light sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the sensor. See: [`enable()`].
//! - Read calibrated UVA and UVB measurement. See: [`read()`].
//! - Read raw measurement. See: [`read_uva_raw()`].
//! - Set integration time. See: [`set_integration_time()`].
//! - Set dynamic setting. See: [`set_dynamic_setting()`].
//! - Change operating mode. See: [`set_mode()`].
//! - Trigger measurement when on active force mode. See: [`trigger_measurement()`].
//! - Read the device id. See: [`read_device_id()`].
//!
//! [`enable()`]: struct.Veml6075.html#method.enable
//! [`read()`]: struct.Veml6075.html#method.read
//! [`read_uva_raw()`]: struct.Veml6075.html#method.read_uva_raw
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
//! - [VEML6075](https://cdn.sparkfun.com/assets/3/c/3/2/f/veml6075.pdf)
//!
//! Application note:
//! - [Designing the VEML6075 into an Application](https://cdn.sparkfun.com/assets/3/9/d/4/1/designingveml6075.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read calibrated UVA and UVB
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6075;
//! use veml6075::{Calibration, Veml6075};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6075::new(dev, Calibration::default());
//! let m = sensor.read().unwrap();
//! println!("Measurements UVA: {:2}, UVB: {:2}", m.uva, m.uvb);
//! # }
//! ```
//!
//! ### Set integration time to 400ms
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6075;
//! use veml6075::{Calibration, IntegrationTime, Veml6075};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6075::new(dev, Calibration::default());
//! sensor.set_integration_time(IntegrationTime::Ms400).unwrap();
//! # }
//! ```
//!
//! ### Set high dynamic setting
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6075;
//! use veml6075::{Calibration, DynamicSetting, Veml6075};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6075::new(dev, Calibration::default());
//! sensor.set_dynamic_setting(DynamicSetting::High).unwrap();
//! # }
//! ```
//!
//! ### Change mode to active force (one-shot) and trigger a measurement
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6075;
//! use veml6075::{Calibration, Mode, Veml6075};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6075::new(dev, Calibration::default());
//! sensor.set_mode(Mode::ActiveForce).unwrap();
//! loop {
//!     sensor.trigger_measurement().unwrap();
//!     // wait until measurement is ready (integration time)
//!     let m = sensor.read().unwrap();
//!     println!("Measurements UVA: {:2}, UVB: {:2}", m.uva, m.uvb);
//! }
//! # }
//! ```
//!
//! ### Read raw measurements for UV and UVB
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6075;
//! use veml6075::{Calibration, Veml6075};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6075::new(dev, Calibration::default());
//! let uva = sensor.read_uva_raw().unwrap();
//! let uvb = sensor.read_uvb_raw().unwrap();
//! println!("Measurements UVA: {}, UVB: {}", uva, uvb);
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

/// Calibrated Measurement
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// UVA calibrated reading
    pub uva: f32,
    /// UVB calibrated reading
    pub uvb: f32,
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

/// Calibration coefficients
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calibration {
    /// UVA visible (a) coefficient
    pub uva_visible: f32,
    /// UVA IR (b) coefficient
    pub uva_ir: f32,
    /// UVB visible (c) coefficient
    pub uvb_visible: f32,
    /// UVB IR (d) coefficient
    pub uvb_ir: f32,
    /// UVA responsivity
    pub uva_responsivity: f32,
    /// UVB responsivity
    pub uvb_responsivity: f32,
}

/// Veml6075 device driver.
#[derive(Debug, Default)]
pub struct Veml6075<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Configuration register status.
    config: u8,
    calibration: Calibration,
}

mod device_impl;

impl Default for Calibration {
    fn default() -> Self {
        Calibration {
            uva_visible: 2.22,
            uva_ir: 1.33,
            uvb_visible: 2.95,
            uvb_ir: 1.74,
            uva_responsivity: 0.001461,
            uvb_responsivity: 0.002591,
        }
    }
}
