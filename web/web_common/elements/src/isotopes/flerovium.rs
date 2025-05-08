#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum FleroviumIsotope {
    Fl285,
    Fl286,
    Fl287,
    Fl288,
    Fl289,
}
impl super::RelativeAtomicMass for FleroviumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Fl285 => 285.18364f64,
            Self::Fl286 => 286.18423f64,
            Self::Fl287 => 287.18678f64,
            Self::Fl288 => 288.18757f64,
            Self::Fl289 => 289.19042f64,
        }
    }
}
impl super::ElementVariant for FleroviumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Fl
    }
}
impl super::MassNumber for FleroviumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Fl285 => 285u16,
            Self::Fl286 => 286u16,
            Self::Fl287 => 287u16,
            Self::Fl288 => 288u16,
            Self::Fl289 => 289u16,
        }
    }
}
impl super::IsotopicComposition for FleroviumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Fl285 => None,
            Self::Fl286 => None,
            Self::Fl287 => None,
            Self::Fl288 => None,
            Self::Fl289 => None,
        }
    }
}
impl super::MostCommonIsotope for FleroviumIsotope {
    fn most_common_isotope() -> Self {
        Self::Fl289
    }
}
