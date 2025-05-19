//! Isotopes of the element Thorium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Thorium
pub enum ThoriumIsotope {
    /// Isotope Th208 of Thorium
    Th208,
    /// Isotope Th209 of Thorium
    Th209,
    /// Isotope Th210 of Thorium
    Th210,
    /// Isotope Th211 of Thorium
    Th211,
    /// Isotope Th212 of Thorium
    Th212,
    /// Isotope Th213 of Thorium
    Th213,
    /// Isotope Th214 of Thorium
    Th214,
    /// Isotope Th215 of Thorium
    Th215,
    /// Isotope Th216 of Thorium
    Th216,
    /// Isotope Th217 of Thorium
    Th217,
    /// Isotope Th218 of Thorium
    Th218,
    /// Isotope Th219 of Thorium
    Th219,
    /// Isotope Th220 of Thorium
    Th220,
    /// Isotope Th221 of Thorium
    Th221,
    /// Isotope Th222 of Thorium
    Th222,
    /// Isotope Th223 of Thorium
    Th223,
    /// Isotope Th224 of Thorium
    Th224,
    /// Isotope Th225 of Thorium
    Th225,
    /// Isotope Th226 of Thorium
    Th226,
    /// Isotope Th227 of Thorium
    Th227,
    /// Isotope Th228 of Thorium
    Th228,
    /// Isotope Th229 of Thorium
    Th229,
    /// Isotope Th230 of Thorium
    Th230,
    /// Isotope Th231 of Thorium
    Th231,
    /// Isotope Th232 of Thorium
    Th232,
    /// Isotope Th233 of Thorium
    Th233,
    /// Isotope Th234 of Thorium
    Th234,
    /// Isotope Th235 of Thorium
    Th235,
    /// Isotope Th236 of Thorium
    Th236,
    /// Isotope Th237 of Thorium
    Th237,
    /// Isotope Th238 of Thorium
    Th238,
    /// Isotope Th239 of Thorium
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
            Self::Th232 => Some(1f64),
            Self::Th208
            | Self::Th209
            | Self::Th210
            | Self::Th211
            | Self::Th212
            | Self::Th213
            | Self::Th214
            | Self::Th215
            | Self::Th216
            | Self::Th217
            | Self::Th218
            | Self::Th219
            | Self::Th220
            | Self::Th221
            | Self::Th222
            | Self::Th223
            | Self::Th224
            | Self::Th225
            | Self::Th226
            | Self::Th227
            | Self::Th228
            | Self::Th229
            | Self::Th230
            | Self::Th231
            | Self::Th233
            | Self::Th234
            | Self::Th235
            | Self::Th236
            | Self::Th237
            | Self::Th238
            | Self::Th239 => None,
        }
    }
}
impl super::MostAbundantIsotope for ThoriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Th232
    }
}
impl TryFrom<u16> for ThoriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            208u16 => Ok(Self::Th208),
            209u16 => Ok(Self::Th209),
            210u16 => Ok(Self::Th210),
            211u16 => Ok(Self::Th211),
            212u16 => Ok(Self::Th212),
            213u16 => Ok(Self::Th213),
            214u16 => Ok(Self::Th214),
            215u16 => Ok(Self::Th215),
            216u16 => Ok(Self::Th216),
            217u16 => Ok(Self::Th217),
            218u16 => Ok(Self::Th218),
            219u16 => Ok(Self::Th219),
            220u16 => Ok(Self::Th220),
            221u16 => Ok(Self::Th221),
            222u16 => Ok(Self::Th222),
            223u16 => Ok(Self::Th223),
            224u16 => Ok(Self::Th224),
            225u16 => Ok(Self::Th225),
            226u16 => Ok(Self::Th226),
            227u16 => Ok(Self::Th227),
            228u16 => Ok(Self::Th228),
            229u16 => Ok(Self::Th229),
            230u16 => Ok(Self::Th230),
            231u16 => Ok(Self::Th231),
            232u16 => Ok(Self::Th232),
            233u16 => Ok(Self::Th233),
            234u16 => Ok(Self::Th234),
            235u16 => Ok(Self::Th235),
            236u16 => Ok(Self::Th236),
            237u16 => Ok(Self::Th237),
            238u16 => Ok(Self::Th238),
            239u16 => Ok(Self::Th239),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Th, value)),
        }
    }
}
impl std::fmt::Display for ThoriumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Th208 => write!(f, "Th208"),
            Self::Th209 => write!(f, "Th209"),
            Self::Th210 => write!(f, "Th210"),
            Self::Th211 => write!(f, "Th211"),
            Self::Th212 => write!(f, "Th212"),
            Self::Th213 => write!(f, "Th213"),
            Self::Th214 => write!(f, "Th214"),
            Self::Th215 => write!(f, "Th215"),
            Self::Th216 => write!(f, "Th216"),
            Self::Th217 => write!(f, "Th217"),
            Self::Th218 => write!(f, "Th218"),
            Self::Th219 => write!(f, "Th219"),
            Self::Th220 => write!(f, "Th220"),
            Self::Th221 => write!(f, "Th221"),
            Self::Th222 => write!(f, "Th222"),
            Self::Th223 => write!(f, "Th223"),
            Self::Th224 => write!(f, "Th224"),
            Self::Th225 => write!(f, "Th225"),
            Self::Th226 => write!(f, "Th226"),
            Self::Th227 => write!(f, "Th227"),
            Self::Th228 => write!(f, "Th228"),
            Self::Th229 => write!(f, "Th229"),
            Self::Th230 => write!(f, "Th230"),
            Self::Th231 => write!(f, "Th231"),
            Self::Th232 => write!(f, "Th232"),
            Self::Th233 => write!(f, "Th233"),
            Self::Th234 => write!(f, "Th234"),
            Self::Th235 => write!(f, "Th235"),
            Self::Th236 => write!(f, "Th236"),
            Self::Th237 => write!(f, "Th237"),
            Self::Th238 => write!(f, "Th238"),
            Self::Th239 => write!(f, "Th239"),
        }
    }
}
