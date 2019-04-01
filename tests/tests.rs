extern crate embedded_hal_mock as hal;
extern crate veml6075;
use veml6075::Veml6075;
use hal::i2c::{Transaction as I2cTrans, Mock as I2cMock};

const DEVICE_ADDRESS: u8 = 0x10;
struct Register;
impl Register {
    const CONFIG: u8 = 0x00;
    const UVA: u8 = 0x07;
    const UVB: u8 = 0x09;
    const UVCOMP1: u8 = 0x0A;
    const UVCOMP2: u8 = 0x0B;
    const DEVICE_ID: u8 = 0x0C;
}


pub fn new(
    transactions: &[I2cTrans],
) -> Veml6075<I2cMock> {
    Veml6075::new(I2cMock::new(&transactions))
}

pub fn destroy(sensor: Veml6075<I2cMock>) {
    sensor.destroy().done();
}

#[test]
fn can_enable() {
    let transactions = [I2cTrans::write(DEVICE_ADDRESS, vec![Register::CONFIG, 0, 0])];
    let mut dev = new(&transactions);
    dev.enable().unwrap();
    destroy(dev);
}

#[test]
fn can_disable() {
    let transactions = [I2cTrans::write(DEVICE_ADDRESS, vec![Register::CONFIG, 1, 0])];
    let mut dev = new(&transactions);
    dev.disable().unwrap();
    destroy(dev);
}

#[test]
fn can_read_uva() {
    let transactions = [I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVA], vec![0xCD, 0xAB])];
    let mut dev = new(&transactions);
    let reading = dev.read_uva().unwrap();
    assert_eq!(reading, 0xABCD);
    destroy(dev);
}

#[test]
fn can_read_uvb() {
    let transactions = [I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVB], vec![0xCD, 0xAB])];
    let mut dev = new(&transactions);
    let reading = dev.read_uvb().unwrap();
    assert_eq!(reading, 0xABCD);
    destroy(dev);
}

#[test]
fn can_read_uvcomp1() {
    let transactions = [I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVCOMP1], vec![0xCD, 0xAB])];
    let mut dev = new(&transactions);
    let reading = dev.read_uvcomp1().unwrap();
    assert_eq!(reading, 0xABCD);
    destroy(dev);
}

#[test]
fn can_read_uvcomp2() {
        let transactions = [I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVCOMP2], vec![0xCD, 0xAB])];
    let mut dev = new(&transactions);
    let reading = dev.read_uvcomp2().unwrap();
    assert_eq!(reading, 0xABCD);
    destroy(dev);
}

#[test]
fn can_read_device_id() {
    let transactions = [I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::DEVICE_ID], vec![0xCD, 0xAB])];
    let mut dev = new(&transactions);
    let reading = dev.read_device_id().unwrap();
    assert_eq!(reading, 0xABCD);
    destroy(dev);
}
