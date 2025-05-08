#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ChromiumIsotope {
    Cr42,
    Cr43,
    Cr44,
    Cr45,
    Cr46,
    Cr47,
    Cr48,
    Cr49,
    Cr50,
    Cr51,
    Cr52,
    Cr53,
    Cr54,
    Cr55,
    Cr56,
    Cr57,
    Cr58,
    Cr59,
    Cr60,
    Cr61,
    Cr62,
    Cr63,
    Cr64,
    Cr65,
    Cr66,
    Cr67,
    Cr68,
}
impl super::RelativeAtomicMass for ChromiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cr42 => 42.0067f64,
            Self::Cr43 => 42.99753f64,
            Self::Cr44 => 43.98536f64,
            Self::Cr45 => 44.97905f64,
            Self::Cr46 => 45.968359f64,
            Self::Cr47 => 46.9628974f64,
            Self::Cr48 => 47.9540291f64,
            Self::Cr49 => 48.9513333f64,
            Self::Cr50 => 49.94604183f64,
            Self::Cr51 => 50.94476502f64,
            Self::Cr52 => 51.94050623f64,
            Self::Cr53 => 52.94064815f64,
            Self::Cr54 => 53.93887916f64,
            Self::Cr55 => 54.94083843f64,
            Self::Cr56 => 55.9406531f64,
            Self::Cr57 => 56.943613f64,
            Self::Cr58 => 57.94435f64,
            Self::Cr59 => 58.94859f64,
            Self::Cr60 => 59.95008f64,
            Self::Cr61 => 60.95442f64,
            Self::Cr62 => 61.9561f64,
            Self::Cr63 => 62.96165f64,
            Self::Cr64 => 63.96408f64,
            Self::Cr65 => 64.96996f64,
            Self::Cr66 => 65.97366f64,
            Self::Cr67 => 66.98016f64,
            Self::Cr68 => 67.98403f64,
        }
    }
}
impl super::ElementVariant for ChromiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cr
    }
}
impl super::MassNumber for ChromiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cr42 => 42u16,
            Self::Cr43 => 43u16,
            Self::Cr44 => 44u16,
            Self::Cr45 => 45u16,
            Self::Cr46 => 46u16,
            Self::Cr47 => 47u16,
            Self::Cr48 => 48u16,
            Self::Cr49 => 49u16,
            Self::Cr50 => 50u16,
            Self::Cr51 => 51u16,
            Self::Cr52 => 52u16,
            Self::Cr53 => 53u16,
            Self::Cr54 => 54u16,
            Self::Cr55 => 55u16,
            Self::Cr56 => 56u16,
            Self::Cr57 => 57u16,
            Self::Cr58 => 58u16,
            Self::Cr59 => 59u16,
            Self::Cr60 => 60u16,
            Self::Cr61 => 61u16,
            Self::Cr62 => 62u16,
            Self::Cr63 => 63u16,
            Self::Cr64 => 64u16,
            Self::Cr65 => 65u16,
            Self::Cr66 => 66u16,
            Self::Cr67 => 67u16,
            Self::Cr68 => 68u16,
        }
    }
}
impl super::IsotopicComposition for ChromiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cr42 => None,
            Self::Cr43 => None,
            Self::Cr44 => None,
            Self::Cr45 => None,
            Self::Cr46 => None,
            Self::Cr47 => None,
            Self::Cr48 => None,
            Self::Cr49 => None,
            Self::Cr50 => Some(0.04345f64),
            Self::Cr51 => None,
            Self::Cr52 => Some(0.83789f64),
            Self::Cr53 => Some(0.09501f64),
            Self::Cr54 => Some(0.02365f64),
            Self::Cr55 => None,
            Self::Cr56 => None,
            Self::Cr57 => None,
            Self::Cr58 => None,
            Self::Cr59 => None,
            Self::Cr60 => None,
            Self::Cr61 => None,
            Self::Cr62 => None,
            Self::Cr63 => None,
            Self::Cr64 => None,
            Self::Cr65 => None,
            Self::Cr66 => None,
            Self::Cr67 => None,
            Self::Cr68 => None,
        }
    }
}
impl super::MostCommonIsotope for ChromiumIsotope {
    fn most_common_isotope() -> Self {
        Self::Cr52
    }
}
