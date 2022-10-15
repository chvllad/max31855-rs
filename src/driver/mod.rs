#[cfg(feature = "hal-async")]
mod hal_async;

#[cfg(feature = "hal-block")]
mod hal_blocking;

mod error;
pub use error::MAX31855Error;

/// Struct for reading from MAX31855
pub struct MAX31855<SPI>(SPI);
