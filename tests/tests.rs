extern crate embedded_hal_mock as hal;
extern crate veml6075;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
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

pub fn new(transactions: &[I2cTrans]) -> Veml6075<I2cMock> {
    Veml6075::new(I2cMock::new(&transactions))
}

pub fn destroy(sensor: Veml6075<I2cMock>) {
    sensor.destroy().done();
}

#[test]
fn can_enable() {
    let transactions = [I2cTrans::write(
        DEVICE_ADDRESS,
        vec![Register::CONFIG, 0, 0],
    )];
    let mut dev = new(&transactions);
    dev.enable().unwrap();
    destroy(dev);
}

#[test]
fn can_disable() {
    let transactions = [I2cTrans::write(
        DEVICE_ADDRESS,
        vec![Register::CONFIG, 1, 0],
    )];
    let mut dev = new(&transactions);
    dev.disable().unwrap();
    destroy(dev);
}

macro_rules! read_test {
    ($name:ident, $method:ident, $reg:ident) => {
        #[test]
        fn $name() {
            let transactions = [I2cTrans::write_read(
                DEVICE_ADDRESS,
                vec![Register::$reg],
                vec![0xCD, 0xAB],
            )];
            let mut dev = new(&transactions);
            let reading = dev.$method().unwrap();
            assert_eq!(reading, 0xABCD);
            destroy(dev);
        }
    };
}

read_test!(can_read_uva, read_uva, UVA);
read_test!(can_read_uvb, read_uvb, UVB);
read_test!(can_read_uvcomp1, read_uvcomp1, UVCOMP1);
read_test!(can_read_uvcomp2, read_uvcomp2, UVCOMP2);
read_test!(can_read_dev_id, read_device_id, DEVICE_ID);
