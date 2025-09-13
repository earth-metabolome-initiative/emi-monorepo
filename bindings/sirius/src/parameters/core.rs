use crate::traits::IntoDefault;

/// The possible core settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreV5 {
    /// The maximal mz is in the core parameters
    MaximalMz(f64),
}

impl ToString for CoreV5 {
    fn to_string(&self) -> String {
        match self {
            CoreV5::MaximalMz(maximal_mz) => {
                // SIRIUS 5.x core filter option
                format!("--maxmz={}", maximal_mz)
            }
        }
    }
}

impl IntoDefault for CoreV5 {
    fn into_default(self) -> Self {
        match self {
            CoreV5::MaximalMz(_) => CoreV5::MaximalMz(800.0),
        }
    }
}

/// The possible core settings for SIRIUS 6.x
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreV6 {
    /// The maximal mz is in the core parameters
    MaximalMz(f64),
}

impl ToString for CoreV6 {
    fn to_string(&self) -> String {
        match self {
            CoreV6::MaximalMz(maximal_mz) => {
                // SIRIUS 6.x uses --mzmax as global filter option
                format!("--mzmax={}", maximal_mz)
            }
        }
    }
}

impl IntoDefault for CoreV6 {
    fn into_default(self) -> Self {
        match self {
            CoreV6::MaximalMz(_) => CoreV6::MaximalMz(800.0),
        }
    }
}
