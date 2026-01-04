//! Isotopes of the element Nickel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Nickel
pub enum NickelIsotope {
    /// Isotope Ni48 of Nickel
    Ni48,
    /// Isotope Ni49 of Nickel
    Ni49,
    /// Isotope Ni50 of Nickel
    Ni50,
    /// Isotope Ni51 of Nickel
    Ni51,
    /// Isotope Ni52 of Nickel
    Ni52,
    /// Isotope Ni53 of Nickel
    Ni53,
    /// Isotope Ni54 of Nickel
    Ni54,
    /// Isotope Ni55 of Nickel
    Ni55,
    /// Isotope Ni56 of Nickel
    Ni56,
    /// Isotope Ni57 of Nickel
    Ni57,
    /// Isotope Ni58 of Nickel
    Ni58,
    /// Isotope Ni59 of Nickel
    Ni59,
    /// Isotope Ni60 of Nickel
    Ni60,
    /// Isotope Ni61 of Nickel
    Ni61,
    /// Isotope Ni62 of Nickel
    Ni62,
    /// Isotope Ni63 of Nickel
    Ni63,
    /// Isotope Ni64 of Nickel
    Ni64,
    /// Isotope Ni65 of Nickel
    Ni65,
    /// Isotope Ni66 of Nickel
    Ni66,
    /// Isotope Ni67 of Nickel
    Ni67,
    /// Isotope Ni68 of Nickel
    Ni68,
    /// Isotope Ni69 of Nickel
    Ni69,
    /// Isotope Ni70 of Nickel
    Ni70,
    /// Isotope Ni71 of Nickel
    Ni71,
    /// Isotope Ni72 of Nickel
    Ni72,
    /// Isotope Ni73 of Nickel
    Ni73,
    /// Isotope Ni74 of Nickel
    Ni74,
    /// Isotope Ni75 of Nickel
    Ni75,
    /// Isotope Ni76 of Nickel
    Ni76,
    /// Isotope Ni77 of Nickel
    Ni77,
    /// Isotope Ni78 of Nickel
    Ni78,
    /// Isotope Ni79 of Nickel
    Ni79,
}
impl super::RelativeAtomicMass for NickelIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ni48 => 48.01769f64,
            Self::Ni49 => 49.0077f64,
            Self::Ni50 => 49.99474f64,
            Self::Ni51 => 50.98611f64,
            Self::Ni52 => 51.9748f64,
            Self::Ni53 => 52.96819f64,
            Self::Ni54 => 53.957892f64,
            Self::Ni55 => 54.95133063f64,
            Self::Ni56 => 55.94212855f64,
            Self::Ni57 => 56.93979218f64,
            Self::Ni58 => 57.93534241f64,
            Self::Ni59 => 58.9343462f64,
            Self::Ni60 => 59.93078588f64,
            Self::Ni61 => 60.93105557f64,
            Self::Ni62 => 61.92834537f64,
            Self::Ni63 => 62.92966963f64,
            Self::Ni64 => 63.92796682f64,
            Self::Ni65 => 64.93008517f64,
            Self::Ni66 => 65.9291393f64,
            Self::Ni67 => 66.9315694f64,
            Self::Ni68 => 67.9318688f64,
            Self::Ni69 => 68.9356103f64,
            Self::Ni70 => 69.9364313f64,
            Self::Ni71 => 70.940519f64,
            Self::Ni72 => 71.9417859f64,
            Self::Ni73 => 72.9462067f64,
            Self::Ni74 => 73.94798f64,
            Self::Ni75 => 74.9525f64,
            Self::Ni76 => 75.95533f64,
            Self::Ni77 => 76.96055f64,
            Self::Ni78 => 77.96336f64,
            Self::Ni79 => 78.97025f64,
        }
    }
}
impl super::ElementVariant for NickelIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ni
    }
}
impl super::MassNumber for NickelIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ni48 => 48u16,
            Self::Ni49 => 49u16,
            Self::Ni50 => 50u16,
            Self::Ni51 => 51u16,
            Self::Ni52 => 52u16,
            Self::Ni53 => 53u16,
            Self::Ni54 => 54u16,
            Self::Ni55 => 55u16,
            Self::Ni56 => 56u16,
            Self::Ni57 => 57u16,
            Self::Ni58 => 58u16,
            Self::Ni59 => 59u16,
            Self::Ni60 => 60u16,
            Self::Ni61 => 61u16,
            Self::Ni62 => 62u16,
            Self::Ni63 => 63u16,
            Self::Ni64 => 64u16,
            Self::Ni65 => 65u16,
            Self::Ni66 => 66u16,
            Self::Ni67 => 67u16,
            Self::Ni68 => 68u16,
            Self::Ni69 => 69u16,
            Self::Ni70 => 70u16,
            Self::Ni71 => 71u16,
            Self::Ni72 => 72u16,
            Self::Ni73 => 73u16,
            Self::Ni74 => 74u16,
            Self::Ni75 => 75u16,
            Self::Ni76 => 76u16,
            Self::Ni77 => 77u16,
            Self::Ni78 => 78u16,
            Self::Ni79 => 79u16,
        }
    }
}
impl super::IsotopicComposition for NickelIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ni58 => Some(0.68077f64),
            Self::Ni60 => Some(0.26223f64),
            Self::Ni61 => Some(0.011399f64),
            Self::Ni62 => Some(0.036346f64),
            Self::Ni64 => Some(0.009255f64),
            Self::Ni48
            | Self::Ni49
            | Self::Ni50
            | Self::Ni51
            | Self::Ni52
            | Self::Ni53
            | Self::Ni54
            | Self::Ni55
            | Self::Ni56
            | Self::Ni57
            | Self::Ni59
            | Self::Ni63
            | Self::Ni65
            | Self::Ni66
            | Self::Ni67
            | Self::Ni68
            | Self::Ni69
            | Self::Ni70
            | Self::Ni71
            | Self::Ni72
            | Self::Ni73
            | Self::Ni74
            | Self::Ni75
            | Self::Ni76
            | Self::Ni77
            | Self::Ni78
            | Self::Ni79 => None,
        }
    }
}
impl super::MostAbundantIsotope for NickelIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ni58
    }
}
impl From<NickelIsotope> for crate::Isotope {
    fn from(isotope: NickelIsotope) -> Self {
        crate::Isotope::Ni(isotope)
    }
}
impl From<NickelIsotope> for crate::Element {
    fn from(_isotope: NickelIsotope) -> Self {
        crate::Element::Ni
    }
}
impl TryFrom<u16> for NickelIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            48u16 => Ok(Self::Ni48),
            49u16 => Ok(Self::Ni49),
            50u16 => Ok(Self::Ni50),
            51u16 => Ok(Self::Ni51),
            52u16 => Ok(Self::Ni52),
            53u16 => Ok(Self::Ni53),
            54u16 => Ok(Self::Ni54),
            55u16 => Ok(Self::Ni55),
            56u16 => Ok(Self::Ni56),
            57u16 => Ok(Self::Ni57),
            58u16 => Ok(Self::Ni58),
            59u16 => Ok(Self::Ni59),
            60u16 => Ok(Self::Ni60),
            61u16 => Ok(Self::Ni61),
            62u16 => Ok(Self::Ni62),
            63u16 => Ok(Self::Ni63),
            64u16 => Ok(Self::Ni64),
            65u16 => Ok(Self::Ni65),
            66u16 => Ok(Self::Ni66),
            67u16 => Ok(Self::Ni67),
            68u16 => Ok(Self::Ni68),
            69u16 => Ok(Self::Ni69),
            70u16 => Ok(Self::Ni70),
            71u16 => Ok(Self::Ni71),
            72u16 => Ok(Self::Ni72),
            73u16 => Ok(Self::Ni73),
            74u16 => Ok(Self::Ni74),
            75u16 => Ok(Self::Ni75),
            76u16 => Ok(Self::Ni76),
            77u16 => Ok(Self::Ni77),
            78u16 => Ok(Self::Ni78),
            79u16 => Ok(Self::Ni79),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ni, value)),
        }
    }
}
impl std::fmt::Display for NickelIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ni48 => write!(f, "Ni48"),
            Self::Ni49 => write!(f, "Ni49"),
            Self::Ni50 => write!(f, "Ni50"),
            Self::Ni51 => write!(f, "Ni51"),
            Self::Ni52 => write!(f, "Ni52"),
            Self::Ni53 => write!(f, "Ni53"),
            Self::Ni54 => write!(f, "Ni54"),
            Self::Ni55 => write!(f, "Ni55"),
            Self::Ni56 => write!(f, "Ni56"),
            Self::Ni57 => write!(f, "Ni57"),
            Self::Ni58 => write!(f, "Ni58"),
            Self::Ni59 => write!(f, "Ni59"),
            Self::Ni60 => write!(f, "Ni60"),
            Self::Ni61 => write!(f, "Ni61"),
            Self::Ni62 => write!(f, "Ni62"),
            Self::Ni63 => write!(f, "Ni63"),
            Self::Ni64 => write!(f, "Ni64"),
            Self::Ni65 => write!(f, "Ni65"),
            Self::Ni66 => write!(f, "Ni66"),
            Self::Ni67 => write!(f, "Ni67"),
            Self::Ni68 => write!(f, "Ni68"),
            Self::Ni69 => write!(f, "Ni69"),
            Self::Ni70 => write!(f, "Ni70"),
            Self::Ni71 => write!(f, "Ni71"),
            Self::Ni72 => write!(f, "Ni72"),
            Self::Ni73 => write!(f, "Ni73"),
            Self::Ni74 => write!(f, "Ni74"),
            Self::Ni75 => write!(f, "Ni75"),
            Self::Ni76 => write!(f, "Ni76"),
            Self::Ni77 => write!(f, "Ni77"),
            Self::Ni78 => write!(f, "Ni78"),
            Self::Ni79 => write!(f, "Ni79"),
        }
    }
}
