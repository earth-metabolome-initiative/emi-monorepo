use std::fmt::Display;

/// The possible mass deviations
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MassDeviation {
    /// The mass deviation in ppm
    Ppm(f32),

    /// The mass deviation in Da
    Da(f32),
}

impl MassDeviation {
    /// Create a new mass deviation in ppm
    /// # Panics
    /// If the value is negative
    /// # Example
    /// ```
    /// use sirius_bindings::prelude::*;
    /// let ppm = MassDeviation::ppm(10.0);
    /// ```
    /// # Panics
    /// If the value is negative
    /// ```should_panic
    /// use sirius_bindings::prelude::*;
    /// let ppm = MassDeviation::ppm(-10.0);
    /// ```
    pub fn ppm(value: f32) -> Self {
        // ppm can't be negative
        if value < 0.0 {
            panic!("ppm value can't be negative");
        }
        MassDeviation::Ppm(value)
    }

    /// Create a new mass deviation in Da
    /// # Panics
    /// If the value is negative
    /// # Example
    /// ```
    /// use sirius_bindings::prelude::*;
    /// let da = MassDeviation::da(0.1);
    /// ```
    /// # Panics
    /// If the value is negative
    /// ```should_panic
    /// use sirius_bindings::prelude::*;
    /// let x = MassDeviation::da(-0.1);
    /// ```
    pub fn da(value: f32) -> Self {
        // Da can't be negative
        if value < 0.0 {
            panic!("Da value can't be negative");
        }
        MassDeviation::Da(value)
    }

    /// Check if the value is positive
    /// # Errors
    /// If the value is negative
    /// # Example
    /// ```
    /// use sirius_bindings::prelude::*;
    /// let ppm = MassDeviation::ppm(10.0);
    /// assert_eq!(ppm.must_be_positive().unwrap(), MassDeviation::Ppm(10.0));
    /// ```
    /// # Errors
    /// If the value is negative
    /// ```
    /// use sirius_bindings::prelude::*;
    /// let ppm = MassDeviation::Ppm(-10.0);
    /// assert!(ppm.must_be_positive().is_err());
    /// ```
    pub fn must_be_positive(&self) -> Result<Self, String> {
        match self {
            MassDeviation::Ppm(value) => {
                if *value < 0.0 {
                    Err("ppm value can't be negative".to_string())
                } else {
                    Ok(*self)
                }
            }
            MassDeviation::Da(value) => {
                if *value < 0.0 {
                    Err("Da value can't be negative".to_string())
                } else {
                    Ok(*self)
                }
            }
        }
    }
}

impl Display for MassDeviation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MassDeviation::Ppm(value) => write!(f, "{} ppm", value),
            MassDeviation::Da(value) => write!(f, "{} Da", value),
        }
    }
}

impl<'a> TryFrom<&'a str> for MassDeviation {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut split = s.split_whitespace();
        let value = split
            .next()
            .ok_or_else(|| "No value provided".to_string())?;
        let unit = split.next().ok_or_else(|| "No unit provided".to_string())?;

        match unit {
            "ppm" => Ok(MassDeviation::Ppm(
                value
                    .parse()
                    .map_err(|e| format!("Cannot parse value: {}", e))?,
            )),
            "Da" => Ok(MassDeviation::Da(
                value
                    .parse()
                    .map_err(|e| format!("Cannot parse value: {}", e))?,
            )),
            _ => Err(format!("Unknown unit: {}", unit)),
        }
    }
}

impl TryFrom<String> for MassDeviation {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        MassDeviation::try_from(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mass_deviation_display() {
        let ppm = MassDeviation::Ppm(10.0);
        assert_eq!(format!("{}", ppm), "10 ppm");

        let da = MassDeviation::Da(0.1);
        assert_eq!(format!("{}", da), "0.1 Da");
    }
    #[test]
    fn test_error_if_negative_value_with_funtion_call() {
        let error = std::panic::catch_unwind(|| MassDeviation::ppm(-10.0)).unwrap_err();
        assert_eq!(
            error.downcast_ref::<&str>(),
            Some(&"ppm value can't be negative")
        );

        let error = std::panic::catch_unwind(|| MassDeviation::da(-0.1)).unwrap_err();
        assert_eq!(
            error.downcast_ref::<&str>(),
            Some(&"Da value can't be negative")
        );
    }

    #[test]
    #[should_panic]
    fn test_error_if_negative_value_with_enum() {
        let _ = MassDeviation::Ppm(-10.0).must_be_positive().unwrap();
    }
}
