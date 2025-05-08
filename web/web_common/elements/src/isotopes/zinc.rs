#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ZincIsotope {
    Zn54,
    Zn55,
    Zn56,
    Zn57,
    Zn58,
    Zn59,
    Zn60,
    Zn61,
    Zn62,
    Zn63,
    Zn64,
    Zn65,
    Zn66,
    Zn67,
    Zn68,
    Zn69,
    Zn70,
    Zn71,
    Zn72,
    Zn73,
    Zn74,
    Zn75,
    Zn76,
    Zn77,
    Zn78,
    Zn79,
    Zn80,
    Zn81,
    Zn82,
    Zn83,
    Zn84,
    Zn85,
}
impl super::RelativeAtomicMass for ZincIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Zn54 => 53.99204f64,
            Self::Zn55 => 54.98398f64,
            Self::Zn56 => 55.97254f64,
            Self::Zn57 => 56.96506f64,
            Self::Zn58 => 57.954591f64,
            Self::Zn59 => 58.94931266f64,
            Self::Zn60 => 59.9418421f64,
            Self::Zn61 => 60.939507f64,
            Self::Zn62 => 61.93433397f64,
            Self::Zn63 => 62.9332115f64,
            Self::Zn64 => 63.92914201f64,
            Self::Zn65 => 64.92924077f64,
            Self::Zn66 => 65.92603381f64,
            Self::Zn67 => 66.92712775f64,
            Self::Zn68 => 67.92484455f64,
            Self::Zn69 => 68.9265507f64,
            Self::Zn70 => 69.9253192f64,
            Self::Zn71 => 70.9277196f64,
            Self::Zn72 => 71.9268428f64,
            Self::Zn73 => 72.9295826f64,
            Self::Zn74 => 73.9294073f64,
            Self::Zn75 => 74.9328402f64,
            Self::Zn76 => 75.933115f64,
            Self::Zn77 => 76.9368872f64,
            Self::Zn78 => 77.9382892f64,
            Self::Zn79 => 78.9426381f64,
            Self::Zn80 => 79.9445529f64,
            Self::Zn81 => 80.9504026f64,
            Self::Zn82 => 81.95426f64,
            Self::Zn83 => 82.96056f64,
            Self::Zn84 => 83.96521f64,
            Self::Zn85 => 84.97226f64,
        }
    }
}
impl super::ElementVariant for ZincIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Zn
    }
}
impl super::MassNumber for ZincIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Zn54 => 54u16,
            Self::Zn55 => 55u16,
            Self::Zn56 => 56u16,
            Self::Zn57 => 57u16,
            Self::Zn58 => 58u16,
            Self::Zn59 => 59u16,
            Self::Zn60 => 60u16,
            Self::Zn61 => 61u16,
            Self::Zn62 => 62u16,
            Self::Zn63 => 63u16,
            Self::Zn64 => 64u16,
            Self::Zn65 => 65u16,
            Self::Zn66 => 66u16,
            Self::Zn67 => 67u16,
            Self::Zn68 => 68u16,
            Self::Zn69 => 69u16,
            Self::Zn70 => 70u16,
            Self::Zn71 => 71u16,
            Self::Zn72 => 72u16,
            Self::Zn73 => 73u16,
            Self::Zn74 => 74u16,
            Self::Zn75 => 75u16,
            Self::Zn76 => 76u16,
            Self::Zn77 => 77u16,
            Self::Zn78 => 78u16,
            Self::Zn79 => 79u16,
            Self::Zn80 => 80u16,
            Self::Zn81 => 81u16,
            Self::Zn82 => 82u16,
            Self::Zn83 => 83u16,
            Self::Zn84 => 84u16,
            Self::Zn85 => 85u16,
        }
    }
}
impl super::IsotopicComposition for ZincIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Zn54 => None,
            Self::Zn55 => None,
            Self::Zn56 => None,
            Self::Zn57 => None,
            Self::Zn58 => None,
            Self::Zn59 => None,
            Self::Zn60 => None,
            Self::Zn61 => None,
            Self::Zn62 => None,
            Self::Zn63 => None,
            Self::Zn64 => Some(0.4917f64),
            Self::Zn65 => None,
            Self::Zn66 => Some(0.2773f64),
            Self::Zn67 => Some(0.0404f64),
            Self::Zn68 => Some(0.1845f64),
            Self::Zn69 => None,
            Self::Zn70 => Some(0.0061f64),
            Self::Zn71 => None,
            Self::Zn72 => None,
            Self::Zn73 => None,
            Self::Zn74 => None,
            Self::Zn75 => None,
            Self::Zn76 => None,
            Self::Zn77 => None,
            Self::Zn78 => None,
            Self::Zn79 => None,
            Self::Zn80 => None,
            Self::Zn81 => None,
            Self::Zn82 => None,
            Self::Zn83 => None,
            Self::Zn84 => None,
            Self::Zn85 => None,
        }
    }
}
impl super::MostCommonIsotope for ZincIsotope {
    fn most_common_isotope() -> Self {
        Self::Zn64
    }
}
