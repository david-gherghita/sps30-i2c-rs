/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2C(E),
    /// CRC checksum mismatch
    ChecksumMismatch,
}

/// Measurement results
#[derive(Debug)]
pub struct AirInfo {
    /// Mass Concentration PM1.0 [μg/m³]
    pub mass_pm1_0: f32,
    /// Mass Concentration PM2.5 [μg/m³]
    pub mass_pm2_5: f32,
    /// Mass Concentration PM4.0 [μg/m³]
    pub mass_pm4_0: f32,
    /// Mass Concentration PM10 [μg/m³]
    pub mass_pm10: f32,
    /// Number Concentration PM0.5 [#/cm³]
    pub number_pm0_5: f32,
    /// Number Concentration PM1.0 [#/cm³]
    pub number_pm1_0: f32,
    /// Number Concentration PM2.5 [#/cm³]
    pub number_pm2_5: f32,
    /// Number Concentration PM4.0 [#/cm³]
    pub number_pm4_0: f32,
    /// Number Concentration PM10 [#/cm³]
    pub number_pm10: f32,
    /// Typical Particle Size [μm]
    pub typical_size: f32,
}

/// Device status register bits
/// False is OK, True indicates a problem
pub struct StatusRegisterResult {
    /// Fan speed out of range
    pub speed: bool,
    /// Laser failure
    pub laser: bool,
    /// Fan failure, fan is mechanically blocked or broken
    pub fan: bool,
}