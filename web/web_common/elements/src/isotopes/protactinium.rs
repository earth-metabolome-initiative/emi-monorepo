#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ProtactiniumIsotope {
    Pa212,
    Pa213,
    Pa214,
    Pa215,
    Pa216,
    Pa217,
    Pa218,
    Pa219,
    Pa220,
    Pa221,
    Pa222,
    Pa223,
    Pa224,
    Pa225,
    Pa226,
    Pa227,
    Pa228,
    Pa229,
    Pa230,
    Pa231,
    Pa232,
    Pa233,
    Pa234,
    Pa235,
    Pa236,
    Pa237,
    Pa238,
    Pa239,
    Pa240,
    Pa241,
}
impl super::RelativeAtomicMass for ProtactiniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Pa212 => 212.023203f64,
            Self::Pa213 => 213.021109f64,
            Self::Pa214 => 214.020918f64,
            Self::Pa215 => 215.019183f64,
            Self::Pa216 => 216.019109f64,
            Self::Pa217 => 217.018325f64,
            Self::Pa218 => 218.020059f64,
            Self::Pa219 => 219.019904f64,
            Self::Pa220 => 220.021705f64,
            Self::Pa221 => 221.021875f64,
            Self::Pa222 => 222.023784f64,
            Self::Pa223 => 223.023963f64,
            Self::Pa224 => 224.0256176f64,
            Self::Pa225 => 225.026131f64,
            Self::Pa226 => 226.027948f64,
            Self::Pa227 => 227.0288054f64,
            Self::Pa228 => 228.0310517f64,
            Self::Pa229 => 229.0320972f64,
            Self::Pa230 => 230.034541f64,
            Self::Pa231 => 231.0358842f64,
            Self::Pa232 => 232.0385917f64,
            Self::Pa233 => 233.0402472f64,
            Self::Pa234 => 234.0433072f64,
            Self::Pa235 => 235.045399f64,
            Self::Pa236 => 236.048668f64,
            Self::Pa237 => 237.051023f64,
            Self::Pa238 => 238.054637f64,
            Self::Pa239 => 239.05726f64,
            Self::Pa240 => 240.06098f64,
            Self::Pa241 => 241.06408f64,
        }
    }
}
impl super::ElementVariant for ProtactiniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Pa
    }
}
impl super::MassNumber for ProtactiniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Pa212 => 212u16,
            Self::Pa213 => 213u16,
            Self::Pa214 => 214u16,
            Self::Pa215 => 215u16,
            Self::Pa216 => 216u16,
            Self::Pa217 => 217u16,
            Self::Pa218 => 218u16,
            Self::Pa219 => 219u16,
            Self::Pa220 => 220u16,
            Self::Pa221 => 221u16,
            Self::Pa222 => 222u16,
            Self::Pa223 => 223u16,
            Self::Pa224 => 224u16,
            Self::Pa225 => 225u16,
            Self::Pa226 => 226u16,
            Self::Pa227 => 227u16,
            Self::Pa228 => 228u16,
            Self::Pa229 => 229u16,
            Self::Pa230 => 230u16,
            Self::Pa231 => 231u16,
            Self::Pa232 => 232u16,
            Self::Pa233 => 233u16,
            Self::Pa234 => 234u16,
            Self::Pa235 => 235u16,
            Self::Pa236 => 236u16,
            Self::Pa237 => 237u16,
            Self::Pa238 => 238u16,
            Self::Pa239 => 239u16,
            Self::Pa240 => 240u16,
            Self::Pa241 => 241u16,
        }
    }
}
impl super::IsotopicComposition for ProtactiniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Pa212 => None,
            Self::Pa213 => None,
            Self::Pa214 => None,
            Self::Pa215 => None,
            Self::Pa216 => None,
            Self::Pa217 => None,
            Self::Pa218 => None,
            Self::Pa219 => None,
            Self::Pa220 => None,
            Self::Pa221 => None,
            Self::Pa222 => None,
            Self::Pa223 => None,
            Self::Pa224 => None,
            Self::Pa225 => None,
            Self::Pa226 => None,
            Self::Pa227 => None,
            Self::Pa228 => None,
            Self::Pa229 => None,
            Self::Pa230 => None,
            Self::Pa231 => Some(1f64),
            Self::Pa232 => None,
            Self::Pa233 => None,
            Self::Pa234 => None,
            Self::Pa235 => None,
            Self::Pa236 => None,
            Self::Pa237 => None,
            Self::Pa238 => None,
            Self::Pa239 => None,
            Self::Pa240 => None,
            Self::Pa241 => None,
        }
    }
}
impl super::MostCommonIsotope for ProtactiniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Pa231
    }
}
