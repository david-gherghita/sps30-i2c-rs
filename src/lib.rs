//! This is a platform agnostic Rust driver for the SPS30 particulate matter
//! sensor using the [`embedded-hal`] traits.
//! 
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//! 
//! The driver supports only floating point output and is written for the
//! firmware version 2.1. Although it is fully compatible with older versions
//! and the 2.2 version, it doesn't fully support the latest.
//!
//! This driver allows you to:
//! - Enter measurement mode. See: [`start_measurement()`].
//! - Exit measurement mode. See: [`stop_measurement`].
//! - Poll for the availability of new measurements. See: [`read_data_ready_flag()`].
//! - Read the measured values. See: [`read_measured_values()`].
//! - Enter sleep mode. See: [`sleep()`].
//! - Exit sleep mode. See: [`wake_up()`].
//! - Start the fan-cleaning manually. See: [`start_fan_cleaning()`].
//! - Read the interval[s] of the periodic fan-cleaning. See: [`read_auto_cleaning_interval()`].
//! - Write the interval[s] of the periodic fan-cleaning. See: [`write_auto_cleaning_interval()`].
//! - Read device product type. See: [`read_device_product_type()`].
//! - Read device serial number. See: [`read_device_serial_number()`].
//! - Read firmware version. See: [`read_firmware_version()`].
//! - Read device status register. See: [`read_device_status_register()`].
//! - Clear device status register. See: [`clear_device_status_register()`].
//! - Reset the device. See: [`device_reset()`].
//! 
//! [`start_measurement()`]: struct.Sps30.html#method.start_measurement
//! [`stop_measurement`]: struct.Sps30.html#method.stop_measurement
//! [`read_data_ready_flag()`]: struct.Sps30.html#method.read_data_ready_flag
//! [`read_measured_values()`]: struct.Sps30.html#method.read_measured_values
//! [`sleep()`]: struct.Sps30.html#method.sleep
//! [`wake_up()`]: struct.Sps30.html#method.wake_up
//! [`start_fan_cleaning()`]: struct.Sps30.html#method.start_fan_cleaning
//! [`read_auto_cleaning_interval()`]: struct.Sps30.html#method.read_auto_cleaning_interval
//! [`write_auto_cleaning_interval()`]: struct.Sps30.html#method.write_auto_cleaning_interval
//! [`read_device_product_type()`]: struct.Sps30.html#method.read_device_product_type
//! [`read_device_serial_number()`]: struct.Sps30.html#method.read_device_serial_number
//! [`read_firmware_version()`]: struct.Sps30.html#method.read_firmware_version
//! [`read_device_status_register()`]: struct.Sps30.html#method.read_device_status_register
//! [`clear_device_status_register()`]: struct.Sps30.html#method.clear_device_status_register
//! [`device_reset()`]: struct.Sps30.html#method.device_reset
//! 
//! ## The device
//! 
//! The SPS30 Particulate Matter (PM) sensor is a technological breakthrough
//! in optical PM sensors. Its measurement principle is based on laser
//! scattering and makes use of Sensirion's innovative contamination-resistance
//! technology. This technology, together with high-quality and long-lasting
//! components, enables precise measurements from its first operation and
//! throughout its lifetime of more than ten years. In addition, Sensiron's
//! advanced algorithms provide superior precision for different PM types and
//! higher-resolution particle size binning, opening up new possibilities for
//! the detection of different sorts of environmental dust and other particles.
//! 
//! The SPS30 has been designed for use in various applications and devices, such as:
//! - Air purifiers
//! - HVAC equipment
//! - Demand-controlled ventilation systems
//! - Air conditioners
//! - Air quality and environmental monitors
//! - Smart home and IoT devices
//! 
//! Documentation:
//! - Datasheet: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/9.6_Particulate_Matter/Datasheets/Sensirion_PM_Sensors_SPS30_Datasheet.pdf
//!
//! ## Usage 
//! 
//! To use this driver, import this crate and an `embedded-hal` implementation,
//! then instantiate the device.
//! 
//! Please see examples folder.

#![deny(missing_docs, rust_2018_idioms, unsafe_code, unused_qualifications, warnings)]
#![no_std]

mod crc;
mod register_access;
mod sps30;
mod types;

/// SPS30 device driver
pub struct Sps30<I2C, D> {
    /// Tbe concrete I2C implementation
    i2c: I2C,
    delay: D,
    address: u8,
}
