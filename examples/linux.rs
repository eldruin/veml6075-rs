extern crate linux_embedded_hal as hal;
extern crate veml6075;
use veml6075::Veml6075;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Veml6075::new(dev);
    let uva = sensor.read_uva().unwrap();
    let uvb = sensor.read_uvb().unwrap();
    println!("Measurements UVA: {}, UVB: {}", uva, uvb);
}
