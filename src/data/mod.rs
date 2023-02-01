mod error;
pub use error::MAX31855DataError;
use error::{
    LinearizationSnafu, OpenConnectionSnafu, ShortGroundSnafu, ShortVCCSnafu, UnknownSnafu,
};
use snafu::prelude::*;

/// Represents the data read from the MAX31855
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct MAX31855Data(u32);

impl MAX31855Data {
    pub(crate) fn new(data: u32) -> Self {
        Self(data)
    }
    /// The raw data from MAX31855
    pub fn raw(&self) -> u32 {
        self.0
    }
    /// Gets thermocouple temp in degrees celsius
    pub fn get_thermocouple_temp(&self) -> Result<f32, MAX31855DataError> {
        self.error()?;
        Ok(self.thermocouple_temp())
    }
    /// Gets internal temp in degrees celsius
    pub fn get_internal_temp(&self) -> Result<f32, MAX31855DataError> {
        self.error()?;
        Ok(self.internal_temp())
    }
    fn error(&self) -> Result<(), MAX31855DataError> {
        if (self.0 & (1 << 16)) != 0 {
            if (self.0 & 0b001) != 0 {
                return OpenConnectionSnafu.fail();
            }
            if (self.0 & 0b010) != 0 {
                return ShortGroundSnafu.fail();
            }
            if (self.0 & 0b100) != 0 {
                return ShortVCCSnafu.fail();
            }
            return UnknownSnafu.fail();
        }
        Ok(())
    }
    fn thermocouple_temp(&self) -> f32 {
        let temp = (((self.0 >> 16) & 0b1111_1111_1111_1100) as i16) >> 2;
        (temp as f32) * 0.25
    }
    fn internal_temp(&self) -> f32 {
        let temp = ((self.0 & 0b1111_1111_1111_0000) as i16) >> 4;
        (temp as f32) * 0.0625
    }
    #[cfg(feature = "linearization")]
    /// Gets thermocouple temp in degrees celsius and corrects it using
    /// [NIST Equations](https://srdata.nist.gov/its90/download/type_k.tab) for
    /// better accuracy over an extended range
    pub fn get_linear_temp(&self) -> Result<f32, MAX31855DataError> {
        self.error()?;
        crate::linearization::linearize_temp(self.thermocouple_temp(), self.internal_temp())
            .context(LinearizationSnafu)
    }
}

#[cfg(test)]
mod tests {
    use more_asserts::assert_lt;

    use super::*;
    const LINEARIZE_ERROR_VALUES: [(u32, f32); 2] = [
        (0b10000000000000000001010000110000, -2048f32),
        (0b01111100000000000001010000110000, 1984f32),
    ];

    #[test]
    fn test_linearization_error() {
        for value in LINEARIZE_ERROR_VALUES {
            let data = MAX31855Data(value.0);
            let err = data.get_linear_temp().unwrap_err();
            assert!(
                matches!(err, MAX31855DataError::Linearization { .. }),
                "Linearize should fail for {}",
                value.1
            );
        }
    }

    #[test]
    fn test_high_low_temps_without_linearization() {
        for value in LINEARIZE_ERROR_VALUES {
            let data = MAX31855Data(value.0);
            let val = data.get_thermocouple_temp().unwrap();
            assert_eq!(val, value.1);
        }
    }

    fn test_error(val: u32, fun: impl Fn(MAX31855DataError) -> bool) {
        let data = MAX31855Data(val);
        let err = data.clone().get_thermocouple_temp().unwrap_err();
        assert!(fun(err));
        let err = data.clone().get_linear_temp().unwrap_err();
        assert!(fun(err));
    }

    fn test_not_error(val: u32, fun: impl Fn(MAX31855DataError) -> bool) {
        let data = MAX31855Data(val);
        if let Err(err) = data.clone().get_thermocouple_temp() {
            assert!(!fun(err));
        }
        if let Err(err) = data.clone().get_linear_temp() {
            assert!(!fun(err));
        }
    }

    fn test_bit(id: u8, fun: fn(MAX31855DataError) -> bool) {
        let val = 1u32 << id;
        test_error(val | (1u32 << 16), fun);
        test_not_error(val, |e| !fun(e));
    }

    #[test]
    fn test_open_connection() {
        test_bit(0, |e| matches!(e, MAX31855DataError::OpenConnection));
    }

    #[test]
    fn test_short_gnd() {
        test_bit(1, |e| matches!(e, MAX31855DataError::ShortGround));
    }

    #[test]
    fn test_short_vcc() {
        test_bit(2, |e| matches!(e, MAX31855DataError::ShortVCC));
    }

    #[test]
    fn test_unknown_error() {
        test_bit(17, |e| matches!(e, MAX31855DataError::Unknown));
    }

    #[test]
    fn test_dummy_value() {
        let value = 0b00111100000000000001010000110000;
        let data = MAX31855Data(value);
        assert_lt!(
            (data.get_thermocouple_temp().unwrap() - 960.0f32).abs(),
            0.01f32
        );
        assert_lt!(
            (data.get_internal_temp().unwrap() - 20.1875f32).abs(),
            0.01f32
        );
        assert_lt!((data.get_linear_temp().unwrap() - 957.0f32).abs(), 0.5f32);
    }
}
