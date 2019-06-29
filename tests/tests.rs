extern crate embedded_hal_mock as hal;
extern crate veml6075;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use veml6075::{IntegrationTime as IT, Measurement, Veml6075};

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

macro_rules! cfg_test {
    ($name:ident, $method:ident, $value:expr $(, $arg:expr)* ) => {
        #[test]
        fn $name() {
            let transactions = [I2cTrans::write(
                DEVICE_ADDRESS,
                vec![Register::CONFIG, $value, 0],
            )];
            let mut dev = new(&transactions);
            dev.$method($($arg),*).unwrap();
            destroy(dev);
        }
    };
}

cfg_test!(can_enable, enable, 0);
cfg_test!(can_disable, disable, 1);
cfg_test!(set_it_50, set_integration_time, 1, IT::Ms50);
cfg_test!(set_it_100, set_integration_time, 0b0001_0001, IT::Ms100);
cfg_test!(set_it_200, set_integration_time, 0b0010_0001, IT::Ms200);
cfg_test!(set_it_400, set_integration_time, 0b0011_0001, IT::Ms400);
cfg_test!(set_it_800, set_integration_time, 0b0100_0001, IT::Ms800);

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

#[test]
fn can_read_all() {
    let transactions = [
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVA], vec![0x34, 0x12]),
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVB], vec![0x78, 0x56]),
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVCOMP1], vec![0xBC, 0x9A]),
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVCOMP2], vec![0xF0, 0xDE]),
    ];
    let mut dev = new(&transactions);
    let measurement = dev.read_all().unwrap();
    assert_eq!(
        Measurement {
            uva: 0x1234,
            uvb: 0x5678,
            uvcomp1: 0x9ABC,
            uvcomp2: 0xDEF0,
        },
        measurement
    );
    destroy(dev);
}
