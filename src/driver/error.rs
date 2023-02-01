use snafu::prelude::*;

/// MAX31855 error type
#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub(super)))]
#[snafu(display("could not read MAX31855 data: {error}"))]
pub struct MAX31855Error<E: core::fmt::Display> {
    error: E,
}
