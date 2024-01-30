use crate::traits::IntoDefault;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreV5 {
    MaximalMz(f64),
}

impl ToString for CoreV5 {
    fn to_string(&self) -> String {
        match self {
            CoreV5::MaximalMz(maximal_mz) => {
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
