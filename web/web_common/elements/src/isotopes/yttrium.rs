#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum YttriumIsotope {
    Y76,
    Y77,
    Y78,
    Y79,
    Y80,
    Y81,
    Y82,
    Y83,
    Y84,
    Y85,
    Y86,
    Y87,
    Y88,
    Y89,
    Y90,
    Y91,
    Y92,
    Y93,
    Y94,
    Y95,
    Y96,
    Y97,
    Y98,
    Y99,
    Y100,
    Y101,
    Y102,
    Y103,
    Y104,
    Y105,
    Y106,
    Y107,
    Y108,
    Y109,
}
impl super::RelativeAtomicMass for YttriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Y76 => 75.95856f64,
            Self::Y77 => 76.949781f64,
            Self::Y78 => 77.94361f64,
            Self::Y79 => 78.93735f64,
            Self::Y80 => 79.9343561f64,
            Self::Y81 => 80.9294556f64,
            Self::Y82 => 81.9269314f64,
            Self::Y83 => 82.922485f64,
            Self::Y84 => 83.9206721f64,
            Self::Y85 => 84.916433f64,
            Self::Y86 => 85.914886f64,
            Self::Y87 => 86.9108761f64,
            Self::Y88 => 87.9095016f64,
            Self::Y89 => 88.9058403f64,
            Self::Y90 => 89.9071439f64,
            Self::Y91 => 90.9072974f64,
            Self::Y92 => 91.9089451f64,
            Self::Y93 => 92.909578f64,
            Self::Y94 => 93.9115906f64,
            Self::Y95 => 94.9128161f64,
            Self::Y96 => 95.9158968f64,
            Self::Y97 => 96.9182741f64,
            Self::Y98 => 97.9223821f64,
            Self::Y99 => 98.924148f64,
            Self::Y100 => 99.927715f64,
            Self::Y101 => 100.9301477f64,
            Self::Y102 => 101.9343277f64,
            Self::Y103 => 102.937243f64,
            Self::Y104 => 103.94196f64,
            Self::Y105 => 104.94544f64,
            Self::Y106 => 105.95056f64,
            Self::Y107 => 106.95452f64,
            Self::Y108 => 107.95996f64,
            Self::Y109 => 108.96436f64,
        }
    }
}
impl super::ElementVariant for YttriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Y
    }
}
impl super::MassNumber for YttriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Y76 => 76u16,
            Self::Y77 => 77u16,
            Self::Y78 => 78u16,
            Self::Y79 => 79u16,
            Self::Y80 => 80u16,
            Self::Y81 => 81u16,
            Self::Y82 => 82u16,
            Self::Y83 => 83u16,
            Self::Y84 => 84u16,
            Self::Y85 => 85u16,
            Self::Y86 => 86u16,
            Self::Y87 => 87u16,
            Self::Y88 => 88u16,
            Self::Y89 => 89u16,
            Self::Y90 => 90u16,
            Self::Y91 => 91u16,
            Self::Y92 => 92u16,
            Self::Y93 => 93u16,
            Self::Y94 => 94u16,
            Self::Y95 => 95u16,
            Self::Y96 => 96u16,
            Self::Y97 => 97u16,
            Self::Y98 => 98u16,
            Self::Y99 => 99u16,
            Self::Y100 => 100u16,
            Self::Y101 => 101u16,
            Self::Y102 => 102u16,
            Self::Y103 => 103u16,
            Self::Y104 => 104u16,
            Self::Y105 => 105u16,
            Self::Y106 => 106u16,
            Self::Y107 => 107u16,
            Self::Y108 => 108u16,
            Self::Y109 => 109u16,
        }
    }
}
impl super::IsotopicComposition for YttriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Y76 => None,
            Self::Y77 => None,
            Self::Y78 => None,
            Self::Y79 => None,
            Self::Y80 => None,
            Self::Y81 => None,
            Self::Y82 => None,
            Self::Y83 => None,
            Self::Y84 => None,
            Self::Y85 => None,
            Self::Y86 => None,
            Self::Y87 => None,
            Self::Y88 => None,
            Self::Y89 => Some(1f64),
            Self::Y90 => None,
            Self::Y91 => None,
            Self::Y92 => None,
            Self::Y93 => None,
            Self::Y94 => None,
            Self::Y95 => None,
            Self::Y96 => None,
            Self::Y97 => None,
            Self::Y98 => None,
            Self::Y99 => None,
            Self::Y100 => None,
            Self::Y101 => None,
            Self::Y102 => None,
            Self::Y103 => None,
            Self::Y104 => None,
            Self::Y105 => None,
            Self::Y106 => None,
            Self::Y107 => None,
            Self::Y108 => None,
            Self::Y109 => None,
        }
    }
}
impl super::MostCommonIsotope for YttriumIsotope {
    fn most_common_isotope() -> Self {
        Self::Y89
    }
}
