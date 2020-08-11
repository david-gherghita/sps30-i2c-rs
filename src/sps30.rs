use crate::register_access::sps30::{DEV_ADDR, Register, StatusRegisterBits};
use crate::Sps30;
use crate::types::{AirInfo, Error, StatusRegisterResult};
use byteorder::{BigEndian, ByteOrder};
use embedded_hal::blocking::{delay, i2c};

impl<I2C, D, E> Sps30<I2C, D>
where I2C: i2c::Read<Error = E> + i2c::Write<Error = E>,
D: delay::DelayMs<u8> {
    /// Create new instance of the SPS30 device
    pub fn new_sps30(i2c: I2C, delay: D) -> Self {
        Sps30 {
            i2c,
            delay,
            address: DEV_ADDR,
        }
    }

    /// Destory driver instance
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enter measurement mode
    /// Command execution time: 20 ms
    pub fn start_measurement(&mut self) -> Result<(), Error<E>> {
        let mut data: [u8; 5] = [0; 5];
        data[..2].clone_from_slice(&Register::START_MEASUREMENT[..2]);
        data[2] = 0x03;
    
        self.write_data(&mut data)?;
        self.delay.delay_ms(20);

        Ok(())
    }

    /// Exit measurement mode
    /// Command execution time: 20 ms
    pub fn stop_measurement(&mut self) -> Result<(), Error<E>> {
        let mut data: [u8; 2] = Register::STOP_MEASUREMENT;

        self.write_data(&mut data)?;
        self.delay.delay_ms(20);

        Ok(())
    }

    /// Poll for the availability of new measurements
    /// Command execution time: -
    pub fn read_data_ready_flag(&mut self) -> Result<bool, Error<E>> {
        let mut data: [u8; 2] = Register::READ_DATA_READY_FLAG;
        self.write_data(&mut data)?;

        let mut buffer: [u8; 3] = [0; 3];
        self.read_data(&mut buffer)?;

        if buffer[1] == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Read the measured values
    /// Command execution time: -
    pub fn read_measured_values(&mut self) -> Result<AirInfo, Error<E>> {
        let mut data: [u8; 2] = Register::READ_MEASURED_VALUES;
        self.write_data(&mut data)?;

        let mut buffer: [u8; 60] = [0; 60];
        self.read_data(&mut buffer)?;

        let air_info: AirInfo = AirInfo {
            mass_pm1_0: BigEndian::read_f32(&buffer[0..]),
            mass_pm2_5: BigEndian::read_f32(&buffer[4..]), 
            mass_pm4_0: BigEndian::read_f32(&buffer[4 * 2..]),
            mass_pm10: BigEndian::read_f32(&buffer[4 * 3..]),
            number_pm0_5: BigEndian::read_f32(&buffer[4 * 4..]),
            number_pm1_0: BigEndian::read_f32(&buffer[4 * 5..]),
            number_pm2_5: BigEndian::read_f32(&buffer[4 * 6..]),
            number_pm4_0: BigEndian::read_f32(&buffer[4 * 7..]),
            number_pm10: BigEndian::read_f32(&buffer[4 * 8..]),
            typical_size: BigEndian::read_f32(&buffer[4 * 9..]),
        };

        Ok(air_info)
    }
    
    /// Enter sleep mode
    /// Command execution time: 5 ms
    pub fn sleep(&mut self) -> Result<(), Error<E>> {
        let mut data: [u8; 2] = Register::SLEEP;

        self.write_data(&mut data)?;
        self.delay.delay_ms(5);

        Ok(())
    }

    /// Exit sleep mode
    /// Command execution time: 5 ms
    pub fn wake_up(&mut self) -> Result<(), Error<E>> {
        let mut data: [u8; 2] = Register::WAKE_UP;
    
        self.write_data(&mut [])?;
        self.write_data(&mut data)?;
        self.delay.delay_ms(5);

        Ok(())
    }

    /// Start the fan-cleaning manually
    /// This commmand can only be executed in Measurement-Mode
    /// Command execution time: 5 ms
    pub fn start_fan_cleaning(&mut self) -> Result<(), Error<E>> {
        let mut data: [u8; 2] = Register::START_FAN_CLEANING;

        self.write_data(&mut data)?;
        self.delay.delay_ms(5);

        Ok(())
    }

    /// Read the interval[s] of the periodic fan-cleaning
    /// Command execution time: 5 ms
    pub fn read_auto_cleaning_interval(&mut self) -> Result<u32, Error<E>> {
        let mut data: [u8; 2] = Register::READ_WRITE_AUTO_CLEANING_INTERVAL;

        self.write_data(&mut data)?;
        self.delay.delay_ms(5);

        let mut buffer: [u8; 6] = [0; 6];
        self.read_data(&mut buffer)?;

        Ok(BigEndian::read_u32(&buffer))
    }

    /// Write the interval[s] of the periodic fan-cleaning
    /// Command execution time: 20 ms
    pub fn write_auto_cleaning_interval(&mut self, n: u32) -> Result<(), Error<E>> {
        let mut data: [u8; 8] = [0; 8];
        data[..2].clone_from_slice(&Register::READ_WRITE_AUTO_CLEANING_INTERVAL[..2]);
        BigEndian::write_u32(&mut data[2..], n);

        self.write_data(&mut data)?;
        self.delay.delay_ms(20);

        Ok(())
    }

    /// Read device product type
    /// Command execution time: -
    pub fn read_device_product_type(&mut self) -> Result<[u8; 8], Error<E>> {
        let mut data: [u8; 2] = Register::READ_DEVICE_PRODUCT_TYPE;
        self.write_data(&mut data)?;

        let mut buffer: [u8; 12] = [0; 12];
        self.read_data(&mut buffer)?;
        
        let mut res: [u8; 8] = [0; 8];
        res[..8].clone_from_slice(&buffer[..8]);

        Ok(res)
    }

    /// Read device serial number
    /// Command execution time: -
    pub fn read_device_serial_number(&mut self) -> Result<[u8; 32], Error<E>> {
        let mut data: [u8; 2] = Register::READ_DEVICE_SERIAL_NUMBER;
        self.write_data(&mut data)?;

        let mut buffer: [u8; 48] = [0; 48];
        self.read_data(&mut buffer)?;

        let mut res: [u8; 32] = [0; 32];
        res[..32].clone_from_slice(&buffer[..32]);

        Ok(res)
    }

    /// Read firmware version
    /// Command execution time: -
    pub fn read_firmware_version(&mut self) -> Result<(u8, u8), Error<E>> {
        let mut data: [u8; 2] = Register::READ_FIRMWARE_VERSION;
        self.write_data(&mut data)?;

        let mut buffer: [u8; 3] = [0; 3];
        self.read_data(&mut buffer)?;

        Ok((buffer[0], buffer[1]))
    }

    /// Read device status register
    /// Command execution time: -
    pub fn read_device_status_register(&mut self) -> Result<StatusRegisterResult, Error<E>> {
        let mut data: [u8; 2] = Register::READ_DEVICE_STATUS_REGISTER;
        self.write_data(&mut data)?;

        let mut buffer: [u8; 6] = [0; 6];
        self.read_data(&mut buffer)?;

        let res = BigEndian::read_u32(&buffer);

        let status_speed: bool = {
            (res & StatusRegisterBits::SPEED) != 0
        };

        let status_laser: bool = {
            (res & StatusRegisterBits::LASER) != 0
        };

        let status_fan: bool = {
            (res & StatusRegisterBits::FAN) != 0
        };

        Ok(StatusRegisterResult{
            speed: status_speed,
            laser: status_laser,
            fan: status_fan,
        })
    }

    /// Clear device status register
    /// Command execution time: 5 ms
    pub fn clear_device_status_register(&mut self) -> Result<(), Error<E>> {
        let mut data: [u8; 2] = Register::CLEAR_DEVICE_STATUS_REGISTER;

        self.write_data(&mut data)?;
        self.delay.delay_ms(5);

        Ok(())
    }

    /// Reset the device
    /// Command execution time: 100 ms
    pub fn device_reset(&mut self) -> Result<(), Error<E>> {
        let mut data: [u8; 2] = Register::DEVICE_RESET;

        self.write_data(&mut data)?;
        self.delay.delay_ms(100);
        
        Ok(())
    }
}
