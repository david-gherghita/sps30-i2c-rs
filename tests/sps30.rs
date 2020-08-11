use sps30_i2c::Sps30;
use embedded_hal_mock::{delay::MockNoop as NoopDelay, i2c::Mock as I2cMock,
    i2c::Transaction as I2cTrans};

const DEV_ADDR: u8 = 0x69;
struct Register {}
impl Register {
    pub const START_MEASUREMENT: [u8; 2] = [0x00, 0x10];
    pub const STOP_MEASUREMENT: [u8; 2] = [0x01, 0x04];
    pub const READ_DATA_READY_FLAG: [u8; 2] = [0x02, 0x02];
    pub const READ_MEASURED_VALUES: [u8; 2] = [0x03, 0x00];
    pub const SLEEP: [u8; 2] = [0x10, 0x01];
    pub const WAKE_UP: [u8; 2] = [0x11, 0x03];
    pub const START_FAN_CLEANING: [u8; 2] = [0x56, 0x07];
    pub const READ_WRITE_AUTO_CLEANING_INTERVAL: [u8; 2] = [0x80, 0x04];
    pub const READ_DEVICE_PRODUCT_TYPE: [u8; 2] = [0xD0, 0x02];
    pub const READ_DEVICE_SERIAL_NUMBER: [u8; 2] = [0xD0, 0x33];
    pub const READ_FIRMWARE_VERSION: [u8; 2] = [0xD1, 0x00];
    pub const READ_DEVICE_STATUS_REGISTER: [u8; 2] = [0xD2, 0x06];
    pub const CLEAR_DEVICE_STATUS_REGISTER: [u8; 2] = [0xD2, 0x10];
    pub const DEVICE_RESET: [u8; 2] = [0xD3, 0x04];
}

fn calc_crc(data: &[u8; 2]) -> u8 {
    let mut crc: u8 = 0xFF;

    for i in 0..2 {
        crc ^= data[i];
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc = crc << 1;
            }
        }
    }

    crc
}

#[test]
fn test_create_destroy() {
    let sensor = Sps30::new_sps30(I2cMock::new(&[]), NoopDelay);

    sensor.destroy();
}

#[test]
fn test_start_measurement() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::START_MEASUREMENT);
    cmd.extend_from_slice(&[0x03, 0x00, calc_crc(&[0x03, 0x00])]);

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.start_measurement().unwrap();

    sensor.destroy();
}

#[test]
fn test_stop_measurement() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::STOP_MEASUREMENT);

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.stop_measurement().unwrap();

    sensor.destroy();
}

#[test]
fn test_read_data_ready_flag() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_DATA_READY_FLAG);

    let mut res: Vec<u8> = Vec::new();
    res.extend_from_slice(&[0x00, 0x00, calc_crc(&[0x00, 0x00])]);

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
        I2cTrans::read(DEV_ADDR, res),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.read_data_ready_flag().unwrap();

    sensor.destroy();
}

#[test]
fn test_read_measured_values() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_MEASURED_VALUES);

    let mut res: Vec<u8> = vec![0; 60];
    for i in 0..60 {
        if i % 3 == 2 {
            res[i] = calc_crc(&[res[i - 2], res[i - 1]]);
        }
    }

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
        I2cTrans::read(DEV_ADDR, res),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.read_measured_values().unwrap();

    sensor.destroy();
}

#[test]
fn test_sleep() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::SLEEP);

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.sleep().unwrap();

    sensor.destroy();
}

#[test]
fn test_wake_up() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::WAKE_UP);

    let expectations = [
        I2cTrans::write(DEV_ADDR, vec![]),
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.wake_up().unwrap();

    sensor.destroy();
}

#[test]
fn test_start_fan_cleaning() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::START_FAN_CLEANING);

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.start_fan_cleaning().unwrap();

    sensor.destroy();
}

#[test]
fn test_read_auto_cleaning_interval() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_WRITE_AUTO_CLEANING_INTERVAL);

    let mut res: Vec<u8> = vec![0; 6];
    for i in 0..6 {
        if i % 3 == 2 {
            res[i] = calc_crc(&[res[i - 2], res[i - 1]]);
        }
    }

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
        I2cTrans::read(DEV_ADDR, res),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.read_auto_cleaning_interval().unwrap();

    sensor.destroy();
}

#[test]
fn test_write_auto_cleaning_interval() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_WRITE_AUTO_CLEANING_INTERVAL);
    cmd.extend_from_slice(&[0; 6]);
    for i in 0..6 {
        if i % 3 == 2 {
            cmd[i + 2] = calc_crc(&[cmd[i - 2 + 2], cmd[i - 1 + 2]]);
        }
    }

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.write_auto_cleaning_interval(0).unwrap();

    sensor.destroy();
}

#[test]
fn test_read_device_product_type() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_DEVICE_PRODUCT_TYPE);

    let mut res: Vec<u8> = vec![0; 12];
    for i in 0..12 {
        if i % 3 == 2 {
            res[i] = calc_crc(&[res[i - 2], res[i - 1]]);
        }
    }

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
        I2cTrans::read(DEV_ADDR, res),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.read_device_product_type().unwrap();

    sensor.destroy();
}

#[test]
fn test_read_device_serial_number() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_DEVICE_SERIAL_NUMBER);

    let mut res: Vec<u8> = vec![0; 48];
    for i in 0..48 {
        if i % 3 == 2 {
            res[i] = calc_crc(&[res[i - 2], res[i - 1]]);
        }
    }

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
        I2cTrans::read(DEV_ADDR, res),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.read_device_serial_number().unwrap();

    sensor.destroy();
}

#[test]
fn test_read_firmware_version() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_FIRMWARE_VERSION);

    let mut res: Vec<u8> = vec![0; 3];
    for i in 0..3 {
        if i % 3 == 2 {
            res[i] = calc_crc(&[res[i - 2], res[i - 1]]);
        }
    }

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
        I2cTrans::read(DEV_ADDR, res),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.read_firmware_version().unwrap();

    sensor.destroy();
}

#[test]
fn test_read_device_status_register() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::READ_DEVICE_STATUS_REGISTER);

    let mut res: Vec<u8> = vec![0; 6];
    for i in 0..6 {
        if i % 3 == 2 {
            res[i] = calc_crc(&[res[i - 2], res[i - 1]]);
        }
    }

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
        I2cTrans::read(DEV_ADDR, res),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.read_device_status_register().unwrap();

    sensor.destroy();
}

#[test]
fn test_clear_device_status_register() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::CLEAR_DEVICE_STATUS_REGISTER);

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.clear_device_status_register().unwrap();

    sensor.destroy();
}

#[test]
fn test_device_reset() {
    let mut cmd: Vec<u8> = Vec::new();
    cmd.extend_from_slice(&Register::DEVICE_RESET);

    let expectations = [
        I2cTrans::write(DEV_ADDR, cmd),
    ];
    let mut sensor = Sps30::new_sps30(I2cMock::new(&expectations), NoopDelay);

    sensor.device_reset().unwrap();

    sensor.destroy();
}
