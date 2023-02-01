use snafu::prelude::*;

/// Possible linearization errors
#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum LinearizationError {
    /// The temperature of thermocouple after linearization was too high (bigger than 1372 °C)
    #[snafu(display("temperature of thermocouple may not be higher than 1372 °C, but it was {temp}. Voltage {voltage}"))]
    TooHigh { voltage: f32, temp: f32 },
    /// The temperature of thermocouple after linearization was too low (bigger than -200 °C)
    #[snafu(display("temperature of thermocouple may not be lower than -200 °C, but it was {temp}. Voltage {voltage}"))]
    TooLow { voltage: f32, temp: f32 },
}
