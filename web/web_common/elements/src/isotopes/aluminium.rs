//! Isotopes of the element Aluminium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Aluminium
pub enum AluminiumIsotope {
    /// Isotope Al21 of Aluminium
    Al21,
    /// Isotope Al22 of Aluminium
    Al22,
    /// Isotope Al23 of Aluminium
    Al23,
    /// Isotope Al24 of Aluminium
    Al24,
    /// Isotope Al25 of Aluminium
    Al25,
    /// Isotope Al26 of Aluminium
    Al26,
    /// Isotope Al27 of Aluminium
    Al27,
    /// Isotope Al28 of Aluminium
    Al28,
    /// Isotope Al29 of Aluminium
    Al29,
    /// Isotope Al30 of Aluminium
    Al30,
    /// Isotope Al31 of Aluminium
    Al31,
    /// Isotope Al32 of Aluminium
    Al32,
    /// Isotope Al33 of Aluminium
    Al33,
    /// Isotope Al34 of Aluminium
    Al34,
    /// Isotope Al35 of Aluminium
    Al35,
    /// Isotope Al36 of Aluminium
    Al36,
    /// Isotope Al37 of Aluminium
    Al37,
    /// Isotope Al38 of Aluminium
    Al38,
    /// Isotope Al39 of Aluminium
    Al39,
    /// Isotope Al40 of Aluminium
    Al40,
    /// Isotope Al41 of Aluminium
    Al41,
    /// Isotope Al42 of Aluminium
    Al42,
    /// Isotope Al43 of Aluminium
    Al43,
}
impl super::RelativeAtomicMass for AluminiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Al21 => 21.02897f64,
            Self::Al22 => 22.01954f64,
            Self::Al23 => 23.00724435f64,
            Self::Al24 => 23.9999489f64,
            Self::Al25 => 24.9904281f64,
            Self::Al26 => 25.986891904f64,
            Self::Al27 => 26.98153853f64,
            Self::Al28 => 27.98191021f64,
            Self::Al29 => 28.9804565f64,
            Self::Al30 => 29.98296f64,
            Self::Al31 => 30.983945f64,
            Self::Al32 => 31.988085f64,
            Self::Al33 => 32.990909f64,
            Self::Al34 => 33.996705f64,
            Self::Al35 => 34.999764f64,
            Self::Al36 => 36.00639f64,
            Self::Al37 => 37.01053f64,
            Self::Al38 => 38.0174f64,
            Self::Al39 => 39.02254f64,
            Self::Al40 => 40.03003f64,
            Self::Al41 => 41.03638f64,
            Self::Al42 => 42.04384f64,
            Self::Al43 => 43.05147f64,
        }
    }
}
impl super::ElementVariant for AluminiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Al
    }
}
impl super::MassNumber for AluminiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Al21 => 21u16,
            Self::Al22 => 22u16,
            Self::Al23 => 23u16,
            Self::Al24 => 24u16,
            Self::Al25 => 25u16,
            Self::Al26 => 26u16,
            Self::Al27 => 27u16,
            Self::Al28 => 28u16,
            Self::Al29 => 29u16,
            Self::Al30 => 30u16,
            Self::Al31 => 31u16,
            Self::Al32 => 32u16,
            Self::Al33 => 33u16,
            Self::Al34 => 34u16,
            Self::Al35 => 35u16,
            Self::Al36 => 36u16,
            Self::Al37 => 37u16,
            Self::Al38 => 38u16,
            Self::Al39 => 39u16,
            Self::Al40 => 40u16,
            Self::Al41 => 41u16,
            Self::Al42 => 42u16,
            Self::Al43 => 43u16,
        }
    }
}
impl super::IsotopicComposition for AluminiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Al27 => Some(1f64),
            Self::Al21
            | Self::Al22
            | Self::Al23
            | Self::Al24
            | Self::Al25
            | Self::Al26
            | Self::Al28
            | Self::Al29
            | Self::Al30
            | Self::Al31
            | Self::Al32
            | Self::Al33
            | Self::Al34
            | Self::Al35
            | Self::Al36
            | Self::Al37
            | Self::Al38
            | Self::Al39
            | Self::Al40
            | Self::Al41
            | Self::Al42
            | Self::Al43 => None,
        }
    }
}
impl super::MostAbundantIsotope for AluminiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Al27
    }
}
impl TryFrom<u16> for AluminiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            21u16 => Ok(Self::Al21),
            22u16 => Ok(Self::Al22),
            23u16 => Ok(Self::Al23),
            24u16 => Ok(Self::Al24),
            25u16 => Ok(Self::Al25),
            26u16 => Ok(Self::Al26),
            27u16 => Ok(Self::Al27),
            28u16 => Ok(Self::Al28),
            29u16 => Ok(Self::Al29),
            30u16 => Ok(Self::Al30),
            31u16 => Ok(Self::Al31),
            32u16 => Ok(Self::Al32),
            33u16 => Ok(Self::Al33),
            34u16 => Ok(Self::Al34),
            35u16 => Ok(Self::Al35),
            36u16 => Ok(Self::Al36),
            37u16 => Ok(Self::Al37),
            38u16 => Ok(Self::Al38),
            39u16 => Ok(Self::Al39),
            40u16 => Ok(Self::Al40),
            41u16 => Ok(Self::Al41),
            42u16 => Ok(Self::Al42),
            43u16 => Ok(Self::Al43),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Al, value)),
        }
    }
}
impl std::fmt::Display for AluminiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Al21 => write!(f, "Al21"),
            Self::Al22 => write!(f, "Al22"),
            Self::Al23 => write!(f, "Al23"),
            Self::Al24 => write!(f, "Al24"),
            Self::Al25 => write!(f, "Al25"),
            Self::Al26 => write!(f, "Al26"),
            Self::Al27 => write!(f, "Al27"),
            Self::Al28 => write!(f, "Al28"),
            Self::Al29 => write!(f, "Al29"),
            Self::Al30 => write!(f, "Al30"),
            Self::Al31 => write!(f, "Al31"),
            Self::Al32 => write!(f, "Al32"),
            Self::Al33 => write!(f, "Al33"),
            Self::Al34 => write!(f, "Al34"),
            Self::Al35 => write!(f, "Al35"),
            Self::Al36 => write!(f, "Al36"),
            Self::Al37 => write!(f, "Al37"),
            Self::Al38 => write!(f, "Al38"),
            Self::Al39 => write!(f, "Al39"),
            Self::Al40 => write!(f, "Al40"),
            Self::Al41 => write!(f, "Al41"),
            Self::Al42 => write!(f, "Al42"),
            Self::Al43 => write!(f, "Al43"),
        }
    }
}
