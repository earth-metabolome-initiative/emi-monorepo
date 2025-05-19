//! Isotopes of the element Chlorine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Chlorine
pub enum ChlorineIsotope {
    /// Isotope Cl28 of Chlorine
    Cl28,
    /// Isotope Cl29 of Chlorine
    Cl29,
    /// Isotope Cl30 of Chlorine
    Cl30,
    /// Isotope Cl31 of Chlorine
    Cl31,
    /// Isotope Cl32 of Chlorine
    Cl32,
    /// Isotope Cl33 of Chlorine
    Cl33,
    /// Isotope Cl34 of Chlorine
    Cl34,
    /// Isotope Cl35 of Chlorine
    Cl35,
    /// Isotope Cl36 of Chlorine
    Cl36,
    /// Isotope Cl37 of Chlorine
    Cl37,
    /// Isotope Cl38 of Chlorine
    Cl38,
    /// Isotope Cl39 of Chlorine
    Cl39,
    /// Isotope Cl40 of Chlorine
    Cl40,
    /// Isotope Cl41 of Chlorine
    Cl41,
    /// Isotope Cl42 of Chlorine
    Cl42,
    /// Isotope Cl43 of Chlorine
    Cl43,
    /// Isotope Cl44 of Chlorine
    Cl44,
    /// Isotope Cl45 of Chlorine
    Cl45,
    /// Isotope Cl46 of Chlorine
    Cl46,
    /// Isotope Cl47 of Chlorine
    Cl47,
    /// Isotope Cl48 of Chlorine
    Cl48,
    /// Isotope Cl49 of Chlorine
    Cl49,
    /// Isotope Cl50 of Chlorine
    Cl50,
    /// Isotope Cl51 of Chlorine
    Cl51,
}
impl super::RelativeAtomicMass for ChlorineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cl28 => 28.02954f64,
            Self::Cl29 => 29.01478f64,
            Self::Cl30 => 30.00477f64,
            Self::Cl31 => 30.992414f64,
            Self::Cl32 => 31.98568464f64,
            Self::Cl33 => 32.97745199f64,
            Self::Cl34 => 33.973762485f64,
            Self::Cl35 => 34.968852682f64,
            Self::Cl36 => 35.968306809f64,
            Self::Cl37 => 36.965902602f64,
            Self::Cl38 => 37.96801044f64,
            Self::Cl39 => 38.9680082f64,
            Self::Cl40 => 39.970415f64,
            Self::Cl41 => 40.970685f64,
            Self::Cl42 => 41.97325f64,
            Self::Cl43 => 42.97389f64,
            Self::Cl44 => 43.97787f64,
            Self::Cl45 => 44.98029f64,
            Self::Cl46 => 45.98517f64,
            Self::Cl47 => 46.98916f64,
            Self::Cl48 => 47.99564f64,
            Self::Cl49 => 49.00123f64,
            Self::Cl50 => 50.00905f64,
            Self::Cl51 => 51.01554f64,
        }
    }
}
impl super::ElementVariant for ChlorineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cl
    }
}
impl super::MassNumber for ChlorineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cl28 => 28u16,
            Self::Cl29 => 29u16,
            Self::Cl30 => 30u16,
            Self::Cl31 => 31u16,
            Self::Cl32 => 32u16,
            Self::Cl33 => 33u16,
            Self::Cl34 => 34u16,
            Self::Cl35 => 35u16,
            Self::Cl36 => 36u16,
            Self::Cl37 => 37u16,
            Self::Cl38 => 38u16,
            Self::Cl39 => 39u16,
            Self::Cl40 => 40u16,
            Self::Cl41 => 41u16,
            Self::Cl42 => 42u16,
            Self::Cl43 => 43u16,
            Self::Cl44 => 44u16,
            Self::Cl45 => 45u16,
            Self::Cl46 => 46u16,
            Self::Cl47 => 47u16,
            Self::Cl48 => 48u16,
            Self::Cl49 => 49u16,
            Self::Cl50 => 50u16,
            Self::Cl51 => 51u16,
        }
    }
}
impl super::IsotopicComposition for ChlorineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cl35 => Some(0.7576f64),
            Self::Cl37 => Some(0.2424f64),
            Self::Cl28
            | Self::Cl29
            | Self::Cl30
            | Self::Cl31
            | Self::Cl32
            | Self::Cl33
            | Self::Cl34
            | Self::Cl36
            | Self::Cl38
            | Self::Cl39
            | Self::Cl40
            | Self::Cl41
            | Self::Cl42
            | Self::Cl43
            | Self::Cl44
            | Self::Cl45
            | Self::Cl46
            | Self::Cl47
            | Self::Cl48
            | Self::Cl49
            | Self::Cl50
            | Self::Cl51 => None,
        }
    }
}
impl super::MostAbundantIsotope for ChlorineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cl35
    }
}
impl TryFrom<u16> for ChlorineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            28u16 => Ok(Self::Cl28),
            29u16 => Ok(Self::Cl29),
            30u16 => Ok(Self::Cl30),
            31u16 => Ok(Self::Cl31),
            32u16 => Ok(Self::Cl32),
            33u16 => Ok(Self::Cl33),
            34u16 => Ok(Self::Cl34),
            35u16 => Ok(Self::Cl35),
            36u16 => Ok(Self::Cl36),
            37u16 => Ok(Self::Cl37),
            38u16 => Ok(Self::Cl38),
            39u16 => Ok(Self::Cl39),
            40u16 => Ok(Self::Cl40),
            41u16 => Ok(Self::Cl41),
            42u16 => Ok(Self::Cl42),
            43u16 => Ok(Self::Cl43),
            44u16 => Ok(Self::Cl44),
            45u16 => Ok(Self::Cl45),
            46u16 => Ok(Self::Cl46),
            47u16 => Ok(Self::Cl47),
            48u16 => Ok(Self::Cl48),
            49u16 => Ok(Self::Cl49),
            50u16 => Ok(Self::Cl50),
            51u16 => Ok(Self::Cl51),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cl, value)),
        }
    }
}
impl std::fmt::Display for ChlorineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cl28 => write!(f, "Cl28"),
            Self::Cl29 => write!(f, "Cl29"),
            Self::Cl30 => write!(f, "Cl30"),
            Self::Cl31 => write!(f, "Cl31"),
            Self::Cl32 => write!(f, "Cl32"),
            Self::Cl33 => write!(f, "Cl33"),
            Self::Cl34 => write!(f, "Cl34"),
            Self::Cl35 => write!(f, "Cl35"),
            Self::Cl36 => write!(f, "Cl36"),
            Self::Cl37 => write!(f, "Cl37"),
            Self::Cl38 => write!(f, "Cl38"),
            Self::Cl39 => write!(f, "Cl39"),
            Self::Cl40 => write!(f, "Cl40"),
            Self::Cl41 => write!(f, "Cl41"),
            Self::Cl42 => write!(f, "Cl42"),
            Self::Cl43 => write!(f, "Cl43"),
            Self::Cl44 => write!(f, "Cl44"),
            Self::Cl45 => write!(f, "Cl45"),
            Self::Cl46 => write!(f, "Cl46"),
            Self::Cl47 => write!(f, "Cl47"),
            Self::Cl48 => write!(f, "Cl48"),
            Self::Cl49 => write!(f, "Cl49"),
            Self::Cl50 => write!(f, "Cl50"),
            Self::Cl51 => write!(f, "Cl51"),
        }
    }
}
