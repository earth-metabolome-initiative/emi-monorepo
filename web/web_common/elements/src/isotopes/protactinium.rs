//! Isotopes of the element Protactinium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Protactinium
pub enum ProtactiniumIsotope {
    /// Isotope Pa212 of Protactinium
    Pa212,
    /// Isotope Pa213 of Protactinium
    Pa213,
    /// Isotope Pa214 of Protactinium
    Pa214,
    /// Isotope Pa215 of Protactinium
    Pa215,
    /// Isotope Pa216 of Protactinium
    Pa216,
    /// Isotope Pa217 of Protactinium
    Pa217,
    /// Isotope Pa218 of Protactinium
    Pa218,
    /// Isotope Pa219 of Protactinium
    Pa219,
    /// Isotope Pa220 of Protactinium
    Pa220,
    /// Isotope Pa221 of Protactinium
    Pa221,
    /// Isotope Pa222 of Protactinium
    Pa222,
    /// Isotope Pa223 of Protactinium
    Pa223,
    /// Isotope Pa224 of Protactinium
    Pa224,
    /// Isotope Pa225 of Protactinium
    Pa225,
    /// Isotope Pa226 of Protactinium
    Pa226,
    /// Isotope Pa227 of Protactinium
    Pa227,
    /// Isotope Pa228 of Protactinium
    Pa228,
    /// Isotope Pa229 of Protactinium
    Pa229,
    /// Isotope Pa230 of Protactinium
    Pa230,
    /// Isotope Pa231 of Protactinium
    Pa231,
    /// Isotope Pa232 of Protactinium
    Pa232,
    /// Isotope Pa233 of Protactinium
    Pa233,
    /// Isotope Pa234 of Protactinium
    Pa234,
    /// Isotope Pa235 of Protactinium
    Pa235,
    /// Isotope Pa236 of Protactinium
    Pa236,
    /// Isotope Pa237 of Protactinium
    Pa237,
    /// Isotope Pa238 of Protactinium
    Pa238,
    /// Isotope Pa239 of Protactinium
    Pa239,
    /// Isotope Pa240 of Protactinium
    Pa240,
    /// Isotope Pa241 of Protactinium
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
            Self::Pa231 => Some(1f64),
            Self::Pa212
            | Self::Pa213
            | Self::Pa214
            | Self::Pa215
            | Self::Pa216
            | Self::Pa217
            | Self::Pa218
            | Self::Pa219
            | Self::Pa220
            | Self::Pa221
            | Self::Pa222
            | Self::Pa223
            | Self::Pa224
            | Self::Pa225
            | Self::Pa226
            | Self::Pa227
            | Self::Pa228
            | Self::Pa229
            | Self::Pa230
            | Self::Pa232
            | Self::Pa233
            | Self::Pa234
            | Self::Pa235
            | Self::Pa236
            | Self::Pa237
            | Self::Pa238
            | Self::Pa239
            | Self::Pa240
            | Self::Pa241 => None,
        }
    }
}
impl super::MostAbundantIsotope for ProtactiniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Pa231
    }
}
impl TryFrom<u16> for ProtactiniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            212u16 => Ok(Self::Pa212),
            213u16 => Ok(Self::Pa213),
            214u16 => Ok(Self::Pa214),
            215u16 => Ok(Self::Pa215),
            216u16 => Ok(Self::Pa216),
            217u16 => Ok(Self::Pa217),
            218u16 => Ok(Self::Pa218),
            219u16 => Ok(Self::Pa219),
            220u16 => Ok(Self::Pa220),
            221u16 => Ok(Self::Pa221),
            222u16 => Ok(Self::Pa222),
            223u16 => Ok(Self::Pa223),
            224u16 => Ok(Self::Pa224),
            225u16 => Ok(Self::Pa225),
            226u16 => Ok(Self::Pa226),
            227u16 => Ok(Self::Pa227),
            228u16 => Ok(Self::Pa228),
            229u16 => Ok(Self::Pa229),
            230u16 => Ok(Self::Pa230),
            231u16 => Ok(Self::Pa231),
            232u16 => Ok(Self::Pa232),
            233u16 => Ok(Self::Pa233),
            234u16 => Ok(Self::Pa234),
            235u16 => Ok(Self::Pa235),
            236u16 => Ok(Self::Pa236),
            237u16 => Ok(Self::Pa237),
            238u16 => Ok(Self::Pa238),
            239u16 => Ok(Self::Pa239),
            240u16 => Ok(Self::Pa240),
            241u16 => Ok(Self::Pa241),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Pa, value)),
        }
    }
}
impl std::fmt::Display for ProtactiniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pa212 => write!(f, "Pa212"),
            Self::Pa213 => write!(f, "Pa213"),
            Self::Pa214 => write!(f, "Pa214"),
            Self::Pa215 => write!(f, "Pa215"),
            Self::Pa216 => write!(f, "Pa216"),
            Self::Pa217 => write!(f, "Pa217"),
            Self::Pa218 => write!(f, "Pa218"),
            Self::Pa219 => write!(f, "Pa219"),
            Self::Pa220 => write!(f, "Pa220"),
            Self::Pa221 => write!(f, "Pa221"),
            Self::Pa222 => write!(f, "Pa222"),
            Self::Pa223 => write!(f, "Pa223"),
            Self::Pa224 => write!(f, "Pa224"),
            Self::Pa225 => write!(f, "Pa225"),
            Self::Pa226 => write!(f, "Pa226"),
            Self::Pa227 => write!(f, "Pa227"),
            Self::Pa228 => write!(f, "Pa228"),
            Self::Pa229 => write!(f, "Pa229"),
            Self::Pa230 => write!(f, "Pa230"),
            Self::Pa231 => write!(f, "Pa231"),
            Self::Pa232 => write!(f, "Pa232"),
            Self::Pa233 => write!(f, "Pa233"),
            Self::Pa234 => write!(f, "Pa234"),
            Self::Pa235 => write!(f, "Pa235"),
            Self::Pa236 => write!(f, "Pa236"),
            Self::Pa237 => write!(f, "Pa237"),
            Self::Pa238 => write!(f, "Pa238"),
            Self::Pa239 => write!(f, "Pa239"),
            Self::Pa240 => write!(f, "Pa240"),
            Self::Pa241 => write!(f, "Pa241"),
        }
    }
}
