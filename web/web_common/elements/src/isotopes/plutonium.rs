//! Isotopes of the element Plutonium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Plutonium
pub enum PlutoniumIsotope {
    /// Isotope Pu228 of Plutonium
    Pu228,
    /// Isotope Pu229 of Plutonium
    Pu229,
    /// Isotope Pu230 of Plutonium
    Pu230,
    /// Isotope Pu231 of Plutonium
    Pu231,
    /// Isotope Pu232 of Plutonium
    Pu232,
    /// Isotope Pu233 of Plutonium
    Pu233,
    /// Isotope Pu234 of Plutonium
    Pu234,
    /// Isotope Pu235 of Plutonium
    Pu235,
    /// Isotope Pu236 of Plutonium
    Pu236,
    /// Isotope Pu237 of Plutonium
    Pu237,
    /// Isotope Pu238 of Plutonium
    Pu238,
    /// Isotope Pu239 of Plutonium
    Pu239,
    /// Isotope Pu240 of Plutonium
    Pu240,
    /// Isotope Pu241 of Plutonium
    Pu241,
    /// Isotope Pu242 of Plutonium
    Pu242,
    /// Isotope Pu243 of Plutonium
    Pu243,
    /// Isotope Pu244 of Plutonium
    Pu244,
    /// Isotope Pu245 of Plutonium
    Pu245,
    /// Isotope Pu246 of Plutonium
    Pu246,
    /// Isotope Pu247 of Plutonium
    Pu247,
}
impl super::RelativeAtomicMass for PlutoniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Pu228 => 228.038732f64,
            Self::Pu229 => 229.040144f64,
            Self::Pu230 => 230.03965f64,
            Self::Pu231 => 231.041102f64,
            Self::Pu232 => 232.041185f64,
            Self::Pu233 => 233.042998f64,
            Self::Pu234 => 234.0433174f64,
            Self::Pu235 => 235.045286f64,
            Self::Pu236 => 236.0460581f64,
            Self::Pu237 => 237.0484098f64,
            Self::Pu238 => 238.0495601f64,
            Self::Pu239 => 239.0521636f64,
            Self::Pu240 => 240.0538138f64,
            Self::Pu241 => 241.0568517f64,
            Self::Pu242 => 242.0587428f64,
            Self::Pu243 => 243.0620036f64,
            Self::Pu244 => 244.0642053f64,
            Self::Pu245 => 245.067826f64,
            Self::Pu246 => 246.070205f64,
            Self::Pu247 => 247.07419f64,
        }
    }
}
impl super::ElementVariant for PlutoniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Pu
    }
}
impl super::MassNumber for PlutoniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Pu228 => 228u16,
            Self::Pu229 => 229u16,
            Self::Pu230 => 230u16,
            Self::Pu231 => 231u16,
            Self::Pu232 => 232u16,
            Self::Pu233 => 233u16,
            Self::Pu234 => 234u16,
            Self::Pu235 => 235u16,
            Self::Pu236 => 236u16,
            Self::Pu237 => 237u16,
            Self::Pu238 => 238u16,
            Self::Pu239 => 239u16,
            Self::Pu240 => 240u16,
            Self::Pu241 => 241u16,
            Self::Pu242 => 242u16,
            Self::Pu243 => 243u16,
            Self::Pu244 => 244u16,
            Self::Pu245 => 245u16,
            Self::Pu246 => 246u16,
            Self::Pu247 => 247u16,
        }
    }
}
impl super::IsotopicComposition for PlutoniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for PlutoniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Pu247
    }
}
impl TryFrom<u16> for PlutoniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            228u16 => Ok(Self::Pu228),
            229u16 => Ok(Self::Pu229),
            230u16 => Ok(Self::Pu230),
            231u16 => Ok(Self::Pu231),
            232u16 => Ok(Self::Pu232),
            233u16 => Ok(Self::Pu233),
            234u16 => Ok(Self::Pu234),
            235u16 => Ok(Self::Pu235),
            236u16 => Ok(Self::Pu236),
            237u16 => Ok(Self::Pu237),
            238u16 => Ok(Self::Pu238),
            239u16 => Ok(Self::Pu239),
            240u16 => Ok(Self::Pu240),
            241u16 => Ok(Self::Pu241),
            242u16 => Ok(Self::Pu242),
            243u16 => Ok(Self::Pu243),
            244u16 => Ok(Self::Pu244),
            245u16 => Ok(Self::Pu245),
            246u16 => Ok(Self::Pu246),
            247u16 => Ok(Self::Pu247),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Pu, value)),
        }
    }
}
impl std::fmt::Display for PlutoniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pu228 => write!(f, "Pu228"),
            Self::Pu229 => write!(f, "Pu229"),
            Self::Pu230 => write!(f, "Pu230"),
            Self::Pu231 => write!(f, "Pu231"),
            Self::Pu232 => write!(f, "Pu232"),
            Self::Pu233 => write!(f, "Pu233"),
            Self::Pu234 => write!(f, "Pu234"),
            Self::Pu235 => write!(f, "Pu235"),
            Self::Pu236 => write!(f, "Pu236"),
            Self::Pu237 => write!(f, "Pu237"),
            Self::Pu238 => write!(f, "Pu238"),
            Self::Pu239 => write!(f, "Pu239"),
            Self::Pu240 => write!(f, "Pu240"),
            Self::Pu241 => write!(f, "Pu241"),
            Self::Pu242 => write!(f, "Pu242"),
            Self::Pu243 => write!(f, "Pu243"),
            Self::Pu244 => write!(f, "Pu244"),
            Self::Pu245 => write!(f, "Pu245"),
            Self::Pu246 => write!(f, "Pu246"),
            Self::Pu247 => write!(f, "Pu247"),
        }
    }
}
