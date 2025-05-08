#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum BohriumIsotope {
    Bh260,
    Bh261,
    Bh262,
    Bh263,
    Bh264,
    Bh265,
    Bh266,
    Bh267,
    Bh268,
    Bh269,
    Bh270,
    Bh271,
    Bh272,
    Bh273,
    Bh274,
    Bh275,
}
impl super::RelativeAtomicMass for BohriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Bh260 => 260.12166f64,
            Self::Bh261 => 261.12145f64,
            Self::Bh262 => 262.12297f64,
            Self::Bh263 => 263.12292f64,
            Self::Bh264 => 264.12459f64,
            Self::Bh265 => 265.12491f64,
            Self::Bh266 => 266.12679f64,
            Self::Bh267 => 267.1275f64,
            Self::Bh268 => 268.12969f64,
            Self::Bh269 => 269.13042f64,
            Self::Bh270 => 270.13336f64,
            Self::Bh271 => 271.13526f64,
            Self::Bh272 => 272.13826f64,
            Self::Bh273 => 273.14024f64,
            Self::Bh274 => 274.14355f64,
            Self::Bh275 => 275.14567f64,
        }
    }
}
impl super::ElementVariant for BohriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Bh
    }
}
impl super::MassNumber for BohriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Bh260 => 260u16,
            Self::Bh261 => 261u16,
            Self::Bh262 => 262u16,
            Self::Bh263 => 263u16,
            Self::Bh264 => 264u16,
            Self::Bh265 => 265u16,
            Self::Bh266 => 266u16,
            Self::Bh267 => 267u16,
            Self::Bh268 => 268u16,
            Self::Bh269 => 269u16,
            Self::Bh270 => 270u16,
            Self::Bh271 => 271u16,
            Self::Bh272 => 272u16,
            Self::Bh273 => 273u16,
            Self::Bh274 => 274u16,
            Self::Bh275 => 275u16,
        }
    }
}
impl super::IsotopicComposition for BohriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Bh260 => None,
            Self::Bh261 => None,
            Self::Bh262 => None,
            Self::Bh263 => None,
            Self::Bh264 => None,
            Self::Bh265 => None,
            Self::Bh266 => None,
            Self::Bh267 => None,
            Self::Bh268 => None,
            Self::Bh269 => None,
            Self::Bh270 => None,
            Self::Bh271 => None,
            Self::Bh272 => None,
            Self::Bh273 => None,
            Self::Bh274 => None,
            Self::Bh275 => None,
        }
    }
}
impl super::MostCommonIsotope for BohriumIsotope {
    fn most_common_isotope() -> Self {
        Self::Bh275
    }
}
