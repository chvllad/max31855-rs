//! MAX31855 driver with linearization and async embedded_hal support
//! # Example
//! ```ignore
//! use max31855_rs::MAX31855;
//! let mut device = MAX31855::new(spi);
//!
//! // Sync
//! let data = device.read_data()?;
//! // Async
//! let data = device.read_data().await?;
//!
//! let data_raw = data.raw();
//! let temp = data.get_linear_temp()?;
//! ```
//! # Features
//! * `linearization` - enables linearization for K-type thermocouples as described in <https://learn.adafruit.com/calibrating-sensors/maxim-31855-linearization>. Requires either `libm` or `micromath` feature to work.
//! * `hal-async` - enables async spi reading using `embedded-hal-async` interfaces (currently requires nightly compiler)
//! * `hal-block` - enables blocking spi reading using `embedded-hal` interfaces

#![cfg_attr(all(not(test), not(doctest)), no_std)]
#[allow(clippy::excessive_precision)]
#[cfg(feature = "linearization")]
mod linearization;
#[cfg(feature = "linearization")]
pub use linearization::LinearizationError;

mod data;
pub use data::{MAX31855Data, MAX31855DataError};

mod driver;
pub use driver::{MAX31855Error, MAX31855};
