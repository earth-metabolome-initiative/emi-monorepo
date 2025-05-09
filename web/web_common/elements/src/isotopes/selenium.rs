#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum SeleniumIsotope {
    Se64,
    Se65,
    Se66,
    Se67,
    Se68,
    Se69,
    Se70,
    Se71,
    Se72,
    Se73,
    Se74,
    Se75,
    Se76,
    Se77,
    Se78,
    Se79,
    Se80,
    Se81,
    Se82,
    Se83,
    Se84,
    Se85,
    Se86,
    Se87,
    Se88,
    Se89,
    Se90,
    Se91,
    Se92,
    Se93,
    Se94,
    Se95,
}
impl super::RelativeAtomicMass for SeleniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Se64 => 63.97109f64,
            Self::Se65 => 64.9644f64,
            Self::Se66 => 65.95559f64,
            Self::Se67 => 66.949994f64,
            Self::Se68 => 67.94182524f64,
            Self::Se69 => 68.9394148f64,
            Self::Se70 => 69.9335155f64,
            Self::Se71 => 70.9322094f64,
            Self::Se72 => 71.9271405f64,
            Self::Se73 => 72.9267549f64,
            Self::Se74 => 73.922475934f64,
            Self::Se75 => 74.92252287f64,
            Self::Se76 => 75.919213704f64,
            Self::Se77 => 76.919914154f64,
            Self::Se78 => 77.91730928f64,
            Self::Se79 => 78.91849929f64,
            Self::Se80 => 79.9165218f64,
            Self::Se81 => 80.917993f64,
            Self::Se82 => 81.9166995f64,
            Self::Se83 => 82.9191186f64,
            Self::Se84 => 83.9184668f64,
            Self::Se85 => 84.9222608f64,
            Self::Se86 => 85.9243117f64,
            Self::Se87 => 86.9286886f64,
            Self::Se88 => 87.9314175f64,
            Self::Se89 => 88.9366691f64,
            Self::Se90 => 89.9401f64,
            Self::Se91 => 90.94596f64,
            Self::Se92 => 91.94984f64,
            Self::Se93 => 92.95629f64,
            Self::Se94 => 93.96049f64,
            Self::Se95 => 94.9673f64,
        }
    }
}
impl super::ElementVariant for SeleniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Se
    }
}
impl super::MassNumber for SeleniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Se64 => 64u16,
            Self::Se65 => 65u16,
            Self::Se66 => 66u16,
            Self::Se67 => 67u16,
            Self::Se68 => 68u16,
            Self::Se69 => 69u16,
            Self::Se70 => 70u16,
            Self::Se71 => 71u16,
            Self::Se72 => 72u16,
            Self::Se73 => 73u16,
            Self::Se74 => 74u16,
            Self::Se75 => 75u16,
            Self::Se76 => 76u16,
            Self::Se77 => 77u16,
            Self::Se78 => 78u16,
            Self::Se79 => 79u16,
            Self::Se80 => 80u16,
            Self::Se81 => 81u16,
            Self::Se82 => 82u16,
            Self::Se83 => 83u16,
            Self::Se84 => 84u16,
            Self::Se85 => 85u16,
            Self::Se86 => 86u16,
            Self::Se87 => 87u16,
            Self::Se88 => 88u16,
            Self::Se89 => 89u16,
            Self::Se90 => 90u16,
            Self::Se91 => 91u16,
            Self::Se92 => 92u16,
            Self::Se93 => 93u16,
            Self::Se94 => 94u16,
            Self::Se95 => 95u16,
        }
    }
}
impl super::IsotopicComposition for SeleniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Se64 => None,
            Self::Se65 => None,
            Self::Se66 => None,
            Self::Se67 => None,
            Self::Se68 => None,
            Self::Se69 => None,
            Self::Se70 => None,
            Self::Se71 => None,
            Self::Se72 => None,
            Self::Se73 => None,
            Self::Se74 => Some(0.0089f64),
            Self::Se75 => None,
            Self::Se76 => Some(0.0937f64),
            Self::Se77 => Some(0.0763f64),
            Self::Se78 => Some(0.2377f64),
            Self::Se79 => None,
            Self::Se80 => Some(0.4961f64),
            Self::Se81 => None,
            Self::Se82 => Some(0.0873f64),
            Self::Se83 => None,
            Self::Se84 => None,
            Self::Se85 => None,
            Self::Se86 => None,
            Self::Se87 => None,
            Self::Se88 => None,
            Self::Se89 => None,
            Self::Se90 => None,
            Self::Se91 => None,
            Self::Se92 => None,
            Self::Se93 => None,
            Self::Se94 => None,
            Self::Se95 => None,
        }
    }
}
impl super::MostAbundantIsotope for SeleniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Se80
    }
}
impl std::fmt::Display for SeleniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Se64 => write!(f, "Se64"),
            Self::Se65 => write!(f, "Se65"),
            Self::Se66 => write!(f, "Se66"),
            Self::Se67 => write!(f, "Se67"),
            Self::Se68 => write!(f, "Se68"),
            Self::Se69 => write!(f, "Se69"),
            Self::Se70 => write!(f, "Se70"),
            Self::Se71 => write!(f, "Se71"),
            Self::Se72 => write!(f, "Se72"),
            Self::Se73 => write!(f, "Se73"),
            Self::Se74 => write!(f, "Se74"),
            Self::Se75 => write!(f, "Se75"),
            Self::Se76 => write!(f, "Se76"),
            Self::Se77 => write!(f, "Se77"),
            Self::Se78 => write!(f, "Se78"),
            Self::Se79 => write!(f, "Se79"),
            Self::Se80 => write!(f, "Se80"),
            Self::Se81 => write!(f, "Se81"),
            Self::Se82 => write!(f, "Se82"),
            Self::Se83 => write!(f, "Se83"),
            Self::Se84 => write!(f, "Se84"),
            Self::Se85 => write!(f, "Se85"),
            Self::Se86 => write!(f, "Se86"),
            Self::Se87 => write!(f, "Se87"),
            Self::Se88 => write!(f, "Se88"),
            Self::Se89 => write!(f, "Se89"),
            Self::Se90 => write!(f, "Se90"),
            Self::Se91 => write!(f, "Se91"),
            Self::Se92 => write!(f, "Se92"),
            Self::Se93 => write!(f, "Se93"),
            Self::Se94 => write!(f, "Se94"),
            Self::Se95 => write!(f, "Se95"),
        }
    }
}
