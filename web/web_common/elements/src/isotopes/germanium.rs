#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum GermaniumIsotope {
    Ge58,
    Ge59,
    Ge60,
    Ge61,
    Ge62,
    Ge63,
    Ge64,
    Ge65,
    Ge66,
    Ge67,
    Ge68,
    Ge69,
    Ge70,
    Ge71,
    Ge72,
    Ge73,
    Ge74,
    Ge75,
    Ge76,
    Ge77,
    Ge78,
    Ge79,
    Ge80,
    Ge81,
    Ge82,
    Ge83,
    Ge84,
    Ge85,
    Ge86,
    Ge87,
    Ge88,
    Ge89,
    Ge90,
}
impl super::RelativeAtomicMass for GermaniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ge58 => 57.99172f64,
            Self::Ge59 => 58.98249f64,
            Self::Ge60 => 59.97036f64,
            Self::Ge61 => 60.96379f64,
            Self::Ge62 => 61.95502f64,
            Self::Ge63 => 62.949628f64,
            Self::Ge64 => 63.9416899f64,
            Self::Ge65 => 64.9393681f64,
            Self::Ge66 => 65.9338621f64,
            Self::Ge67 => 66.9327339f64,
            Self::Ge68 => 67.9280953f64,
            Self::Ge69 => 68.9279645f64,
            Self::Ge70 => 69.92424875f64,
            Self::Ge71 => 70.92495233f64,
            Self::Ge72 => 71.922075826f64,
            Self::Ge73 => 72.923458956f64,
            Self::Ge74 => 73.921177761f64,
            Self::Ge75 => 74.92285837f64,
            Self::Ge76 => 75.921402726f64,
            Self::Ge77 => 76.923549843f64,
            Self::Ge78 => 77.9228529f64,
            Self::Ge79 => 78.92536f64,
            Self::Ge80 => 79.9253508f64,
            Self::Ge81 => 80.9288329f64,
            Self::Ge82 => 81.929774f64,
            Self::Ge83 => 82.9345391f64,
            Self::Ge84 => 83.9375751f64,
            Self::Ge85 => 84.9429697f64,
            Self::Ge86 => 85.94658f64,
            Self::Ge87 => 86.95268f64,
            Self::Ge88 => 87.95691f64,
            Self::Ge89 => 88.96379f64,
            Self::Ge90 => 89.96863f64,
        }
    }
}
impl super::ElementVariant for GermaniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ge
    }
}
impl super::MassNumber for GermaniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ge58 => 58u16,
            Self::Ge59 => 59u16,
            Self::Ge60 => 60u16,
            Self::Ge61 => 61u16,
            Self::Ge62 => 62u16,
            Self::Ge63 => 63u16,
            Self::Ge64 => 64u16,
            Self::Ge65 => 65u16,
            Self::Ge66 => 66u16,
            Self::Ge67 => 67u16,
            Self::Ge68 => 68u16,
            Self::Ge69 => 69u16,
            Self::Ge70 => 70u16,
            Self::Ge71 => 71u16,
            Self::Ge72 => 72u16,
            Self::Ge73 => 73u16,
            Self::Ge74 => 74u16,
            Self::Ge75 => 75u16,
            Self::Ge76 => 76u16,
            Self::Ge77 => 77u16,
            Self::Ge78 => 78u16,
            Self::Ge79 => 79u16,
            Self::Ge80 => 80u16,
            Self::Ge81 => 81u16,
            Self::Ge82 => 82u16,
            Self::Ge83 => 83u16,
            Self::Ge84 => 84u16,
            Self::Ge85 => 85u16,
            Self::Ge86 => 86u16,
            Self::Ge87 => 87u16,
            Self::Ge88 => 88u16,
            Self::Ge89 => 89u16,
            Self::Ge90 => 90u16,
        }
    }
}
impl super::IsotopicComposition for GermaniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ge58 => None,
            Self::Ge59 => None,
            Self::Ge60 => None,
            Self::Ge61 => None,
            Self::Ge62 => None,
            Self::Ge63 => None,
            Self::Ge64 => None,
            Self::Ge65 => None,
            Self::Ge66 => None,
            Self::Ge67 => None,
            Self::Ge68 => None,
            Self::Ge69 => None,
            Self::Ge70 => Some(0.2057f64),
            Self::Ge71 => None,
            Self::Ge72 => Some(0.2745f64),
            Self::Ge73 => Some(0.0775f64),
            Self::Ge74 => Some(0.365f64),
            Self::Ge75 => None,
            Self::Ge76 => Some(0.0773f64),
            Self::Ge77 => None,
            Self::Ge78 => None,
            Self::Ge79 => None,
            Self::Ge80 => None,
            Self::Ge81 => None,
            Self::Ge82 => None,
            Self::Ge83 => None,
            Self::Ge84 => None,
            Self::Ge85 => None,
            Self::Ge86 => None,
            Self::Ge87 => None,
            Self::Ge88 => None,
            Self::Ge89 => None,
            Self::Ge90 => None,
        }
    }
}
impl super::MostCommonIsotope for GermaniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Ge74
    }
}
