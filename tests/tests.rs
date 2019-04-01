extern crate embedded_hal_mock as hal;
extern crate veml6075;
use veml6075::Veml6075;

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

fn setup<'a>(data: &'a [u8]) -> Veml6075<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Veml6075::new(dev)
}

fn check_sent_data(sensor: Veml6075<hal::I2cMock>, address: u8, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(address));
    assert_eq!(dev.get_write_data(), &data[..]);
}

#[test]
fn can_enable() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    check_sent_data(dev, DEVICE_ADDRESS, &[Register::CONFIG, 0, 0]);
}

#[test]
fn can_disable() {
    let mut dev = setup(&[0]);
    dev.disable().unwrap();
    check_sent_data(dev, DEVICE_ADDRESS, &[Register::CONFIG, 1, 0]);
}

#[test]
fn can_read_uva() {
    let mut dev = setup(&[0xCD, 0xAB]);
    let reading = dev.read_uva().unwrap();
    assert_eq!(reading, 0xABCD);
    check_sent_data(dev, DEVICE_ADDRESS, &[Register::UVA]);
}

#[test]
fn can_read_uvb() {
    let mut dev = setup(&[0xCD, 0xAB]);
    let reading = dev.read_uvb().unwrap();
    assert_eq!(reading, 0xABCD);
    check_sent_data(dev, DEVICE_ADDRESS, &[Register::UVB]);
}

#[test]
fn can_read_uvcomp1() {
    let mut dev = setup(&[0xCD, 0xAB]);
    let reading = dev.read_uvcomp1().unwrap();
    assert_eq!(reading, 0xABCD);
    check_sent_data(dev, DEVICE_ADDRESS, &[Register::UVCOMP1]);
}

#[test]
fn can_read_uvcomp2() {
    let mut dev = setup(&[0xCD, 0xAB]);
    let reading = dev.read_uvcomp2().unwrap();
    assert_eq!(reading, 0xABCD);
    check_sent_data(dev, DEVICE_ADDRESS, &[Register::UVCOMP2]);
}

#[test]
fn can_read_device_id() {
    let mut dev = setup(&[0xCD, 0xAB]);
    let reading = dev.read_device_id().unwrap();
    assert_eq!(reading, 0xABCD);
    check_sent_data(dev, DEVICE_ADDRESS, &[Register::DEVICE_ID]);
}
