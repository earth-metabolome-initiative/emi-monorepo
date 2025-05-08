#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum DubniumIsotope {
    Db255,
    Db256,
    Db257,
    Db258,
    Db259,
    Db260,
    Db261,
    Db262,
    Db263,
    Db264,
    Db265,
    Db266,
    Db267,
    Db268,
    Db269,
    Db270,
}
impl super::RelativeAtomicMass for DubniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Db255 => 255.10707f64,
            Self::Db256 => 256.10789f64,
            Self::Db257 => 257.10758f64,
            Self::Db258 => 258.10928f64,
            Self::Db259 => 259.109492f64,
            Self::Db260 => 260.1113f64,
            Self::Db261 => 261.11192f64,
            Self::Db262 => 262.11407f64,
            Self::Db263 => 263.11499f64,
            Self::Db264 => 264.11741f64,
            Self::Db265 => 265.11861f64,
            Self::Db266 => 266.12103f64,
            Self::Db267 => 267.12247f64,
            Self::Db268 => 268.12567f64,
            Self::Db269 => 269.12791f64,
            Self::Db270 => 270.13136f64,
        }
    }
}
impl super::ElementVariant for DubniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Db
    }
}
impl super::MassNumber for DubniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Db255 => 255u16,
            Self::Db256 => 256u16,
            Self::Db257 => 257u16,
            Self::Db258 => 258u16,
            Self::Db259 => 259u16,
            Self::Db260 => 260u16,
            Self::Db261 => 261u16,
            Self::Db262 => 262u16,
            Self::Db263 => 263u16,
            Self::Db264 => 264u16,
            Self::Db265 => 265u16,
            Self::Db266 => 266u16,
            Self::Db267 => 267u16,
            Self::Db268 => 268u16,
            Self::Db269 => 269u16,
            Self::Db270 => 270u16,
        }
    }
}
impl super::IsotopicComposition for DubniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Db255 => None,
            Self::Db256 => None,
            Self::Db257 => None,
            Self::Db258 => None,
            Self::Db259 => None,
            Self::Db260 => None,
            Self::Db261 => None,
            Self::Db262 => None,
            Self::Db263 => None,
            Self::Db264 => None,
            Self::Db265 => None,
            Self::Db266 => None,
            Self::Db267 => None,
            Self::Db268 => None,
            Self::Db269 => None,
            Self::Db270 => None,
        }
    }
}
impl super::MostCommonIsotope for DubniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Db270
    }
}
