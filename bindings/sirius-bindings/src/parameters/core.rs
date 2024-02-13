use crate::traits::IntoDefault;

/// The possible core settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreV5 {
    /// The maximal mz is in the core parameters
    MaximalMz(f64),

    /// The maximal number of logical cpus to use.
    NCpus(usize),
}

impl ToString for CoreV5 {
    fn to_string(&self) -> String {
        match self {
            CoreV5::MaximalMz(maximal_mz) => {
                format!("--maxmz={}", maximal_mz)
            }
            CoreV5::NCpus(n_cores) => {
                format!("--cores={}", n_cores)
            }
        }
    }
}

impl IntoDefault for CoreV5 {
    fn into_default(self) -> Self {
        match self {
            CoreV5::MaximalMz(_) => CoreV5::MaximalMz(800.0),
            CoreV5::NCpus(_) => CoreV5::NCpus(num_cpus::get()),
        }
    }
}
