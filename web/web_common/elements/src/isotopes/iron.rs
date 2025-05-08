#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum IronIsotope {
    Fe45,
    Fe46,
    Fe47,
    Fe48,
    Fe49,
    Fe50,
    Fe51,
    Fe52,
    Fe53,
    Fe54,
    Fe55,
    Fe56,
    Fe57,
    Fe58,
    Fe59,
    Fe60,
    Fe61,
    Fe62,
    Fe63,
    Fe64,
    Fe65,
    Fe66,
    Fe67,
    Fe68,
    Fe69,
    Fe70,
    Fe71,
    Fe72,
    Fe73,
    Fe74,
}
impl super::RelativeAtomicMass for IronIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Fe45 => 45.01442f64,
            Self::Fe46 => 46.00063f64,
            Self::Fe47 => 46.99185f64,
            Self::Fe48 => 47.98023f64,
            Self::Fe49 => 48.973429f64,
            Self::Fe50 => 49.962975f64,
            Self::Fe51 => 50.956841f64,
            Self::Fe52 => 51.9481131f64,
            Self::Fe53 => 52.9453064f64,
            Self::Fe54 => 53.93960899f64,
            Self::Fe55 => 54.93829199f64,
            Self::Fe56 => 55.93493633f64,
            Self::Fe57 => 56.93539284f64,
            Self::Fe58 => 57.93327443f64,
            Self::Fe59 => 58.93487434f64,
            Self::Fe60 => 59.9340711f64,
            Self::Fe61 => 60.9367462f64,
            Self::Fe62 => 61.9367918f64,
            Self::Fe63 => 62.9402727f64,
            Self::Fe64 => 63.9409878f64,
            Self::Fe65 => 64.9450115f64,
            Self::Fe66 => 65.94625f64,
            Self::Fe67 => 66.95054f64,
            Self::Fe68 => 67.95295f64,
            Self::Fe69 => 68.95807f64,
            Self::Fe70 => 69.96102f64,
            Self::Fe71 => 70.96672f64,
            Self::Fe72 => 71.96983f64,
            Self::Fe73 => 72.97572f64,
            Self::Fe74 => 73.97935f64,
        }
    }
}
impl super::ElementVariant for IronIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Fe
    }
}
impl super::MassNumber for IronIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Fe45 => 45u16,
            Self::Fe46 => 46u16,
            Self::Fe47 => 47u16,
            Self::Fe48 => 48u16,
            Self::Fe49 => 49u16,
            Self::Fe50 => 50u16,
            Self::Fe51 => 51u16,
            Self::Fe52 => 52u16,
            Self::Fe53 => 53u16,
            Self::Fe54 => 54u16,
            Self::Fe55 => 55u16,
            Self::Fe56 => 56u16,
            Self::Fe57 => 57u16,
            Self::Fe58 => 58u16,
            Self::Fe59 => 59u16,
            Self::Fe60 => 60u16,
            Self::Fe61 => 61u16,
            Self::Fe62 => 62u16,
            Self::Fe63 => 63u16,
            Self::Fe64 => 64u16,
            Self::Fe65 => 65u16,
            Self::Fe66 => 66u16,
            Self::Fe67 => 67u16,
            Self::Fe68 => 68u16,
            Self::Fe69 => 69u16,
            Self::Fe70 => 70u16,
            Self::Fe71 => 71u16,
            Self::Fe72 => 72u16,
            Self::Fe73 => 73u16,
            Self::Fe74 => 74u16,
        }
    }
}
impl super::IsotopicComposition for IronIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Fe45 => None,
            Self::Fe46 => None,
            Self::Fe47 => None,
            Self::Fe48 => None,
            Self::Fe49 => None,
            Self::Fe50 => None,
            Self::Fe51 => None,
            Self::Fe52 => None,
            Self::Fe53 => None,
            Self::Fe54 => Some(0.05845f64),
            Self::Fe55 => None,
            Self::Fe56 => Some(0.91754f64),
            Self::Fe57 => Some(0.02119f64),
            Self::Fe58 => Some(0.00282f64),
            Self::Fe59 => None,
            Self::Fe60 => None,
            Self::Fe61 => None,
            Self::Fe62 => None,
            Self::Fe63 => None,
            Self::Fe64 => None,
            Self::Fe65 => None,
            Self::Fe66 => None,
            Self::Fe67 => None,
            Self::Fe68 => None,
            Self::Fe69 => None,
            Self::Fe70 => None,
            Self::Fe71 => None,
            Self::Fe72 => None,
            Self::Fe73 => None,
            Self::Fe74 => None,
        }
    }
}
impl super::MostCommonIsotope for IronIsotope {
    fn most_common_isotope() -> Self {
        Self::Fe56
    }
}
