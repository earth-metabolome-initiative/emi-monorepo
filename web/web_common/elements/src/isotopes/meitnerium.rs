#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum MeitneriumIsotope {
    Mt265,
    Mt266,
    Mt267,
    Mt268,
    Mt269,
    Mt270,
    Mt271,
    Mt272,
    Mt273,
    Mt274,
    Mt275,
    Mt276,
    Mt277,
    Mt278,
    Mt279,
}
impl super::RelativeAtomicMass for MeitneriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mt265 => 265.136f64,
            Self::Mt266 => 266.13737f64,
            Self::Mt267 => 267.13719f64,
            Self::Mt268 => 268.13865f64,
            Self::Mt269 => 269.13882f64,
            Self::Mt270 => 270.14033f64,
            Self::Mt271 => 271.14074f64,
            Self::Mt272 => 272.14341f64,
            Self::Mt273 => 273.1444f64,
            Self::Mt274 => 274.14724f64,
            Self::Mt275 => 275.14882f64,
            Self::Mt276 => 276.15159f64,
            Self::Mt277 => 277.15327f64,
            Self::Mt278 => 278.15631f64,
            Self::Mt279 => 279.15808f64,
        }
    }
}
impl super::ElementVariant for MeitneriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mt
    }
}
impl super::MassNumber for MeitneriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mt265 => 265u16,
            Self::Mt266 => 266u16,
            Self::Mt267 => 267u16,
            Self::Mt268 => 268u16,
            Self::Mt269 => 269u16,
            Self::Mt270 => 270u16,
            Self::Mt271 => 271u16,
            Self::Mt272 => 272u16,
            Self::Mt273 => 273u16,
            Self::Mt274 => 274u16,
            Self::Mt275 => 275u16,
            Self::Mt276 => 276u16,
            Self::Mt277 => 277u16,
            Self::Mt278 => 278u16,
            Self::Mt279 => 279u16,
        }
    }
}
impl super::IsotopicComposition for MeitneriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Mt265 => None,
            Self::Mt266 => None,
            Self::Mt267 => None,
            Self::Mt268 => None,
            Self::Mt269 => None,
            Self::Mt270 => None,
            Self::Mt271 => None,
            Self::Mt272 => None,
            Self::Mt273 => None,
            Self::Mt274 => None,
            Self::Mt275 => None,
            Self::Mt276 => None,
            Self::Mt277 => None,
            Self::Mt278 => None,
            Self::Mt279 => None,
        }
    }
}
impl super::MostCommonIsotope for MeitneriumIsotope {
    fn most_common_isotope() -> Self {
        Self::Mt279
    }
}
