use error_stack::{bail, Result};

mod k_type_constants;
use k_type_constants::*;

mod error;
pub use error::LinearizationError;

// Computes coeffs[0] * data^0 + coeffs[1] * data^1 + ... + coeffs[n] * data^n
fn compute_polynomial<const N: usize>(coeffs: &[f32; N], data: f32) -> f32 {
    let c1 = coeffs[0];
    coeffs
        .iter()
        .skip(1)
        .fold((c1, 1f32), |(acc, current_data), c| {
            let new_data = data * current_data;
            (acc + c * new_data, new_data)
        })
        .0
}

#[inline(always)]
fn exp(val: f32) -> f32 {
    cfg_if::cfg_if! {
        if #[cfg(feature = "libm")] {
            libm::expf(val)
        } else if #[cfg(feature = "micromath")] {
            micromath::F32Ext::exp(val)
        } else {
            compile_error!("Either libm or micromath feature should be specified")
        }
    }
}

pub fn linearize_temp(total_temp: f32, ic_temp: f32) -> Result<f32, LinearizationError> {
    // Subtract cold junction temperature from the raw thermocouple temperature and convert to mV.
    let thermocouple_voltage = (total_temp - ic_temp) * 0.041276;

    // Calculate the cold junction equivalent thermocouple voltage.
    let ic_temp_positive = ic_temp > 0f32;

    let inverse_array = if ic_temp_positive {
        &POSITIVE
    } else {
        &NEGATIVE
    };

    let cj_voltage = compute_polynomial(inverse_array, ic_temp);

    let cj_voltage = if ic_temp_positive {
        let c1 = ic_temp - 0.1269686E+03;
        cj_voltage + 0.1185976E+00 * exp(-0.1183432E-03 * c1 * c1)
    } else {
        cj_voltage
    };

    // Add the cold junction equivalent thermocouple voltage calculated in step 3 to the thermocouple voltage
    let total_voltage = thermocouple_voltage + cj_voltage;

    // Use the result of step 4 and the NIST voltage-to-temperature (inverse) coefficients to calculate
    // the cold junction compensated, linearized temperature value.
    let array = if total_voltage < 0f32 {
        if total_voltage < MIN_VOLTAGE {
            bail!(LinearizationError::new_low(total_voltage, total_temp));
        }
        &NEGATIVE_INVERSE
    } else if total_voltage < 20.644f32 {
        &LOW_POSITIVE_INVERSE
    } else if total_voltage < MAX_VOLTAGE {
        &HIGH_POSITIVE_INVERSE
    } else {
        bail!(LinearizationError::new_high(total_voltage, total_temp));
    };

    let corrected_temp = compute_polynomial(array, total_voltage);

    Ok(corrected_temp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_temp_no_change() {
        let result = linearize_temp(0f32, 0f32);
        assert_eq!(result.unwrap(), 0f32);
    }

    #[test]
    fn high_temp_error() {
        const TEMP: f32 = 1350.0;
        let result = linearize_temp(TEMP, 0f32);
        let report = result.unwrap_err();
        let err = report.current_context();
        assert!(matches!(err, LinearizationError::TooHigh { .. }));
    }

    #[test]
    fn low_temp_error() {
        const TEMP: f32 = -200.0;
        let result = linearize_temp(TEMP, 0f32);
        let report = result.unwrap_err();
        let err = report.current_context();
        assert!(matches!(err, LinearizationError::TooLow { .. }));
    }
}
