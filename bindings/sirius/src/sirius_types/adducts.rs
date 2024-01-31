use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Adducts {
    #[default]
    H,
    K,
    Na,
    Nh4,
    Cl,
    Mplus,
    MPlusHMinusH2O,
    MPlusH3NPlusH,
    MPlusH2OPlusH,
    MPlusCH4OPlusH,
    MPlusC2H3NPlusH,
    MPlusC4H6N2PlusH,
    MPlusC3H8OPlusH,
    MPlusC2H6OSPlusH,
    MMinusHPlusKK,
    MMinusHPlusNaNa,
    MPlusC2H3NPlusNa,
    MMinusH,
    MPlusHMinusTwoH2O,
    MMinusH20MinusH,
    MPlusBromide,
}

impl Display for Adducts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Adducts::H => write!(f, "[M+H]+"),
            Adducts::K => write!(f, "[M+K]+"),
            Adducts::Na => write!(f, "[M+Na]+"),
            Adducts::Nh4 => write!(f, "[M+NH4]+"),
            Adducts::Cl => write!(f, "[M+Cl]-"),
            Adducts::Mplus => write!(f, "[M]+"),
            Adducts::MPlusHMinusH2O => write!(f, "[M+H-H2O]+"),
            Adducts::MPlusH3NPlusH => write!(f, "[M+H3N+H]+"),
            Adducts::MPlusH2OPlusH => write!(f, "[M+H2O+H]+"),
            Adducts::MPlusCH4OPlusH => write!(f, "[M+CH4O+H]+"),
            Adducts::MPlusC2H3NPlusH => write!(f, "[M+C2H3N+H]+"),
            Adducts::MPlusC4H6N2PlusH => write!(f, "[M+C4H6N2+H]+"),
            Adducts::MPlusC3H8OPlusH => write!(f, "[M+C3H8O+H]+"),
            Adducts::MPlusC2H6OSPlusH => write!(f, "[M+C2H6OS+H]+"),
            Adducts::MMinusHPlusKK => write!(f, "[M-H+K+K]+"),
            Adducts::MMinusHPlusNaNa => write!(f, "[M-H+Na+Na]+"),
            Adducts::MPlusC2H3NPlusNa => write!(f, "[M+C2H3N+Na]+"),
            Adducts::MMinusH => write!(f, "[M-H]-"),
            Adducts::MPlusHMinusTwoH2O => write!(f, "[M+H-H4O2]+"),
            Adducts::MMinusH20MinusH => write!(f, "[M-H2O-H]-"),
            Adducts::MPlusBromide => write!(f, "[M+Br]-"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Adducts {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "[M+H]+" => Ok(Adducts::H),
            "[M+K]+" => Ok(Adducts::K),
            "[M+Na]+" => Ok(Adducts::Na),
            "[M+NH4]+" => Ok(Adducts::Nh4),
            "[M+Cl]-" => Ok(Adducts::Cl),
            "[M]+" => Ok(Adducts::Mplus),
            "[M+H-H2O]+" => Ok(Adducts::MPlusHMinusH2O),
            "[M+H3N+H]+" => Ok(Adducts::MPlusH3NPlusH),
            "[M+H2O+H]+" => Ok(Adducts::MPlusH2OPlusH),
            "[M+CH4O+H]+" => Ok(Adducts::MPlusCH4OPlusH),
            "[M+C2H3N+H]+" => Ok(Adducts::MPlusC2H3NPlusH),
            "[M+C4H6N2+H]+" => Ok(Adducts::MPlusC4H6N2PlusH),
            "[M+C3H8O+H]+" => Ok(Adducts::MPlusC3H8OPlusH),
            "[M+C2H6OS+H]+" => Ok(Adducts::MPlusC2H6OSPlusH),
            "[M-H+K+K]+" => Ok(Adducts::MMinusHPlusKK),
            "[M-H+Na+Na]+" => Ok(Adducts::MMinusHPlusNaNa),
            "[M+C2H3N+Na]+" => Ok(Adducts::MPlusC2H3NPlusNa),
            "[M-H]-" => Ok(Adducts::MMinusH),
            "[M+H-H4O2]+" => Ok(Adducts::MPlusHMinusTwoH2O),
            "[M-H2O-H]-" => Ok(Adducts::MMinusH20MinusH),
            "[M+Br]-" => Ok(Adducts::MPlusBromide),
            _ => Err(format!("Unknown adduct: {}", s)),
        }
    }
}

impl TryFrom<String> for Adducts {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Adducts::try_from(s.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AdductsVector(Vec<Adducts>);
impl Display for AdductsVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut adducts_string = self.0.iter();
        if let Some(adduct) = adducts_string.next() {
            write!(f, "{}", adduct)?;
            for adduct in adducts_string {
                write!(f, ",{}", adduct)?;
            }
        }
        Ok(())
    }
}

impl AdductsVector {
    pub fn new(adducts: Vec<Adducts>) -> Self {
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
                    .map_err(|e| format!("Cannot parse adduct: {} ({}). Maybe forgot to put a comma between adducts ?", adduct, e))
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
