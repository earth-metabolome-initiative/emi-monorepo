#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ScandiumIsotope {
    Sc36,
    Sc37,
    Sc38,
    Sc39,
    Sc40,
    Sc41,
    Sc42,
    Sc43,
    Sc44,
    Sc45,
    Sc46,
    Sc47,
    Sc48,
    Sc49,
    Sc50,
    Sc51,
    Sc52,
    Sc53,
    Sc54,
    Sc55,
    Sc56,
    Sc57,
    Sc58,
    Sc59,
    Sc60,
    Sc61,
}
impl super::RelativeAtomicMass for ScandiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Sc36 => 36.01648f64,
            Self::Sc37 => 37.00374f64,
            Self::Sc38 => 37.99512f64,
            Self::Sc39 => 38.984785f64,
            Self::Sc40 => 39.9779673f64,
            Self::Sc41 => 40.969251105f64,
            Self::Sc42 => 41.96551653f64,
            Self::Sc43 => 42.9611505f64,
            Self::Sc44 => 43.9594029f64,
            Self::Sc45 => 44.95590828f64,
            Self::Sc46 => 45.95516826f64,
            Self::Sc47 => 46.9524037f64,
            Self::Sc48 => 47.9522236f64,
            Self::Sc49 => 48.9500146f64,
            Self::Sc50 => 49.952176f64,
            Self::Sc51 => 50.953592f64,
            Self::Sc52 => 51.95688f64,
            Self::Sc53 => 52.95909f64,
            Self::Sc54 => 53.96393f64,
            Self::Sc55 => 54.96782f64,
            Self::Sc56 => 55.97345f64,
            Self::Sc57 => 56.97777f64,
            Self::Sc58 => 57.98403f64,
            Self::Sc59 => 58.98894f64,
            Self::Sc60 => 59.99565f64,
            Self::Sc61 => 61.001f64,
        }
    }
}
impl super::ElementVariant for ScandiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Sc
    }
}
impl super::MassNumber for ScandiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Sc36 => 36u16,
            Self::Sc37 => 37u16,
            Self::Sc38 => 38u16,
            Self::Sc39 => 39u16,
            Self::Sc40 => 40u16,
            Self::Sc41 => 41u16,
            Self::Sc42 => 42u16,
            Self::Sc43 => 43u16,
            Self::Sc44 => 44u16,
            Self::Sc45 => 45u16,
            Self::Sc46 => 46u16,
            Self::Sc47 => 47u16,
            Self::Sc48 => 48u16,
            Self::Sc49 => 49u16,
            Self::Sc50 => 50u16,
            Self::Sc51 => 51u16,
            Self::Sc52 => 52u16,
            Self::Sc53 => 53u16,
            Self::Sc54 => 54u16,
            Self::Sc55 => 55u16,
            Self::Sc56 => 56u16,
            Self::Sc57 => 57u16,
            Self::Sc58 => 58u16,
            Self::Sc59 => 59u16,
            Self::Sc60 => 60u16,
            Self::Sc61 => 61u16,
        }
    }
}
impl super::IsotopicComposition for ScandiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Sc36 => None,
            Self::Sc37 => None,
            Self::Sc38 => None,
            Self::Sc39 => None,
            Self::Sc40 => None,
            Self::Sc41 => None,
            Self::Sc42 => None,
            Self::Sc43 => None,
            Self::Sc44 => None,
            Self::Sc45 => Some(1f64),
            Self::Sc46 => None,
            Self::Sc47 => None,
            Self::Sc48 => None,
            Self::Sc49 => None,
            Self::Sc50 => None,
            Self::Sc51 => None,
            Self::Sc52 => None,
            Self::Sc53 => None,
            Self::Sc54 => None,
            Self::Sc55 => None,
            Self::Sc56 => None,
            Self::Sc57 => None,
            Self::Sc58 => None,
            Self::Sc59 => None,
            Self::Sc60 => None,
            Self::Sc61 => None,
        }
    }
}
impl super::MostCommonIsotope for ScandiumIsotope {
    fn most_common_isotope() -> Self {
        Self::Sc45
    }
}
