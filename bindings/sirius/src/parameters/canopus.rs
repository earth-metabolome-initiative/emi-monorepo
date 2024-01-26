use crate::traits::IntoDefault;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanopusV5 {
    Enabled,
    Version,
    Help
}

impl ToString for CanopusV5 {
    fn to_string(&self) -> String {
        match self {
            CanopusV5::Enabled => "".to_string(),
            CanopusV5::Help => "--help".to_string(),
            CanopusV5::Version => "--version".to_string()
        }
    }
}

impl IntoDefault for CanopusV5 {
    fn into_default(self) -> Self {
        match self {
            CanopusV5::Enabled => CanopusV5::Enabled,
            CanopusV5::Help => CanopusV5::Help,
            CanopusV5::Version => CanopusV5::Version
        }
    }
}