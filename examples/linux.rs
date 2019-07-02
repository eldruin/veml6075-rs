extern crate linux_embedded_hal as hal;
extern crate veml6075;
use veml6075::{Calibration, Veml6075};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Veml6075::new(dev, Calibration::default());
    let m = sensor.read().unwrap();
    println!("Measurements UVA: {:2}, UVB: {:2}", m.uva, m.uvb);
}
