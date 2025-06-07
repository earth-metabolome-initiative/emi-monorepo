//! Isotopes of the element Actinium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Actinium
pub enum ActiniumIsotope {
    /// Isotope Ac206 of Actinium
    Ac206,
    /// Isotope Ac207 of Actinium
    Ac207,
    /// Isotope Ac208 of Actinium
    Ac208,
    /// Isotope Ac209 of Actinium
    Ac209,
    /// Isotope Ac210 of Actinium
    Ac210,
    /// Isotope Ac211 of Actinium
    Ac211,
    /// Isotope Ac212 of Actinium
    Ac212,
    /// Isotope Ac213 of Actinium
    Ac213,
    /// Isotope Ac214 of Actinium
    Ac214,
    /// Isotope Ac215 of Actinium
    Ac215,
    /// Isotope Ac216 of Actinium
    Ac216,
    /// Isotope Ac217 of Actinium
    Ac217,
    /// Isotope Ac218 of Actinium
    Ac218,
    /// Isotope Ac219 of Actinium
    Ac219,
    /// Isotope Ac220 of Actinium
    Ac220,
    /// Isotope Ac221 of Actinium
    Ac221,
    /// Isotope Ac222 of Actinium
    Ac222,
    /// Isotope Ac223 of Actinium
    Ac223,
    /// Isotope Ac224 of Actinium
    Ac224,
    /// Isotope Ac225 of Actinium
    Ac225,
    /// Isotope Ac226 of Actinium
    Ac226,
    /// Isotope Ac227 of Actinium
    Ac227,
    /// Isotope Ac228 of Actinium
    Ac228,
    /// Isotope Ac229 of Actinium
    Ac229,
    /// Isotope Ac230 of Actinium
    Ac230,
    /// Isotope Ac231 of Actinium
    Ac231,
    /// Isotope Ac232 of Actinium
    Ac232,
    /// Isotope Ac233 of Actinium
    Ac233,
    /// Isotope Ac234 of Actinium
    Ac234,
    /// Isotope Ac235 of Actinium
    Ac235,
    /// Isotope Ac236 of Actinium
    Ac236,
    /// Isotope Ac237 of Actinium
    Ac237,
}
impl super::RelativeAtomicMass for ActiniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ac206 => 206.014452f64,
            Self::Ac207 => 207.011966f64,
            Self::Ac208 => 208.01155f64,
            Self::Ac209 => 209.009495f64,
            Self::Ac210 => 210.009436f64,
            Self::Ac211 => 211.007732f64,
            Self::Ac212 => 212.007813f64,
            Self::Ac213 => 213.006609f64,
            Self::Ac214 => 214.006918f64,
            Self::Ac215 => 215.006475f64,
            Self::Ac216 => 216.008743f64,
            Self::Ac217 => 217.009344f64,
            Self::Ac218 => 218.011642f64,
            Self::Ac219 => 219.012421f64,
            Self::Ac220 => 220.0147549f64,
            Self::Ac221 => 221.015592f64,
            Self::Ac222 => 222.0178442f64,
            Self::Ac223 => 223.0191377f64,
            Self::Ac224 => 224.0217232f64,
            Self::Ac225 => 225.02323f64,
            Self::Ac226 => 226.0260984f64,
            Self::Ac227 => 227.0277523f64,
            Self::Ac228 => 228.0310215f64,
            Self::Ac229 => 229.032956f64,
            Self::Ac230 => 230.036327f64,
            Self::Ac231 => 231.038393f64,
            Self::Ac232 => 232.042034f64,
            Self::Ac233 => 233.044346f64,
            Self::Ac234 => 234.048139f64,
            Self::Ac235 => 235.05084f64,
            Self::Ac236 => 236.054988f64,
            Self::Ac237 => 237.05827f64,
        }
    }
}
impl super::ElementVariant for ActiniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ac
    }
}
impl super::MassNumber for ActiniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ac206 => 206u16,
            Self::Ac207 => 207u16,
            Self::Ac208 => 208u16,
            Self::Ac209 => 209u16,
            Self::Ac210 => 210u16,
            Self::Ac211 => 211u16,
            Self::Ac212 => 212u16,
            Self::Ac213 => 213u16,
            Self::Ac214 => 214u16,
            Self::Ac215 => 215u16,
            Self::Ac216 => 216u16,
            Self::Ac217 => 217u16,
            Self::Ac218 => 218u16,
            Self::Ac219 => 219u16,
            Self::Ac220 => 220u16,
            Self::Ac221 => 221u16,
            Self::Ac222 => 222u16,
            Self::Ac223 => 223u16,
            Self::Ac224 => 224u16,
            Self::Ac225 => 225u16,
            Self::Ac226 => 226u16,
            Self::Ac227 => 227u16,
            Self::Ac228 => 228u16,
            Self::Ac229 => 229u16,
            Self::Ac230 => 230u16,
            Self::Ac231 => 231u16,
            Self::Ac232 => 232u16,
            Self::Ac233 => 233u16,
            Self::Ac234 => 234u16,
            Self::Ac235 => 235u16,
            Self::Ac236 => 236u16,
            Self::Ac237 => 237u16,
        }
    }
}
impl super::IsotopicComposition for ActiniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for ActiniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ac237
    }
}
impl From<ActiniumIsotope> for crate::Isotope {
    fn from(isotope: ActiniumIsotope) -> Self {
        crate::Isotope::Ac(isotope)
    }
}
impl From<ActiniumIsotope> for crate::Element {
    fn from(_isotope: ActiniumIsotope) -> Self {
        crate::Element::Ac
    }
}
impl TryFrom<u16> for ActiniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            206u16 => Ok(Self::Ac206),
            207u16 => Ok(Self::Ac207),
            208u16 => Ok(Self::Ac208),
            209u16 => Ok(Self::Ac209),
            210u16 => Ok(Self::Ac210),
            211u16 => Ok(Self::Ac211),
            212u16 => Ok(Self::Ac212),
            213u16 => Ok(Self::Ac213),
            214u16 => Ok(Self::Ac214),
            215u16 => Ok(Self::Ac215),
            216u16 => Ok(Self::Ac216),
            217u16 => Ok(Self::Ac217),
            218u16 => Ok(Self::Ac218),
            219u16 => Ok(Self::Ac219),
            220u16 => Ok(Self::Ac220),
            221u16 => Ok(Self::Ac221),
            222u16 => Ok(Self::Ac222),
            223u16 => Ok(Self::Ac223),
            224u16 => Ok(Self::Ac224),
            225u16 => Ok(Self::Ac225),
            226u16 => Ok(Self::Ac226),
            227u16 => Ok(Self::Ac227),
            228u16 => Ok(Self::Ac228),
            229u16 => Ok(Self::Ac229),
            230u16 => Ok(Self::Ac230),
            231u16 => Ok(Self::Ac231),
            232u16 => Ok(Self::Ac232),
            233u16 => Ok(Self::Ac233),
            234u16 => Ok(Self::Ac234),
            235u16 => Ok(Self::Ac235),
            236u16 => Ok(Self::Ac236),
            237u16 => Ok(Self::Ac237),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ac, value)),
        }
    }
}
impl std::fmt::Display for ActiniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ac206 => write!(f, "Ac206"),
            Self::Ac207 => write!(f, "Ac207"),
            Self::Ac208 => write!(f, "Ac208"),
            Self::Ac209 => write!(f, "Ac209"),
            Self::Ac210 => write!(f, "Ac210"),
            Self::Ac211 => write!(f, "Ac211"),
            Self::Ac212 => write!(f, "Ac212"),
            Self::Ac213 => write!(f, "Ac213"),
            Self::Ac214 => write!(f, "Ac214"),
            Self::Ac215 => write!(f, "Ac215"),
            Self::Ac216 => write!(f, "Ac216"),
            Self::Ac217 => write!(f, "Ac217"),
            Self::Ac218 => write!(f, "Ac218"),
            Self::Ac219 => write!(f, "Ac219"),
            Self::Ac220 => write!(f, "Ac220"),
            Self::Ac221 => write!(f, "Ac221"),
            Self::Ac222 => write!(f, "Ac222"),
            Self::Ac223 => write!(f, "Ac223"),
            Self::Ac224 => write!(f, "Ac224"),
            Self::Ac225 => write!(f, "Ac225"),
            Self::Ac226 => write!(f, "Ac226"),
            Self::Ac227 => write!(f, "Ac227"),
            Self::Ac228 => write!(f, "Ac228"),
            Self::Ac229 => write!(f, "Ac229"),
            Self::Ac230 => write!(f, "Ac230"),
            Self::Ac231 => write!(f, "Ac231"),
            Self::Ac232 => write!(f, "Ac232"),
            Self::Ac233 => write!(f, "Ac233"),
            Self::Ac234 => write!(f, "Ac234"),
            Self::Ac235 => write!(f, "Ac235"),
            Self::Ac236 => write!(f, "Ac236"),
            Self::Ac237 => write!(f, "Ac237"),
        }
    }
}
