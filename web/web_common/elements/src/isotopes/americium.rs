#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum AmericiumIsotope {
    Am230,
    Am231,
    Am232,
    Am233,
    Am234,
    Am235,
    Am236,
    Am237,
    Am238,
    Am239,
    Am240,
    Am241,
    Am242,
    Am243,
    Am244,
    Am245,
    Am246,
    Am247,
    Am248,
    Am249,
}
impl super::RelativeAtomicMass for AmericiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Am230 => 230.04609f64,
            Self::Am231 => 231.04556f64,
            Self::Am232 => 232.04645f64,
            Self::Am233 => 233.04644f64,
            Self::Am234 => 234.04773f64,
            Self::Am235 => 235.047908f64,
            Self::Am236 => 236.04943f64,
            Self::Am237 => 237.049996f64,
            Self::Am238 => 238.051985f64,
            Self::Am239 => 239.0530247f64,
            Self::Am240 => 240.0553f64,
            Self::Am241 => 241.0568293f64,
            Self::Am242 => 242.0595494f64,
            Self::Am243 => 243.0613813f64,
            Self::Am244 => 244.0642851f64,
            Self::Am245 => 245.0664548f64,
            Self::Am246 => 246.069775f64,
            Self::Am247 => 247.07209f64,
            Self::Am248 => 248.07575f64,
            Self::Am249 => 249.07848f64,
        }
    }
}
impl super::ElementVariant for AmericiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Am
    }
}
impl super::MassNumber for AmericiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Am230 => 230u16,
            Self::Am231 => 231u16,
            Self::Am232 => 232u16,
            Self::Am233 => 233u16,
            Self::Am234 => 234u16,
            Self::Am235 => 235u16,
            Self::Am236 => 236u16,
            Self::Am237 => 237u16,
            Self::Am238 => 238u16,
            Self::Am239 => 239u16,
            Self::Am240 => 240u16,
            Self::Am241 => 241u16,
            Self::Am242 => 242u16,
            Self::Am243 => 243u16,
            Self::Am244 => 244u16,
            Self::Am245 => 245u16,
            Self::Am246 => 246u16,
            Self::Am247 => 247u16,
            Self::Am248 => 248u16,
            Self::Am249 => 249u16,
        }
    }
}
impl super::IsotopicComposition for AmericiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Am230 => None,
            Self::Am231 => None,
            Self::Am232 => None,
            Self::Am233 => None,
            Self::Am234 => None,
            Self::Am235 => None,
            Self::Am236 => None,
            Self::Am237 => None,
            Self::Am238 => None,
            Self::Am239 => None,
            Self::Am240 => None,
            Self::Am241 => None,
            Self::Am242 => None,
            Self::Am243 => None,
            Self::Am244 => None,
            Self::Am245 => None,
            Self::Am246 => None,
            Self::Am247 => None,
            Self::Am248 => None,
            Self::Am249 => None,
        }
    }
}
impl super::MostCommonIsotope for AmericiumIsotope {
    fn most_common_isotope() -> Self {
        Self::Am249
    }
}
