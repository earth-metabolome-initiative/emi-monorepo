use std::fmt::Display;

use crate::traits::IntoDefault;

/// The possible core settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreV5 {
    /// The maximal mz is in the core parameters
    MaximalMz(f64),

    /// The maximal number of logical cpus to use.
    NCPUs(usize),

    /// Recompute results of ALL tools where results are
    /// already present. Per default already present
    /// results will be preserved and the instance will
    /// be skipped for the corresponding Task/Tool
    Recompute(bool),
}

impl Display for CoreV5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreV5::MaximalMz(maximal_mz) => {
                write!(f, "--maxmz={maximal_mz}",)
            }
            CoreV5::NCPUs(n_cores) => {
                write!(f, "--cores={n_cores}",)
            }
            CoreV5::Recompute(recompute) => {
                write!(f, "--recompute={recompute}",)
            }
        }
    }
}

impl IntoDefault for CoreV5 {
    fn into_default(self) -> Self {
        match self {
            CoreV5::MaximalMz(_) => CoreV5::MaximalMz(800.0),
            CoreV5::NCPUs(_) => CoreV5::NCPUs(num_cpus::get()),
            CoreV5::Recompute(_) => CoreV5::Recompute(false),
        }
    }
}
