#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum PlutoniumIsotope {
    Pu228,
    Pu229,
    Pu230,
    Pu231,
    Pu232,
    Pu233,
    Pu234,
    Pu235,
    Pu236,
    Pu237,
    Pu238,
    Pu239,
    Pu240,
    Pu241,
    Pu242,
    Pu243,
    Pu244,
    Pu245,
    Pu246,
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
        match self {
            Self::Pu228 => None,
            Self::Pu229 => None,
            Self::Pu230 => None,
            Self::Pu231 => None,
            Self::Pu232 => None,
            Self::Pu233 => None,
            Self::Pu234 => None,
            Self::Pu235 => None,
            Self::Pu236 => None,
            Self::Pu237 => None,
            Self::Pu238 => None,
            Self::Pu239 => None,
            Self::Pu240 => None,
            Self::Pu241 => None,
            Self::Pu242 => None,
            Self::Pu243 => None,
            Self::Pu244 => None,
            Self::Pu245 => None,
            Self::Pu246 => None,
            Self::Pu247 => None,
        }
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
