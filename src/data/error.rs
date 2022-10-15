use core::fmt;
use error_stack::Context;

/// MAX31855Data error type
#[derive(Debug)]
pub enum MAX31855DataError {
    /// Internal (currently this can be only linearization) error
    Internal,
    /// Unknown error
    Unknown,
    /// Errors when the thermocouple is open (no connections)
    OpenConnection,
    /// Errors when the thermocouple is short-circuited to Vcc
    ShortVCC,
    /// Errors when the thermocouple is short-circuited to GND
    ShortGround,
}

impl fmt::Display for MAX31855DataError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Cannot convert MAX31855 data")?;
        match self {
            MAX31855DataError::Internal => Ok(()),
            MAX31855DataError::Unknown => fmt.write_str(": unknown error"),
            MAX31855DataError::OpenConnection => fmt.write_str(": thermocouple is open"),
            MAX31855DataError::ShortVCC => {
                fmt.write_str(": thermocouple is short circuited to Vcc")
            }
            MAX31855DataError::ShortGround => {
                fmt.write_str(": thermocouple is short circuited to Ground")
            }
        }
    }
}

impl Context for MAX31855DataError {}
