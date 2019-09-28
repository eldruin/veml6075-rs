extern crate embedded_hal_mock as hal;
extern crate veml6075;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use veml6075::{
    Calibration, DynamicSetting as DS, IntegrationTime as IT, Measurement, Mode, Veml6075,
};

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
    Veml6075::new(I2cMock::new(&transactions), Calibration::default())
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
cfg_test!(set_ds_normal, set_dynamic_setting, 1, DS::Normal);
cfg_test!(set_ds_high, set_dynamic_setting, 0b0000_1001, DS::High);
cfg_test!(set_continuous, set_mode, 1, Mode::Continuous);
cfg_test!(set_active_force, set_mode, 0b0000_0011, Mode::ActiveForce);
cfg_test!(can_trigger, trigger_measurement, 0b0000_0101);

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

read_test!(can_read_uva, read_uva_raw, UVA);
read_test!(can_read_uvb, read_uvb_raw, UVB);
read_test!(can_read_uvcomp1, read_uvcomp1_raw, UVCOMP1);
read_test!(can_read_uvcomp2, read_uvcomp2_raw, UVCOMP2);
read_test!(can_read_dev_id, read_device_id, DEVICE_ID);

#[test]
fn can_read_calibrated() {
    let transactions = [
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVA], vec![0x7F, 0x0F]),
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVB], vec![0xBA, 0x16]),
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVCOMP1], vec![0xEF, 0x03]),
        I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::UVCOMP2], vec![0xD7, 0x02]),
    ];
    let mut dev = new(&transactions);
    let Measurement { uva, uvb, uv_index } = dev.read().unwrap();

    let expected_uva = 3967.0 - 2.22 * 1007.0 - 1.33 * 727.0;
    assert!(uva - 0.5 < expected_uva);
    assert!(uva + 0.5 > expected_uva);
    let expected_uvb = 5818.0 - 2.95 * 1007.0 - 1.74 * 727.0;
    assert!(uvb - 0.5 < expected_uvb);
    assert!(uvb + 0.5 > expected_uvb);
    let expected_uv_index = (uva * 0.001_461 + uvb * 0.002_591) / 2.0;
    assert!(uv_index - 0.5 < expected_uv_index);
    assert!(uv_index + 0.5 > expected_uv_index);

    destroy(dev);
}

#[test]
fn calibration_default() {
    let c = Calibration {
        uva_visible: 2.22,
        uva_ir: 1.33,
        uvb_visible: 2.95,
        uvb_ir: 1.74,
        uva_responsivity: 0.001_461,
        uvb_responsivity: 0.002_591,
    };
    assert_eq!(c, Calibration::default());
}

#[test]
fn measurement_can_store() {
    let m = Measurement {
        uva: 1.1,
        uvb: 2.2,
        uv_index: 3.3,
    };
    assert!(m.uva - 0.5 < 1.1);
    assert!(m.uva + 0.5 > 1.1);
    assert!(m.uvb - 0.5 < 2.2);
    assert!(m.uvb + 0.5 > 2.2);
    assert!(m.uv_index - 0.5 < 3.3);
    assert!(m.uv_index + 0.5 > 3.3);
    assert_eq!(
        m,
        Measurement {
            uva: 1.1,
            uvb: 2.2,
            uv_index: 3.3,
        }
    );
}
