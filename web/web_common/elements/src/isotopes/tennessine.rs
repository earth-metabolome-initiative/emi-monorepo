#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum TennessineIsotope {
    Ts291,
    Ts292,
    Ts293,
    Uus294,
}
impl super::RelativeAtomicMass for TennessineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ts291 => 291.20553f64,
            Self::Ts292 => 292.20746f64,
            Self::Ts293 => 293.20824f64,
            Self::Uus294 => 294.21046f64,
        }
    }
}
impl super::ElementVariant for TennessineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ts
    }
}
impl super::MassNumber for TennessineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ts291 => 291u16,
            Self::Ts292 => 292u16,
            Self::Ts293 => 293u16,
            Self::Uus294 => 294u16,
        }
    }
}
impl super::IsotopicComposition for TennessineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ts291 => None,
            Self::Ts292 => None,
            Self::Ts293 => None,
            Self::Uus294 => None,
        }
    }
}
impl super::MostCommonIsotope for TennessineIsotope {
    fn most_common_isotope() -> Self {
        Self::Uus294
    }
}
