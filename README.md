# Rust VEML6075 UVA and UVB Light Sensor Driver [![crates.io](https://img.shields.io/crates/v/veml6075.svg)](https://crates.io/crates/veml6075) [![Docs](https://docs.rs/veml6075/badge.svg)](https://docs.rs/veml6075) [![Build Status](https://travis-ci.org/eldruin/veml6075-rs.svg?branch=master)](https://travis-ci.org/eldruin/veml6075-rs)

This is a platform agnostic Rust driver for the VEML6075 UVA and UVB light sensor,
based on the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

This driver allows you to:
- Enable/disable the sensor
- Read the UVA measurement
- Read the UVB measurement
- Read the UVcomp1 measurement
- Read the UVcomp2 measurement
- Read the device id

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
- [VEML6075](https://www.vishay.com/docs/84304/veml6075.pdf)

Application note:
- [VEML6075 AN](https://www.vishay.com/docs/84339/designingveml6075.pdf)

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

