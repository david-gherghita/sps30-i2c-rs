use linux_embedded_hal::{Delay, I2cdev};
use sps30_i2c::Sps30;
use std::{thread, time};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let delay = Delay;
    let mut sensor = Sps30::new_sps30(dev, delay);

    sensor.wake_up().unwrap();

    println!("{:X?}", sensor.read_device_product_type().unwrap());
    println!("{:X?}", sensor.read_device_serial_number().unwrap());
    println!("{:?}", sensor.read_firmware_version().unwrap());
    println!("{}", sensor.read_device_status_register().unwrap().speed);
    println!("{}", sensor.read_auto_cleaning_interval().unwrap());

    sensor.start_measurement().unwrap();
    for _ in 0..20 {
        println!("{:?}", sensor.read_measured_values().unwrap());
        thread::sleep(time::Duration::from_secs(1));
    }
    sensor.stop_measurement().unwrap();

    sensor.sleep().unwrap();
}