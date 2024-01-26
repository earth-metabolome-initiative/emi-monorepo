use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SiriusParametersVersion5 {
    MaximalMz(f64),
    IsotopeSettingsFilter(bool),
    FormulaSearchDB(FormulaSearchDB),
    TimeoutSecondsPerTree(u64),
    // FormulaSettingsEnforced(FormulaSettingsEnforced),
}

impl ToString for SiriusParametersVersion5 {
    fn to_string(&self) -> String {
        match self {
            SiriusParametersVersion5::MaximalMz(maximal_mz) => {
                format!("--maxmz={}", maximal_mz)
            }
            SiriusParametersVersion5::IsotopeSettingsFilter(isotope_settings_filter) => {
                format!("--IsotopeSettings.filter={}", isotope_settings_filter)
            }
            SiriusParametersVersion5::FormulaSearchDB(formula_search_db) => {
                format!("--FormulaSearchDB={}", formula_search_db)
            }
            SiriusParametersVersion5::TimeoutSecondsPerTree(timeout_seconds_per_tree) => {
                format!("--Timeout.secondsPerTree={}", timeout_seconds_per_tree)
            } // SiriusParametersVersion5::FormulaSettingsEnforced(formula_settings_enforced) => {
              //     format!(
              //         "--FormulaSettings.enforced={}",
              //         formula_settings_enforced.to_string()
              //     )
              // }
        }
    }
}

impl SiriusParametersVersion5 {
    pub fn to_default(self) -> Self {
        match self {
            SiriusParametersVersion5::MaximalMz(_) => SiriusParametersVersion5::MaximalMz(800.0),
            SiriusParametersVersion5::IsotopeSettingsFilter(_) => {
                SiriusParametersVersion5::IsotopeSettingsFilter(true)
            }
            SiriusParametersVersion5::FormulaSearchDB(_) => {
                SiriusParametersVersion5::FormulaSearchDB(FormulaSearchDB::Bio)
            }
            SiriusParametersVersion5::TimeoutSecondsPerTree(_) => {
                SiriusParametersVersion5::TimeoutSecondsPerTree(0)
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
            SiriusParametersVersion5::MaximalMz(800.0),
            SiriusParametersVersion5::MaximalMz(5858.0).to_default()
        );
        assert_ne!(
            SiriusParametersVersion5::MaximalMz(5858.0),
            SiriusParametersVersion5::MaximalMz(5858.0).to_default()
        );
    }

    #[test]
    fn test_default_isotope_settings_filter() {
        assert_eq!(
            SiriusParametersVersion5::IsotopeSettingsFilter(true),
            SiriusParametersVersion5::IsotopeSettingsFilter(false).to_default()
        );
        assert_ne!(
            SiriusParametersVersion5::IsotopeSettingsFilter(false),
            SiriusParametersVersion5::IsotopeSettingsFilter(false).to_default()
        );
    }

    #[test]
    fn test_default_formula_search_db() {
        assert_eq!(
            SiriusParametersVersion5::FormulaSearchDB(FormulaSearchDB::Bio),
            SiriusParametersVersion5::FormulaSearchDB(FormulaSearchDB::Gnps).to_default()
        );
        assert_ne!(
            SiriusParametersVersion5::FormulaSearchDB(FormulaSearchDB::Gnps),
            SiriusParametersVersion5::FormulaSearchDB(FormulaSearchDB::Gnps).to_default()
        );
    }
}
