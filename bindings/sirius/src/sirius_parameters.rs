use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SiriusParameters {
    MaximalMz(f64),
    IsotopeSettingsFilter(bool),
    FormulaSearchDB(FormulaSearchDB),
    TimeoutSecondsPerTree(u64),
    // FormulaSettingsEnforced(FormulaSettingsEnforced),
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
            SiriusParameters::FormulaSearchDB(formula_search_db) => {
                format!("--FormulaSearchDB={}", formula_search_db)
            }
            SiriusParameters::TimeoutSecondsPerTree(timeout_seconds_per_tree) => {
                format!("--Timeout.secondsPerTree={}", timeout_seconds_per_tree)
            } // SiriusParameters::FormulaSettingsEnforced(formula_settings_enforced) => {
              //     format!(
              //         "--FormulaSettings.enforced={}",
              //         formula_settings_enforced.to_string()
              //     )
              // }
        }
    }
}

impl SiriusParameters {
    pub fn to_default(self) -> Self {
        match self {
            SiriusParameters::MaximalMz(_) => SiriusParameters::MaximalMz(800.0),
            SiriusParameters::IsotopeSettingsFilter(_) => {
                SiriusParameters::IsotopeSettingsFilter(true)
            }
            SiriusParameters::FormulaSearchDB(_) => {
                SiriusParameters::FormulaSearchDB(FormulaSearchDB::Bio)
            }
            SiriusParameters::TimeoutSecondsPerTree(_) => {
                SiriusParameters::TimeoutSecondsPerTree(0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_maximal_mz() {
        assert_eq!(
            SiriusParameters::MaximalMz(800.0),
            SiriusParameters::MaximalMz(5858.0).to_default()
        );
        assert_ne!(
            SiriusParameters::MaximalMz(5858.0),
            SiriusParameters::MaximalMz(5858.0).to_default()
        );
    }

    #[test]
    fn test_default_isotope_settings_filter() {
        assert_eq!(
            SiriusParameters::IsotopeSettingsFilter(true),
            SiriusParameters::IsotopeSettingsFilter(false).to_default()
        );
        assert_ne!(
            SiriusParameters::IsotopeSettingsFilter(false),
            SiriusParameters::IsotopeSettingsFilter(false).to_default()
        );
    }
}
