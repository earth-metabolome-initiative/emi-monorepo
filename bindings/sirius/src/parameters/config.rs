use crate::prelude::*;
use crate::traits::{IntoDefault, NamedParametersSet, Enablable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigV5 {
    Enabled,
    IsotopeSettingsFilter(bool),
    FormulaSearchDB(FormulaSearchDB),
    TimeoutSecondsPerTree(u64),
}

impl ToString for ConfigV5 {
    fn to_string(&self) -> String {
        match self {
            ConfigV5::Enabled => Self::parameter_set_name().to_string(),
            ConfigV5::IsotopeSettingsFilter(isotope_settings_filter) => {
                format!("--IsotopeSettings.filter={}", isotope_settings_filter)
            }
            ConfigV5::FormulaSearchDB(formula_search_db) => {
                format!("--FormulaSearchDB={}", formula_search_db)
            }
            ConfigV5::TimeoutSecondsPerTree(timeout_seconds_per_tree) => {
                format!("--Timeout.secondsPerTree={}", timeout_seconds_per_tree)
            }
        }
    }
}

impl IntoDefault for ConfigV5 {
    fn into_default(self) -> Self {
        match self {
            ConfigV5::Enabled => ConfigV5::Enabled,
            ConfigV5::IsotopeSettingsFilter(_) => {
                ConfigV5::IsotopeSettingsFilter(true)
            }
            ConfigV5::FormulaSearchDB(_) => {
                ConfigV5::FormulaSearchDB(FormulaSearchDB::Bio)
            }
            ConfigV5::TimeoutSecondsPerTree(_) => {
                ConfigV5::TimeoutSecondsPerTree(0)
            }
        }
    }
}

impl Enablable for ConfigV5 {
    fn is_enabler(&self) -> bool {
        match self {
            ConfigV5::Enabled => true,
            _ => false,
        }
    }
    
    fn enabler() -> Self {
        ConfigV5::Enabled
    }
}

impl NamedParametersSet for ConfigV5 {
    fn parameter_set_name() -> &'static str {
        "config"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_isotope_settings_filter() {
        assert_eq!(
            ConfigV5::IsotopeSettingsFilter(true),
            ConfigV5::IsotopeSettingsFilter(false).into_default()
        );
        assert_ne!(
            ConfigV5::IsotopeSettingsFilter(false),
            ConfigV5::IsotopeSettingsFilter(false).into_default()
        );
    }

    #[test]
    fn test_default_formula_search_db() {
        assert_eq!(
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Bio),
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Gnps).into_default()
        );
        assert_ne!(
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Gnps),
            ConfigV5::FormulaSearchDB(FormulaSearchDB::Gnps).into_default()
        );
    }
}
