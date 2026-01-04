//! Isotopes of the element Phosphorus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Phosphorus
pub enum PhosphorusIsotope {
    /// Isotope P24 of Phosphorus
    P24,
    /// Isotope P25 of Phosphorus
    P25,
    /// Isotope P26 of Phosphorus
    P26,
    /// Isotope P27 of Phosphorus
    P27,
    /// Isotope P28 of Phosphorus
    P28,
    /// Isotope P29 of Phosphorus
    P29,
    /// Isotope P30 of Phosphorus
    P30,
    /// Isotope P31 of Phosphorus
    P31,
    /// Isotope P32 of Phosphorus
    P32,
    /// Isotope P33 of Phosphorus
    P33,
    /// Isotope P34 of Phosphorus
    P34,
    /// Isotope P35 of Phosphorus
    P35,
    /// Isotope P36 of Phosphorus
    P36,
    /// Isotope P37 of Phosphorus
    P37,
    /// Isotope P38 of Phosphorus
    P38,
    /// Isotope P39 of Phosphorus
    P39,
    /// Isotope P40 of Phosphorus
    P40,
    /// Isotope P41 of Phosphorus
    P41,
    /// Isotope P42 of Phosphorus
    P42,
    /// Isotope P43 of Phosphorus
    P43,
    /// Isotope P44 of Phosphorus
    P44,
    /// Isotope P45 of Phosphorus
    P45,
    /// Isotope P46 of Phosphorus
    P46,
    /// Isotope P47 of Phosphorus
    P47,
}
impl super::RelativeAtomicMass for PhosphorusIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::P24 => 24.03577f64,
            Self::P25 => 25.02119f64,
            Self::P26 => 26.01178f64,
            Self::P27 => 26.999224f64,
            Self::P28 => 27.9923266f64,
            Self::P29 => 28.98180079f64,
            Self::P30 => 29.97831375f64,
            Self::P31 => 30.97376199842f64,
            Self::P32 => 31.973907643f64,
            Self::P33 => 32.9717257f64,
            Self::P34 => 33.97364589f64,
            Self::P35 => 34.9733141f64,
            Self::P36 => 35.97826f64,
            Self::P37 => 36.979607f64,
            Self::P38 => 37.984252f64,
            Self::P39 => 38.986227f64,
            Self::P40 => 39.99133f64,
            Self::P41 => 40.994654f64,
            Self::P42 => 42.00108f64,
            Self::P43 => 43.00502f64,
            Self::P44 => 44.01121f64,
            Self::P45 => 45.01645f64,
            Self::P46 => 46.02446f64,
            Self::P47 => 47.03139f64,
        }
    }
}
impl super::ElementVariant for PhosphorusIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::P
    }
}
impl super::MassNumber for PhosphorusIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::P24 => 24u16,
            Self::P25 => 25u16,
            Self::P26 => 26u16,
            Self::P27 => 27u16,
            Self::P28 => 28u16,
            Self::P29 => 29u16,
            Self::P30 => 30u16,
            Self::P31 => 31u16,
            Self::P32 => 32u16,
            Self::P33 => 33u16,
            Self::P34 => 34u16,
            Self::P35 => 35u16,
            Self::P36 => 36u16,
            Self::P37 => 37u16,
            Self::P38 => 38u16,
            Self::P39 => 39u16,
            Self::P40 => 40u16,
            Self::P41 => 41u16,
            Self::P42 => 42u16,
            Self::P43 => 43u16,
            Self::P44 => 44u16,
            Self::P45 => 45u16,
            Self::P46 => 46u16,
            Self::P47 => 47u16,
        }
    }
}
impl super::IsotopicComposition for PhosphorusIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::P31 => Some(1f64),
            Self::P24
            | Self::P25
            | Self::P26
            | Self::P27
            | Self::P28
            | Self::P29
            | Self::P30
            | Self::P32
            | Self::P33
            | Self::P34
            | Self::P35
            | Self::P36
            | Self::P37
            | Self::P38
            | Self::P39
            | Self::P40
            | Self::P41
            | Self::P42
            | Self::P43
            | Self::P44
            | Self::P45
            | Self::P46
            | Self::P47 => None,
        }
    }
}
impl super::MostAbundantIsotope for PhosphorusIsotope {
    fn most_abundant_isotope() -> Self {
        Self::P31
    }
}
impl From<PhosphorusIsotope> for crate::Isotope {
    fn from(isotope: PhosphorusIsotope) -> Self {
        crate::Isotope::P(isotope)
    }
}
impl From<PhosphorusIsotope> for crate::Element {
    fn from(_isotope: PhosphorusIsotope) -> Self {
        crate::Element::P
    }
}
impl TryFrom<u16> for PhosphorusIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            24u16 => Ok(Self::P24),
            25u16 => Ok(Self::P25),
            26u16 => Ok(Self::P26),
            27u16 => Ok(Self::P27),
            28u16 => Ok(Self::P28),
            29u16 => Ok(Self::P29),
            30u16 => Ok(Self::P30),
            31u16 => Ok(Self::P31),
            32u16 => Ok(Self::P32),
            33u16 => Ok(Self::P33),
            34u16 => Ok(Self::P34),
            35u16 => Ok(Self::P35),
            36u16 => Ok(Self::P36),
            37u16 => Ok(Self::P37),
            38u16 => Ok(Self::P38),
            39u16 => Ok(Self::P39),
            40u16 => Ok(Self::P40),
            41u16 => Ok(Self::P41),
            42u16 => Ok(Self::P42),
            43u16 => Ok(Self::P43),
            44u16 => Ok(Self::P44),
            45u16 => Ok(Self::P45),
            46u16 => Ok(Self::P46),
            47u16 => Ok(Self::P47),
            _ => Err(crate::errors::Error::Isotope(crate::Element::P, value)),
        }
    }
}
impl std::fmt::Display for PhosphorusIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::P24 => write!(f, "P24"),
            Self::P25 => write!(f, "P25"),
            Self::P26 => write!(f, "P26"),
            Self::P27 => write!(f, "P27"),
            Self::P28 => write!(f, "P28"),
            Self::P29 => write!(f, "P29"),
            Self::P30 => write!(f, "P30"),
            Self::P31 => write!(f, "P31"),
            Self::P32 => write!(f, "P32"),
            Self::P33 => write!(f, "P33"),
            Self::P34 => write!(f, "P34"),
            Self::P35 => write!(f, "P35"),
            Self::P36 => write!(f, "P36"),
            Self::P37 => write!(f, "P37"),
            Self::P38 => write!(f, "P38"),
            Self::P39 => write!(f, "P39"),
            Self::P40 => write!(f, "P40"),
            Self::P41 => write!(f, "P41"),
            Self::P42 => write!(f, "P42"),
            Self::P43 => write!(f, "P43"),
            Self::P44 => write!(f, "P44"),
            Self::P45 => write!(f, "P45"),
            Self::P46 => write!(f, "P46"),
            Self::P47 => write!(f, "P47"),
        }
    }
}
