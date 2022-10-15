use core::fmt;
use error_stack::Context;

/// MAX31855 error type
#[derive(Debug)]
pub struct MAX31855Error;

impl fmt::Display for MAX31855Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Cannot read MAX31855 data")
    }
}

impl Context for MAX31855Error {}
