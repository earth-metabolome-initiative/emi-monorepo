//! Isotopes of the element Lithium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Lithium
pub enum LithiumIsotope {
    /// Isotope Li3 of Lithium
    Li3,
    /// Isotope Li4 of Lithium
    Li4,
    /// Isotope Li5 of Lithium
    Li5,
    /// Isotope Li6 of Lithium
    Li6,
    /// Isotope Li7 of Lithium
    Li7,
    /// Isotope Li8 of Lithium
    Li8,
    /// Isotope Li9 of Lithium
    Li9,
    /// Isotope Li10 of Lithium
    Li10,
    /// Isotope Li11 of Lithium
    Li11,
    /// Isotope Li12 of Lithium
    Li12,
    /// Isotope Li13 of Lithium
    Li13,
}
impl super::RelativeAtomicMass for LithiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Li3 => 3.0308f64,
            Self::Li4 => 4.02719f64,
            Self::Li5 => 5.012538f64,
            Self::Li6 => 6.0151228874f64,
            Self::Li7 => 7.0160034366f64,
            Self::Li8 => 8.022486246f64,
            Self::Li9 => 9.02679019f64,
            Self::Li10 => 10.035483f64,
            Self::Li11 => 11.04372358f64,
            Self::Li12 => 12.052517f64,
            Self::Li13 => 13.06263f64,
        }
    }
}
impl super::ElementVariant for LithiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Li
    }
}
impl super::MassNumber for LithiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Li3 => 3u16,
            Self::Li4 => 4u16,
            Self::Li5 => 5u16,
            Self::Li6 => 6u16,
            Self::Li7 => 7u16,
            Self::Li8 => 8u16,
            Self::Li9 => 9u16,
            Self::Li10 => 10u16,
            Self::Li11 => 11u16,
            Self::Li12 => 12u16,
            Self::Li13 => 13u16,
        }
    }
}
impl super::IsotopicComposition for LithiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Li6 => Some(0.0759f64),
            Self::Li7 => Some(0.9241f64),
            Self::Li3
            | Self::Li4
            | Self::Li5
            | Self::Li8
            | Self::Li9
            | Self::Li10
            | Self::Li11
            | Self::Li12
            | Self::Li13 => None,
        }
    }
}
impl super::MostAbundantIsotope for LithiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Li7
    }
}
impl From<LithiumIsotope> for crate::Isotope {
    fn from(isotope: LithiumIsotope) -> Self {
        crate::Isotope::Li(isotope)
    }
}
impl From<LithiumIsotope> for crate::Element {
    fn from(_isotope: LithiumIsotope) -> Self {
        crate::Element::Li
    }
}
impl TryFrom<u16> for LithiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            3u16 => Ok(Self::Li3),
            4u16 => Ok(Self::Li4),
            5u16 => Ok(Self::Li5),
            6u16 => Ok(Self::Li6),
            7u16 => Ok(Self::Li7),
            8u16 => Ok(Self::Li8),
            9u16 => Ok(Self::Li9),
            10u16 => Ok(Self::Li10),
            11u16 => Ok(Self::Li11),
            12u16 => Ok(Self::Li12),
            13u16 => Ok(Self::Li13),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Li, value)),
        }
    }
}
impl std::fmt::Display for LithiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Li3 => write!(f, "Li3"),
            Self::Li4 => write!(f, "Li4"),
            Self::Li5 => write!(f, "Li5"),
            Self::Li6 => write!(f, "Li6"),
            Self::Li7 => write!(f, "Li7"),
            Self::Li8 => write!(f, "Li8"),
            Self::Li9 => write!(f, "Li9"),
            Self::Li10 => write!(f, "Li10"),
            Self::Li11 => write!(f, "Li11"),
            Self::Li12 => write!(f, "Li12"),
            Self::Li13 => write!(f, "Li13"),
        }
    }
}
