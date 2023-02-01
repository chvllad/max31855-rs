#[cfg(feature = "hal-async")]
mod hal_async;
#[cfg(feature = "hal-async")]
pub use hal_async::MAX31855Error;

#[cfg(feature = "hal-block")]
mod hal_blocking;
#[cfg(feature = "hal-block")]
pub use hal_blocking::MAX31855Error;

mod error;
use error::MAX31855Snafu;

/// Struct for reading from MAX31855
pub struct MAX31855<SPI>(SPI);
