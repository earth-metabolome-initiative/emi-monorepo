use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MassDeviation {
    Ppm(f32),
    Da(f32),
}

impl MassDeviation {
    pub fn ppm(value: f32) -> Self {
        MassDeviation::Ppm(value)
    }

    pub fn da(value: f32) -> Self {
        MassDeviation::Da(value)
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
