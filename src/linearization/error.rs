use core::fmt;

use error_stack::Context;

use super::k_type_constants::{MAX_VOLTAGE, MIN_VOLTAGE};

/// Possible linearization errors
#[derive(Debug)]
pub enum LinearizationError {
    /// The temperature of thermocouple after linearization was too high (bigger than 1372 °C)
    TooHigh { voltage: f32, temp: f32 },
    /// The temperature of thermocouple after linearization was too low (bigger than -200 °C)
    TooLow { voltage: f32, temp: f32 },
}

impl LinearizationError {
    pub(super) fn new_high(voltage: f32, temp: f32) -> Self {
        LinearizationError::TooHigh { voltage, temp }
    }
    pub(super) fn new_low(voltage: f32, temp: f32) -> Self {
        LinearizationError::TooLow { voltage, temp }
    }
}

impl fmt::Display for LinearizationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Cannot perform linearization: temp is too ")?;
        let (voltage, temp, desc, val) = match self {
            LinearizationError::TooHigh { voltage, temp } => {
                fmt.write_str("high")?;
                (voltage, temp, "max", MAX_VOLTAGE)
            }
            LinearizationError::TooLow { voltage, temp } => {
                fmt.write_str("low")?;
                (voltage, temp, "min", MIN_VOLTAGE)
            }
        };
        fmt.write_fmt(format_args!(
            ". Got voltage: {voltage}mV ({desc}: {val}mV), non-linearized temp: {temp} °C."
        ))
    }
}

impl Context for LinearizationError {}
