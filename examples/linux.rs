use linux_embedded_hal::I2cdev;
use veml6075::{Calibration, Veml6075};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Veml6075::new(dev, Calibration::default());
    let m = sensor.read().unwrap();
    println!("UVA: {:2}, UVB: {:2}, UVI: {:2}", m.uva, m.uvb, m.uv_index);
}
