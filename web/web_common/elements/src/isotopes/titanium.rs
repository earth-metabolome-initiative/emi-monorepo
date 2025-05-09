#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum TitaniumIsotope {
    Ti38,
    Ti39,
    Ti40,
    Ti41,
    Ti42,
    Ti43,
    Ti44,
    Ti45,
    Ti46,
    Ti47,
    Ti48,
    Ti49,
    Ti50,
    Ti51,
    Ti52,
    Ti53,
    Ti54,
    Ti55,
    Ti56,
    Ti57,
    Ti58,
    Ti59,
    Ti60,
    Ti61,
    Ti62,
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
            Self::Ti38 => None,
            Self::Ti39 => None,
            Self::Ti40 => None,
            Self::Ti41 => None,
            Self::Ti42 => None,
            Self::Ti43 => None,
            Self::Ti44 => None,
            Self::Ti45 => None,
            Self::Ti46 => Some(0.0825f64),
            Self::Ti47 => Some(0.0744f64),
            Self::Ti48 => Some(0.7372f64),
            Self::Ti49 => Some(0.0541f64),
            Self::Ti50 => Some(0.0518f64),
            Self::Ti51 => None,
            Self::Ti52 => None,
            Self::Ti53 => None,
            Self::Ti54 => None,
            Self::Ti55 => None,
            Self::Ti56 => None,
            Self::Ti57 => None,
            Self::Ti58 => None,
            Self::Ti59 => None,
            Self::Ti60 => None,
            Self::Ti61 => None,
            Self::Ti62 => None,
            Self::Ti63 => None,
        }
    }
}
impl super::MostAbundantIsotope for TitaniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ti48
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
