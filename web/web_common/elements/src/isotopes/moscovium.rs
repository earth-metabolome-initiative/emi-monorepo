#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum MoscoviumIsotope {
    Mc287,
    Mc288,
    Mc289,
    Mc290,
    Uup291,
}
impl super::RelativeAtomicMass for MoscoviumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mc287 => 287.1907f64,
            Self::Mc288 => 288.19274f64,
            Self::Mc289 => 289.19363f64,
            Self::Mc290 => 290.19598f64,
            Self::Uup291 => 291.19707f64,
        }
    }
}
impl super::ElementVariant for MoscoviumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mc
    }
}
impl super::MassNumber for MoscoviumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mc287 => 287u16,
            Self::Mc288 => 288u16,
            Self::Mc289 => 289u16,
            Self::Mc290 => 290u16,
            Self::Uup291 => 291u16,
        }
    }
}
impl super::IsotopicComposition for MoscoviumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Mc287 => None,
            Self::Mc288 => None,
            Self::Mc289 => None,
            Self::Mc290 => None,
            Self::Uup291 => None,
        }
    }
}
impl super::MostCommonIsotope for MoscoviumIsotope {
    fn most_common_isotope() -> Self {
        Self::Uup291
    }
}
