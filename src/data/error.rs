use snafu::prelude::*;

/// MAX31855Data error type
#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum MAX31855DataError {
    #[cfg(feature = "linearization")]
    /// Linearization error
    #[snafu(display("linearization error"))]
    Linearization { source: crate::LinearizationError },
    /// Unknown error
    #[snafu(display("unknown error"))]
    Unknown,
    /// Errors when the thermocouple is open (no connections)
    #[snafu(display("thermocouple is open (no connections)"))]
    OpenConnection,
    /// Errors when the thermocouple is short-circuited to Vcc
    #[snafu(display("thermocouple is short circuited to Vcc"))]
    ShortVCC,
    /// Errors when the thermocouple is short-circuited to GND
    #[snafu(display("thermocouple is short circuited to Ground"))]
    ShortGround,
}
