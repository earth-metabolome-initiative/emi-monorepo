#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum CoperniciumIsotope {
    Cn276,
    Cn277,
    Cn278,
    Cn279,
    Cn280,
    Cn281,
    Cn282,
    Cn283,
    Cn284,
    Cn285,
}
impl super::RelativeAtomicMass for CoperniciumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cn276 => 276.16141f64,
            Self::Cn277 => 277.16364f64,
            Self::Cn278 => 278.16416f64,
            Self::Cn279 => 279.16654f64,
            Self::Cn280 => 280.16715f64,
            Self::Cn281 => 281.16975f64,
            Self::Cn282 => 282.1705f64,
            Self::Cn283 => 283.17327f64,
            Self::Cn284 => 284.17416f64,
            Self::Cn285 => 285.17712f64,
        }
    }
}
impl super::ElementVariant for CoperniciumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cn
    }
}
impl super::MassNumber for CoperniciumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cn276 => 276u16,
            Self::Cn277 => 277u16,
            Self::Cn278 => 278u16,
            Self::Cn279 => 279u16,
            Self::Cn280 => 280u16,
            Self::Cn281 => 281u16,
            Self::Cn282 => 282u16,
            Self::Cn283 => 283u16,
            Self::Cn284 => 284u16,
            Self::Cn285 => 285u16,
        }
    }
}
impl super::IsotopicComposition for CoperniciumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cn276 => None,
            Self::Cn277 => None,
            Self::Cn278 => None,
            Self::Cn279 => None,
            Self::Cn280 => None,
            Self::Cn281 => None,
            Self::Cn282 => None,
            Self::Cn283 => None,
            Self::Cn284 => None,
            Self::Cn285 => None,
        }
    }
}
impl super::MostCommonIsotope for CoperniciumIsotope {
    fn most_common_isotope() -> Self {
        Self::Cn285
    }
}
