#[derive(Debug, Clone, Copy)]
pub(crate) enum SiriusParameters {
    MaximalMz(f64),
    IsotopeSettingsFilter(bool),
}

impl ToString for SiriusParameters {
    fn to_string(&self) -> String {
        match self {
            SiriusParameters::MaximalMz(maximal_mz) => {
                format!("--maxmz={}", maximal_mz)
            }
            SiriusParameters::IsotopeSettingsFilter(isotope_settings_filter) => {
                format!("--IsotopeSettings.filter={}", isotope_settings_filter)
            }
        }
    }
}

impl SiriusParameters {
    pub fn get_default(self) -> Self {
        match self {
            SiriusParameters::MaximalMz(_) => SiriusParameters::MaximalMz(800.0),
            SiriusParameters::IsotopeSettingsFilter(_) => SiriusParameters::IsotopeSettingsFilter(true),
        }
    }
}