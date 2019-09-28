# Rust VEML6075 UVA and UVB Light Sensor Driver

[![crates.io](https://img.shields.io/crates/v/veml6075.svg)](https://crates.io/crates/veml6075)
[![Docs](https://docs.rs/veml6075/badge.svg)](https://docs.rs/veml6075)
[![Build Status](https://travis-ci.org/eldruin/veml6075-rs.svg?branch=master)](https://travis-ci.org/eldruin/veml6075-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/veml6075-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/veml6075-rs?branch=master)

This is a platform agnostic Rust driver for the VEML6075 UVA and UVB light sensor,
based on the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

This driver allows you to:
- Enable/disable the sensor. See: `enable()`.
- Read calibrated UVA, UVB and UV index measurement. See: `read()`.
- Read raw measurement. See: `read_uva_raw()`.
- Set integration time. See: `set_integration_time()`.
- Set dynamic setting. See: `set_dynamic_setting()`.
- Change operating mode. See: `set_mode()`.
- Trigger measurement when on active force mode. See: `trigger_measurement()`.
- Read the device id. See: `read_device_id()`.

[Introductory blog post](https://blog.eldruin.com/veml6075-uva-uvb-uv-index-light-sensor-driver-in-rust/)

## The device
The VEML6075 senses UVA and UVB light and incorporates photodiode, amplifiers,
and analog / digital circuits into a single chip using a CMOS process. When the
UV sensor is applied, it is able to detect UVA and UVB intensity to provide a
measure of the signal strength as well as allowing for UVI measurement.

The VEML6075 provides excellent temperature compensation capability for keeping
the output stable under changing temperature. VEML6075's functionality is easily
operated via the simple command format of I2C (SMBus compatible) interface protocol.
VEML6075's operating voltage ranges from 1.7 V to 3.6 V.

Datasheet:
- [VEML6075](https://cdn.sparkfun.com/assets/3/c/3/2/f/veml6075.pdf)

Application note:
- [Designing the VEML6075 into an Application](https://cdn.sparkfun.com/assets/3/9/d/4/1/designingveml6075.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate linux_embedded_hal as hal;
extern crate veml6075;
use veml6075::{Calibration, Veml6075};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Veml6075::new(dev, Calibration::default());
    let m = sensor.read().unwrap();
    println!("UVA: {:2}, UVB: {:2}, UVI: {:2}", m.uva, m.uvb, m.uv_index);
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/veml6075-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

