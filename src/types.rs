use crate::hw_def::*;

/// HDC302x(-Q1) device driver
#[derive(Debug)]
pub struct Hdc302x<I2C, Delay> {
    pub(crate) i2c: I2C,
    pub(crate) delay: Delay,
    pub(crate) i2c_addr: crate::hw_def::I2cAddr,
}

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C communication error
    I2c(E),
    /// Invalid input data provided
    InvalidInputData,
    /// Failure of a checksum from the device was detected
    CrcMismatch,
}

/// Raw (still in u16 format) temperature and/or humidity from the device
#[derive(Debug)]
pub enum RawDatum {
    /// temerature and relative humidity from one-shot or auto mode
    TempAndRH(RawTempAndRH),
    /// minimum temperature since auto mode was enabled
    MinTemp(u16),
    /// maximum temperature since auto mode was enabled
    MaxTemp(u16),
    /// minimum relative humidity since auto mode was enabled
    MinRH(u16),
    /// maximum relative humidity since auto mode was enabled
    MaxRH(u16),
}
/// Raw (still in u16 format) temperature and relative humidity from the device
#[derive(Debug)]
pub struct RawTempAndRH{
    /// unprocessed temperature
    pub temperature: u16,
    /// unprocessed relative humiodity
    pub humidity: u16,
}

/// Temperature and/or humidity from the device after conversion
#[derive(Debug)]
pub enum Datum {
    /// temerature and relative humidity from one-shot or auto mode
    TempAndRH(TempAndRH),
    /// minimum temperature since auto mode was enabled
    MinTemp(Temp),
    /// maximum temperature since auto mode was enabled
    MaxTemp(Temp),
    /// minimum relative humidity since auto mode was enabled
    MinRH(f32),
    /// maximum relative humidity since auto mode was enabled
    MaxRH(f32),
}
impl From<&RawDatum> for Datum {
    fn from(raw: &RawDatum) -> Self {
        match raw {
            RawDatum::TempAndRH(raw) => Datum::TempAndRH(raw.into()),
            RawDatum::MinTemp(raw) => Datum::MinTemp((*raw).into()),
            RawDatum::MaxTemp(raw) => Datum::MaxTemp((*raw).into()),
            RawDatum::MinRH(raw) => Datum::MinRH((*raw).into()),
            RawDatum::MaxRH(raw) => Datum::MaxRH((*raw).into()),
        }
    }
}

/// Temperature and relative humidity from the device after conversion
#[derive(Debug)]
pub struct TempAndRH {
    /// degrees centigrade
    pub centigrade: f32,
    /// degrees fahrenheit
    pub fahrenheit: f32,
    /// relative humidity in percent
    pub humidity_percent: f32,
}
impl From<&RawTempAndRH> for TempAndRH {
    fn from(raw: &RawTempAndRH) -> Self {
        Self {
            centigrade: -45.0 + (175.0 * (raw.temperature as f32) / 65535.0),
            fahrenheit: -49.0 + (315.0 * (raw.temperature as f32) / 65535.0),
            humidity_percent: 100.0 * (raw.humidity as f32) / 65535.0,
        }
    }
}
/// Temperature after conversion
#[derive(Debug)]
pub struct Temp{
    /// degrees centigrade
    pub centigrade: f32,
    /// degrees fahrenheit
    pub fahrenheit: f32,
}
impl From<u16> for Temp {
    fn from(raw: u16) -> Self {
        Self {
            centigrade: -45.0 + (175.0 * (raw as f32) / 65535.0),
            fahrenheit: -49.0 + (315.0 * (raw as f32) / 65535.0),
        }
    }
}

/// Status bits from the device
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusBits {
    /// at least one alert is active
    pub at_least_one_alert: bool,
    /// heater is enabled
    pub heater_enabled: bool,
    /// relative humidity tracking alert
    pub rh_tracking_alert: bool,
    /// temperature tracking alert
    pub t_tracking_alert: bool,
    /// relative humidity high tracking alert
    pub rh_high_tracking_alert: bool,
    /// relative humidity low tracking alert
    pub rh_low_tracking_alert: bool,
    /// temperature high tracking alert
    pub t_high_tracking_alert: bool,
    /// temperature low tracking alert
    pub t_low_tracking_alert: bool,
    /// reset (power-on or software) detected since last clear of status register
    pub reset_since_clear: bool,
    /// failure of a checksum from the driver was detected
    pub checksum_failure: bool,
}
impl From<u16> for StatusBits {
    fn from(raw: u16) -> Self {
        Self {
            at_least_one_alert: (raw >> STATUS_FIELD_LSBIT_AT_LEAST_ONE_ALERT) & ((1 << STATUS_FIELD_WIDTH_AT_LEAST_ONE_ALERT) - 1) != 0,
            heater_enabled: (raw >> STATUS_FIELD_LSBIT_HEATER_ENABLED) & ((1 << STATUS_FIELD_WIDTH_HEATER_ENABLED) - 1) != 0,
            rh_tracking_alert: (raw >> STATUS_FIELD_LSBIT_RH_TRACKING_ALERT) & ((1 << STATUS_FIELD_WIDTH_RH_TRACKING_ALERT) - 1) != 0,
            t_tracking_alert: (raw >> STATUS_FIELD_LSBIT_T_TRACKING_ALERT) & ((1 << STATUS_FIELD_WIDTH_T_TRACKING_ALERT) - 1) != 0,
            rh_high_tracking_alert: (raw >> STATUS_FIELD_LSBIT_RH_HIGH_TRACKING_ALERT) & ((1 << STATUS_FIELD_WIDTH_RH_HIGH_TRACKING_ALERT) - 1) != 0,
            rh_low_tracking_alert: (raw >> STATUS_FIELD_LSBIT_RH_LOW_TRACKING_ALERT) & ((1 << STATUS_FIELD_WIDTH_RH_LOW_TRACKING_ALERT) - 1) != 0,
            t_high_tracking_alert: (raw >> STATUS_FIELD_LSBIT_T_HIGH_TRACKING_ALERT) & ((1 << STATUS_FIELD_WIDTH_T_HIGH_TRACKING_ALERT) - 1) != 0,
            t_low_tracking_alert: (raw >> STATUS_FIELD_LSBIT_T_LOW_TRACKING_ALERT) & ((1 << STATUS_FIELD_WIDTH_T_LOW_TRACKING_ALERT) - 1) != 0,
            reset_since_clear: (raw >> STATUS_FIELD_LSBIT_RESET_SINCE_CLEAR) & ((1 << STATUS_FIELD_WIDTH_RESET_SINCE_CLEAR) - 1) != 0,
            checksum_failure: (raw >> STATUS_FIELD_LSBIT_CHECKSUM_FAILURE) & ((1 << STATUS_FIELD_WIDTH_CHECKSUM_FAILURE) - 1) != 0,
        }
    }
}
