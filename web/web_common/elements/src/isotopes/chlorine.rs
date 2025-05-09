#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ChlorineIsotope {
    Cl28,
    Cl29,
    Cl30,
    Cl31,
    Cl32,
    Cl33,
    Cl34,
    Cl35,
    Cl36,
    Cl37,
    Cl38,
    Cl39,
    Cl40,
    Cl41,
    Cl42,
    Cl43,
    Cl44,
    Cl45,
    Cl46,
    Cl47,
    Cl48,
    Cl49,
    Cl50,
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
            Self::Cl28 => None,
            Self::Cl29 => None,
            Self::Cl30 => None,
            Self::Cl31 => None,
            Self::Cl32 => None,
            Self::Cl33 => None,
            Self::Cl34 => None,
            Self::Cl35 => Some(0.7576f64),
            Self::Cl36 => None,
            Self::Cl37 => Some(0.2424f64),
            Self::Cl38 => None,
            Self::Cl39 => None,
            Self::Cl40 => None,
            Self::Cl41 => None,
            Self::Cl42 => None,
            Self::Cl43 => None,
            Self::Cl44 => None,
            Self::Cl45 => None,
            Self::Cl46 => None,
            Self::Cl47 => None,
            Self::Cl48 => None,
            Self::Cl49 => None,
            Self::Cl50 => None,
            Self::Cl51 => None,
        }
    }
}
impl super::MostAbundantIsotope for ChlorineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cl35
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
