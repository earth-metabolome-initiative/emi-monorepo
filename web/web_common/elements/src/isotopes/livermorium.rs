#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum LivermoriumIsotope {
    Lv289,
    Lv290,
    Lv291,
    Lv292,
    Lv293,
}
impl super::RelativeAtomicMass for LivermoriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Lv289 => 289.19816f64,
            Self::Lv290 => 290.19864f64,
            Self::Lv291 => 291.20108f64,
            Self::Lv292 => 292.20174f64,
            Self::Lv293 => 293.20449f64,
        }
    }
}
impl super::ElementVariant for LivermoriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Lv
    }
}
impl super::MassNumber for LivermoriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Lv289 => 289u16,
            Self::Lv290 => 290u16,
            Self::Lv291 => 291u16,
            Self::Lv292 => 292u16,
            Self::Lv293 => 293u16,
        }
    }
}
impl super::IsotopicComposition for LivermoriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Lv289 => None,
            Self::Lv290 => None,
            Self::Lv291 => None,
            Self::Lv292 => None,
            Self::Lv293 => None,
        }
    }
}
impl super::MostCommonIsotope for LivermoriumIsotope {
    fn most_common_isotope() -> Self {
        Self::Lv293
    }
}
