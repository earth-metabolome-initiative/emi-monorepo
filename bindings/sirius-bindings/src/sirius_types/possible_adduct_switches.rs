use std::fmt::Display;

/// The possible adduct switches. For now only the default adducts switches are
/// supported.
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum PossibleAdductSwitches {
    /// The default adducts switches.
    #[default]
    DefaultAdductsSwitches,
}

impl Display for PossibleAdductSwitches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PossibleAdductSwitches::DefaultAdductsSwitches => {
                write!(f, "[M+Na]+:[M+H]+,[M+K]+:[M+H]+,[M+Cl]-:[M-H]-")
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for PossibleAdductSwitches {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "[M+Na]+:[M+H]+,[M+K]+:[M+H]+,[M+Cl]-:[M-H]-" => {
                Ok(PossibleAdductSwitches::DefaultAdductsSwitches)
            }
            _ => Err(format!("Unknown adduct settings enforced: {}", s)),
        }
    }
}

impl TryFrom<String> for PossibleAdductSwitches {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        PossibleAdductSwitches::try_from(s.as_str())
    }
}
