//! Isotopes of the element Titanium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Titanium
pub enum TitaniumIsotope {
    /// Isotope Ti38 of Titanium
    Ti38,
    /// Isotope Ti39 of Titanium
    Ti39,
    /// Isotope Ti40 of Titanium
    Ti40,
    /// Isotope Ti41 of Titanium
    Ti41,
    /// Isotope Ti42 of Titanium
    Ti42,
    /// Isotope Ti43 of Titanium
    Ti43,
    /// Isotope Ti44 of Titanium
    Ti44,
    /// Isotope Ti45 of Titanium
    Ti45,
    /// Isotope Ti46 of Titanium
    Ti46,
    /// Isotope Ti47 of Titanium
    Ti47,
    /// Isotope Ti48 of Titanium
    Ti48,
    /// Isotope Ti49 of Titanium
    Ti49,
    /// Isotope Ti50 of Titanium
    Ti50,
    /// Isotope Ti51 of Titanium
    Ti51,
    /// Isotope Ti52 of Titanium
    Ti52,
    /// Isotope Ti53 of Titanium
    Ti53,
    /// Isotope Ti54 of Titanium
    Ti54,
    /// Isotope Ti55 of Titanium
    Ti55,
    /// Isotope Ti56 of Titanium
    Ti56,
    /// Isotope Ti57 of Titanium
    Ti57,
    /// Isotope Ti58 of Titanium
    Ti58,
    /// Isotope Ti59 of Titanium
    Ti59,
    /// Isotope Ti60 of Titanium
    Ti60,
    /// Isotope Ti61 of Titanium
    Ti61,
    /// Isotope Ti62 of Titanium
    Ti62,
    /// Isotope Ti63 of Titanium
    Ti63,
}
impl super::RelativeAtomicMass for TitaniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ti38 => 38.01145f64,
            Self::Ti39 => 39.00236f64,
            Self::Ti40 => 39.9905f64,
            Self::Ti41 => 40.983148f64,
            Self::Ti42 => 41.97304903f64,
            Self::Ti43 => 42.9685225f64,
            Self::Ti44 => 43.95968995f64,
            Self::Ti45 => 44.95812198f64,
            Self::Ti46 => 45.95262772f64,
            Self::Ti47 => 46.95175879f64,
            Self::Ti48 => 47.94794198f64,
            Self::Ti49 => 48.94786568f64,
            Self::Ti50 => 49.94478689f64,
            Self::Ti51 => 50.94661065f64,
            Self::Ti52 => 51.946893f64,
            Self::Ti53 => 52.94973f64,
            Self::Ti54 => 53.95105f64,
            Self::Ti55 => 54.95527f64,
            Self::Ti56 => 55.95791f64,
            Self::Ti57 => 56.96364f64,
            Self::Ti58 => 57.9666f64,
            Self::Ti59 => 58.97247f64,
            Self::Ti60 => 59.97603f64,
            Self::Ti61 => 60.98245f64,
            Self::Ti62 => 61.98651f64,
            Self::Ti63 => 62.99375f64,
        }
    }
}
impl super::ElementVariant for TitaniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ti
    }
}
impl super::MassNumber for TitaniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ti38 => 38u16,
            Self::Ti39 => 39u16,
            Self::Ti40 => 40u16,
            Self::Ti41 => 41u16,
            Self::Ti42 => 42u16,
            Self::Ti43 => 43u16,
            Self::Ti44 => 44u16,
            Self::Ti45 => 45u16,
            Self::Ti46 => 46u16,
            Self::Ti47 => 47u16,
            Self::Ti48 => 48u16,
            Self::Ti49 => 49u16,
            Self::Ti50 => 50u16,
            Self::Ti51 => 51u16,
            Self::Ti52 => 52u16,
            Self::Ti53 => 53u16,
            Self::Ti54 => 54u16,
            Self::Ti55 => 55u16,
            Self::Ti56 => 56u16,
            Self::Ti57 => 57u16,
            Self::Ti58 => 58u16,
            Self::Ti59 => 59u16,
            Self::Ti60 => 60u16,
            Self::Ti61 => 61u16,
            Self::Ti62 => 62u16,
            Self::Ti63 => 63u16,
        }
    }
}
impl super::IsotopicComposition for TitaniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ti46 => Some(0.0825f64),
            Self::Ti47 => Some(0.0744f64),
            Self::Ti48 => Some(0.7372f64),
            Self::Ti49 => Some(0.0541f64),
            Self::Ti50 => Some(0.0518f64),
            Self::Ti38
            | Self::Ti39
            | Self::Ti40
            | Self::Ti41
            | Self::Ti42
            | Self::Ti43
            | Self::Ti44
            | Self::Ti45
            | Self::Ti51
            | Self::Ti52
            | Self::Ti53
            | Self::Ti54
            | Self::Ti55
            | Self::Ti56
            | Self::Ti57
            | Self::Ti58
            | Self::Ti59
            | Self::Ti60
            | Self::Ti61
            | Self::Ti62
            | Self::Ti63 => None,
        }
    }
}
impl super::MostAbundantIsotope for TitaniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ti48
    }
}
impl From<TitaniumIsotope> for crate::Isotope {
    fn from(isotope: TitaniumIsotope) -> Self {
        crate::Isotope::Ti(isotope)
    }
}
impl From<TitaniumIsotope> for crate::Element {
    fn from(_isotope: TitaniumIsotope) -> Self {
        crate::Element::Ti
    }
}
impl TryFrom<u16> for TitaniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            38u16 => Ok(Self::Ti38),
            39u16 => Ok(Self::Ti39),
            40u16 => Ok(Self::Ti40),
            41u16 => Ok(Self::Ti41),
            42u16 => Ok(Self::Ti42),
            43u16 => Ok(Self::Ti43),
            44u16 => Ok(Self::Ti44),
            45u16 => Ok(Self::Ti45),
            46u16 => Ok(Self::Ti46),
            47u16 => Ok(Self::Ti47),
            48u16 => Ok(Self::Ti48),
            49u16 => Ok(Self::Ti49),
            50u16 => Ok(Self::Ti50),
            51u16 => Ok(Self::Ti51),
            52u16 => Ok(Self::Ti52),
            53u16 => Ok(Self::Ti53),
            54u16 => Ok(Self::Ti54),
            55u16 => Ok(Self::Ti55),
            56u16 => Ok(Self::Ti56),
            57u16 => Ok(Self::Ti57),
            58u16 => Ok(Self::Ti58),
            59u16 => Ok(Self::Ti59),
            60u16 => Ok(Self::Ti60),
            61u16 => Ok(Self::Ti61),
            62u16 => Ok(Self::Ti62),
            63u16 => Ok(Self::Ti63),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ti, value)),
        }
    }
}
impl std::fmt::Display for TitaniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ti38 => write!(f, "Ti38"),
            Self::Ti39 => write!(f, "Ti39"),
            Self::Ti40 => write!(f, "Ti40"),
            Self::Ti41 => write!(f, "Ti41"),
            Self::Ti42 => write!(f, "Ti42"),
            Self::Ti43 => write!(f, "Ti43"),
            Self::Ti44 => write!(f, "Ti44"),
            Self::Ti45 => write!(f, "Ti45"),
            Self::Ti46 => write!(f, "Ti46"),
            Self::Ti47 => write!(f, "Ti47"),
            Self::Ti48 => write!(f, "Ti48"),
            Self::Ti49 => write!(f, "Ti49"),
            Self::Ti50 => write!(f, "Ti50"),
            Self::Ti51 => write!(f, "Ti51"),
            Self::Ti52 => write!(f, "Ti52"),
            Self::Ti53 => write!(f, "Ti53"),
            Self::Ti54 => write!(f, "Ti54"),
            Self::Ti55 => write!(f, "Ti55"),
            Self::Ti56 => write!(f, "Ti56"),
            Self::Ti57 => write!(f, "Ti57"),
            Self::Ti58 => write!(f, "Ti58"),
            Self::Ti59 => write!(f, "Ti59"),
            Self::Ti60 => write!(f, "Ti60"),
            Self::Ti61 => write!(f, "Ti61"),
            Self::Ti62 => write!(f, "Ti62"),
            Self::Ti63 => write!(f, "Ti63"),
        }
    }
}
