extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate veml6075;

use linux_embedded_hal::I2cdev;
use veml6075::VEML6075;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = VEML6075::new(dev);
    let uva = sensor.read_uva().unwrap(); 
    let uvb = sensor.read_uvb().unwrap();
    println!("Measurements UVA: {}, UVB: {}", uva, uvb);
}
