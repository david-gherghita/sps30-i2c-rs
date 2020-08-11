# Rust SPS30 Particulate Matter Sensor Driver

[![crates.io](https://img.shields.io/crates/v/sps30-i2c)](https://crates.io/crates/sps30-i2c)
[![Docs](https://docs.rs/sps30-i2c/badge.svg)](https://docs.rs/sps30-i2c)
[![Build Status](https://travis-ci.com/david-gherghita/sps30-i2c-rs.svg?branch=master)](https://travis-ci.com/david-gherghita/sps30-i2c-rs)

This is a platform agnostic Rust driver for the SPS30 particulate matter
sensor using the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

The driver supports only floating point output and is written for the
firmware version 2.1. Although it is fully compatible with older versions
and the 2.2 version, it doesn't fully support the latest.

This driver allows you to:
- Enter measurement mode. See: `start_measurement()`.
- Exit measurement mode. See: `stop_measurement`.
- Poll for the availability of new measurements. See: `read_data_ready_flag()`.
- Read the measured values. See: `read_measured_values()`.
- Enter sleep mode. See: `sleep()`.
- Exit sleep mode. See: `wake_up()`.
- Start the fan-cleaning manually. See: `start_fan_cleaning()`.
- Read the interval[s] of the periodic fan-cleaning. See: `read_auto_cleaning_interval()`.
- Write the interval[s] of the periodic fan-cleaning. See: `write_auto_cleaning_interval()`.
- Read device product type. See: `read_device_product_type()`.
- Read device serial number. See: `read_device_serial_number()`.
- Read firmware version. See: `read_firmware_version()`.
- Read device status register. See: `read_device_status_register()`.
- Clear device status register. See: `clear_device_status_register()`.
- Reset the device. See: `device_reset()`.

## The device

The SPS30 Particulate Matter (PM) sensor is a technological breakthrough
in optical PM sensors. Its measurement principle is based on laser
scattering and makes use of Sensirion's innovative contamination-resistance
technology. This technology, together with high-quality and long-lasting
components, enables precise measurements from its first operation and
throughout its lifetime of more than ten years. In addition, Sensiron's
advanced algorithms provide superior precision for different PM types and
higher-resolution particle size binning, opening up new possibilities for
the detection of different sorts of environmental dust and other particles.

The SPS30 has been designed for use in various applications and devices, such as:
- Air purifiers
- HVAC equipment
- Demand-controlled ventilation systems
- Air conditioners
- Air quality and environmental monitors
- Smart home and IoT devices

Documentation:
- Datasheet: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/9.6_Particulate_Matter/Datasheets/Sensirion_PM_Sensors_SPS30_Datasheet.pdf

## Usage 

To use this driver, import this crate and an `embedded-hal` implementation,
then instantiate the device.

Please see examples folder.

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/david-gherghita/sps30-i2c-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
