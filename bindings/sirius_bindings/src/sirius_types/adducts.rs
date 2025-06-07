use std::fmt::Display;

/// The possible adducts
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Adducts {
    /// \[M+H\]+
    MplusHplus,

    /// \[M+K\]+
    MplusKplus,
    /// \[M+Na\]+
    MplusNaplus,

    /// \[M+NH4\]+
    MplusNH4plus,

    /// \[M+Cl\]-
    MplusClminus,

    /// \[M\]+
    Mplus,

    /// \[M+H-H2O\]+
    MplusHminusH2Oplus,

    /// \[M+H3N+H\]+
    MplusH3NplusHplus,

    /// \[M+H2O+H\]+
    MplusH2OplusHplus,

    /// \[M+CH4O+H\]+
    MplusCH4OplusHplus,

    /// \[M+C2H3N+H\]+
    MplusC2H3NplusHplus,

    /// \[M+C4H6N2+H\]+
    MplusC4H6N2plusHplus,

    /// \[M+C3H8O+H\]+
    MplusC3H8OplusHplus,

    /// \[M+C2H6OS+H\]+
    MplusC2H6OSplusHplus,

    /// \[M-H+K+K\]+
    MminusHplusKKplus,

    /// \[M-H+Na+Na\]+
    MminusHplusNaNaplus,

    /// \[M+C2H3N+Na\]+
    MplusC2H3NplusNaplus,

    /// \[M-H\]-
    MminusHminus,

    /// \[M+H-H4O2\]+
    MplusHminusTwoH2Oplus,

    /// \[M-H2O-H\]-
    MminusH20minusHminus,

    /// \[M+Br\]-
    MplusBromideminus,
}

impl Display for Adducts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Adducts::MplusHplus => write!(f, "[M+H]+"),
            Adducts::MplusKplus => write!(f, "[M+K]+"),
            Adducts::MplusNaplus => write!(f, "[M+Na]+"),
            Adducts::MplusNH4plus => write!(f, "[M+NH4]+"),
            Adducts::MplusClminus => write!(f, "[M+Cl]-"),
            Adducts::Mplus => write!(f, "[M]+"),
            Adducts::MplusHminusH2Oplus => write!(f, "[M+H-H2O]+"),
            Adducts::MplusH3NplusHplus => write!(f, "[M+H3N+H]+"),
            Adducts::MplusH2OplusHplus => write!(f, "[M+H2O+H]+"),
            Adducts::MplusCH4OplusHplus => write!(f, "[M+CH4O+H]+"),
            Adducts::MplusC2H3NplusHplus => write!(f, "[M+C2H3N+H]+"),
            Adducts::MplusC4H6N2plusHplus => write!(f, "[M+C4H6N2+H]+"),
            Adducts::MplusC3H8OplusHplus => write!(f, "[M+C3H8O+H]+"),
            Adducts::MplusC2H6OSplusHplus => write!(f, "[M+C2H6OS+H]+"),
            Adducts::MminusHplusKKplus => write!(f, "[M-H+K+K]+"),
            Adducts::MminusHplusNaNaplus => write!(f, "[M-H+Na+Na]+"),
            Adducts::MplusC2H3NplusNaplus => write!(f, "[M+C2H3N+Na]+"),
            Adducts::MminusHminus => write!(f, "[M-H]-"),
            Adducts::MplusHminusTwoH2Oplus => write!(f, "[M+H-H4O2]+"),
            Adducts::MminusH20minusHminus => write!(f, "[M-H2O-H]-"),
            Adducts::MplusBromideminus => write!(f, "[M+Br]-"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Adducts {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "[M+H]+" => Ok(Adducts::MplusHplus),
            "[M+K]+" => Ok(Adducts::MplusKplus),
            "[M+Na]+" => Ok(Adducts::MplusNaplus),
            "[M+NH4]+" => Ok(Adducts::MplusNH4plus),
            "[M+Cl]-" => Ok(Adducts::MplusClminus),
            "[M]+" => Ok(Adducts::Mplus),
            "[M+H-H2O]+" => Ok(Adducts::MplusHminusH2Oplus),
            "[M+H3N+H]+" => Ok(Adducts::MplusH3NplusHplus),
            "[M+H2O+H]+" => Ok(Adducts::MplusH2OplusHplus),
            "[M+CH4O+H]+" => Ok(Adducts::MplusCH4OplusHplus),
            "[M+C2H3N+H]+" => Ok(Adducts::MplusC2H3NplusHplus),
            "[M+C4H6N2+H]+" => Ok(Adducts::MplusC4H6N2plusHplus),
            "[M+C3H8O+H]+" => Ok(Adducts::MplusC3H8OplusHplus),
            "[M+C2H6OS+H]+" => Ok(Adducts::MplusC2H6OSplusHplus),
            "[M-H+K+K]+" => Ok(Adducts::MminusHplusKKplus),
            "[M-H+Na+Na]+" => Ok(Adducts::MminusHplusNaNaplus),
            "[M+C2H3N+Na]+" => Ok(Adducts::MplusC2H3NplusNaplus),
            "[M-H]-" => Ok(Adducts::MminusHminus),
            "[M+H-H4O2]+" => Ok(Adducts::MplusHminusTwoH2Oplus),
            "[M-H2O-H]-" => Ok(Adducts::MminusH20minusHminus),
            "[M+Br]-" => Ok(Adducts::MplusBromideminus),
            _ => Err(format!("Unknown adduct: {s}")),
        }
    }
}

impl TryFrom<String> for Adducts {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Adducts::try_from(s.as_str())
    }
}

/// Creates a vector of adducts
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AdductsVector(Vec<Adducts>);

impl Display for AdductsVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut adducts_string = self.0.iter();
        if let Some(adduct) = adducts_string.next() {
            write!(f, "{adduct}")?;
            for adduct in adducts_string {
                write!(f, ",{adduct}")?;
            }
        }
        Ok(())
    }
}

impl From<Vec<Adducts>> for AdductsVector {
    fn from(adducts: Vec<Adducts>) -> Self {
        AdductsVector(adducts)
    }
}

impl<'a> TryFrom<&'a str> for AdductsVector {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let adducts = s
            .split(',')
            .map(|adduct| {
                Adducts::try_from(adduct)
                    .map_err(|e| format!("Cannot parse adduct: {adduct} ({e}). Maybe forgot to put a comma between adducts ?"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AdductsVector(adducts))
    }
}

impl TryFrom<String> for AdductsVector {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        AdductsVector::try_from(s.as_str())
    }
}
