#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ThoriumIsotope {
    Th208,
    Th209,
    Th210,
    Th211,
    Th212,
    Th213,
    Th214,
    Th215,
    Th216,
    Th217,
    Th218,
    Th219,
    Th220,
    Th221,
    Th222,
    Th223,
    Th224,
    Th225,
    Th226,
    Th227,
    Th228,
    Th229,
    Th230,
    Th231,
    Th232,
    Th233,
    Th234,
    Th235,
    Th236,
    Th237,
    Th238,
    Th239,
}
impl super::RelativeAtomicMass for ThoriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Th208 => 208.0179f64,
            Self::Th209 => 209.017753f64,
            Self::Th210 => 210.015094f64,
            Self::Th211 => 211.014929f64,
            Self::Th212 => 212.012988f64,
            Self::Th213 => 213.013009f64,
            Self::Th214 => 214.0115f64,
            Self::Th215 => 215.0117248f64,
            Self::Th216 => 216.011056f64,
            Self::Th217 => 217.013117f64,
            Self::Th218 => 218.013276f64,
            Self::Th219 => 219.015537f64,
            Self::Th220 => 220.015748f64,
            Self::Th221 => 221.018184f64,
            Self::Th222 => 222.018469f64,
            Self::Th223 => 223.0208119f64,
            Self::Th224 => 224.021464f64,
            Self::Th225 => 225.0239514f64,
            Self::Th226 => 226.0249034f64,
            Self::Th227 => 227.0277042f64,
            Self::Th228 => 228.0287413f64,
            Self::Th229 => 229.0317627f64,
            Self::Th230 => 230.0331341f64,
            Self::Th231 => 231.0363046f64,
            Self::Th232 => 232.0380558f64,
            Self::Th233 => 233.0415823f64,
            Self::Th234 => 234.0436014f64,
            Self::Th235 => 235.047255f64,
            Self::Th236 => 236.049657f64,
            Self::Th237 => 237.053629f64,
            Self::Th238 => 238.0565f64,
            Self::Th239 => 239.06077f64,
        }
    }
}
impl super::ElementVariant for ThoriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Th
    }
}
impl super::MassNumber for ThoriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Th208 => 208u16,
            Self::Th209 => 209u16,
            Self::Th210 => 210u16,
            Self::Th211 => 211u16,
            Self::Th212 => 212u16,
            Self::Th213 => 213u16,
            Self::Th214 => 214u16,
            Self::Th215 => 215u16,
            Self::Th216 => 216u16,
            Self::Th217 => 217u16,
            Self::Th218 => 218u16,
            Self::Th219 => 219u16,
            Self::Th220 => 220u16,
            Self::Th221 => 221u16,
            Self::Th222 => 222u16,
            Self::Th223 => 223u16,
            Self::Th224 => 224u16,
            Self::Th225 => 225u16,
            Self::Th226 => 226u16,
            Self::Th227 => 227u16,
            Self::Th228 => 228u16,
            Self::Th229 => 229u16,
            Self::Th230 => 230u16,
            Self::Th231 => 231u16,
            Self::Th232 => 232u16,
            Self::Th233 => 233u16,
            Self::Th234 => 234u16,
            Self::Th235 => 235u16,
            Self::Th236 => 236u16,
            Self::Th237 => 237u16,
            Self::Th238 => 238u16,
            Self::Th239 => 239u16,
        }
    }
}
impl super::IsotopicComposition for ThoriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Th208 => None,
            Self::Th209 => None,
            Self::Th210 => None,
            Self::Th211 => None,
            Self::Th212 => None,
            Self::Th213 => None,
            Self::Th214 => None,
            Self::Th215 => None,
            Self::Th216 => None,
            Self::Th217 => None,
            Self::Th218 => None,
            Self::Th219 => None,
            Self::Th220 => None,
            Self::Th221 => None,
            Self::Th222 => None,
            Self::Th223 => None,
            Self::Th224 => None,
            Self::Th225 => None,
            Self::Th226 => None,
            Self::Th227 => None,
            Self::Th228 => None,
            Self::Th229 => None,
            Self::Th230 => None,
            Self::Th231 => None,
            Self::Th232 => Some(1f64),
            Self::Th233 => None,
            Self::Th234 => None,
            Self::Th235 => None,
            Self::Th236 => None,
            Self::Th237 => None,
            Self::Th238 => None,
            Self::Th239 => None,
        }
    }
}
impl super::MostCommonIsotope for ThoriumIsotope {
    fn most_common_isotope() -> Self {
        Self::Th232
    }
}
