//! Isotopes of the element Fluorine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Fluorine
pub enum FluorineIsotope {
    /// Isotope F14 of Fluorine
    F14,
    /// Isotope F15 of Fluorine
    F15,
    /// Isotope F16 of Fluorine
    F16,
    /// Isotope F17 of Fluorine
    F17,
    /// Isotope F18 of Fluorine
    F18,
    /// Isotope F19 of Fluorine
    F19,
    /// Isotope F20 of Fluorine
    F20,
    /// Isotope F21 of Fluorine
    F21,
    /// Isotope F22 of Fluorine
    F22,
    /// Isotope F23 of Fluorine
    F23,
    /// Isotope F24 of Fluorine
    F24,
    /// Isotope F25 of Fluorine
    F25,
    /// Isotope F26 of Fluorine
    F26,
    /// Isotope F27 of Fluorine
    F27,
    /// Isotope F28 of Fluorine
    F28,
    /// Isotope F29 of Fluorine
    F29,
    /// Isotope F30 of Fluorine
    F30,
    /// Isotope F31 of Fluorine
    F31,
}
impl super::RelativeAtomicMass for FluorineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::F14 => 14.034315f64,
            Self::F15 => 15.018043f64,
            Self::F16 => 16.0114657f64,
            Self::F17 => 17.00209524f64,
            Self::F18 => 18.00093733f64,
            Self::F19 => 18.99840316273f64,
            Self::F20 => 19.999981252f64,
            Self::F21 => 20.9999489f64,
            Self::F22 => 22.002999f64,
            Self::F23 => 23.003557f64,
            Self::F24 => 24.008115f64,
            Self::F25 => 25.012199f64,
            Self::F26 => 26.020038f64,
            Self::F27 => 27.02644f64,
            Self::F28 => 28.03534f64,
            Self::F29 => 29.04254f64,
            Self::F30 => 30.05165f64,
            Self::F31 => 31.05971f64,
        }
    }
}
impl super::ElementVariant for FluorineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::F
    }
}
impl super::MassNumber for FluorineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::F14 => 14u16,
            Self::F15 => 15u16,
            Self::F16 => 16u16,
            Self::F17 => 17u16,
            Self::F18 => 18u16,
            Self::F19 => 19u16,
            Self::F20 => 20u16,
            Self::F21 => 21u16,
            Self::F22 => 22u16,
            Self::F23 => 23u16,
            Self::F24 => 24u16,
            Self::F25 => 25u16,
            Self::F26 => 26u16,
            Self::F27 => 27u16,
            Self::F28 => 28u16,
            Self::F29 => 29u16,
            Self::F30 => 30u16,
            Self::F31 => 31u16,
        }
    }
}
impl super::IsotopicComposition for FluorineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::F19 => Some(1f64),
            Self::F14
            | Self::F15
            | Self::F16
            | Self::F17
            | Self::F18
            | Self::F20
            | Self::F21
            | Self::F22
            | Self::F23
            | Self::F24
            | Self::F25
            | Self::F26
            | Self::F27
            | Self::F28
            | Self::F29
            | Self::F30
            | Self::F31 => None,
        }
    }
}
impl super::MostAbundantIsotope for FluorineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::F19
    }
}
impl From<FluorineIsotope> for crate::Isotope {
    fn from(isotope: FluorineIsotope) -> Self {
        crate::Isotope::F(isotope)
    }
}
impl From<FluorineIsotope> for crate::Element {
    fn from(_isotope: FluorineIsotope) -> Self {
        crate::Element::F
    }
}
impl TryFrom<u16> for FluorineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            14u16 => Ok(Self::F14),
            15u16 => Ok(Self::F15),
            16u16 => Ok(Self::F16),
            17u16 => Ok(Self::F17),
            18u16 => Ok(Self::F18),
            19u16 => Ok(Self::F19),
            20u16 => Ok(Self::F20),
            21u16 => Ok(Self::F21),
            22u16 => Ok(Self::F22),
            23u16 => Ok(Self::F23),
            24u16 => Ok(Self::F24),
            25u16 => Ok(Self::F25),
            26u16 => Ok(Self::F26),
            27u16 => Ok(Self::F27),
            28u16 => Ok(Self::F28),
            29u16 => Ok(Self::F29),
            30u16 => Ok(Self::F30),
            31u16 => Ok(Self::F31),
            _ => Err(crate::errors::Error::Isotope(crate::Element::F, value)),
        }
    }
}
impl std::fmt::Display for FluorineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F14 => write!(f, "F14"),
            Self::F15 => write!(f, "F15"),
            Self::F16 => write!(f, "F16"),
            Self::F17 => write!(f, "F17"),
            Self::F18 => write!(f, "F18"),
            Self::F19 => write!(f, "F19"),
            Self::F20 => write!(f, "F20"),
            Self::F21 => write!(f, "F21"),
            Self::F22 => write!(f, "F22"),
            Self::F23 => write!(f, "F23"),
            Self::F24 => write!(f, "F24"),
            Self::F25 => write!(f, "F25"),
            Self::F26 => write!(f, "F26"),
            Self::F27 => write!(f, "F27"),
            Self::F28 => write!(f, "F28"),
            Self::F29 => write!(f, "F29"),
            Self::F30 => write!(f, "F30"),
            Self::F31 => write!(f, "F31"),
        }
    }
}
