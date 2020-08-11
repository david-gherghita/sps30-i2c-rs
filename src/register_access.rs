use crate::types::Error;
use crate::crc;
use embedded_hal::blocking::i2c;

pub mod sps30 {
    pub const DEV_ADDR: u8 = 0x69;
    pub struct Register {}
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

    pub struct StatusRegisterBits {}
    impl StatusRegisterBits {
        pub const SPEED: u32 = 1 << 21;
        pub const LASER: u32 = 1 << 5;
        pub const FAN: u32 = 1 << 4;
    }
}

impl<I2C, D, E> crate::Sps30<I2C, D>
where I2C: i2c::Read<Error = E> + i2c::Write<Error = E> {
    pub(crate) fn read_data(&mut self, buffer: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c.read(self.address, buffer).map_err(Error::I2C)?;
        self.check_crc(buffer)?;
        self.remove_crc(buffer);
        Ok(())
    }
    
    pub(crate) fn write_data(&mut self, buffer: &mut [u8]) -> Result<(), Error<E>>{
        if buffer.len() == 0 {
            let _ = self.i2c.write(self.address, &buffer);
            return Ok(());
        }

        if buffer.len() > 2 {
            self.add_crc(buffer);
        }

        self.i2c.write(self.address, &buffer).map_err(Error::I2C)
    }

    fn check_crc(&mut self, data: &[u8]) -> Result<(), Error<E>> {
        for i in 0..data.len() {
            if i % 3 == 2 {
                if crc::calc_crc(&[data[i - 2], data[i - 1]]) != data[i] {
                    return Err(Error::ChecksumMismatch);
                }
            }
        }
        Ok(())
    }

    fn remove_crc(&mut self, data: &mut [u8]) {
        for i in 2..data.len() {
            if i % 2 == 0 {
                for j in i..data.len() - 1 {
                    data[j] = data[j + 1];
                }
            }
        }
    }

    fn add_crc(&mut self, data: &mut [u8]) {
        for i in 4..data.len() {
            if i % 3 == 1 {
                for j in (i..data.len() - 1).rev() {
                    data[j + 1] = data[j];
                }
                data[i] = crc::calc_crc(&[data[i - 2], data[i - 1]]);
            }
        }
    }
}