//! Isotopes of the element Radium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Radium
pub enum RadiumIsotope {
    /// Isotope Ra201 of Radium
    Ra201,
    /// Isotope Ra202 of Radium
    Ra202,
    /// Isotope Ra203 of Radium
    Ra203,
    /// Isotope Ra204 of Radium
    Ra204,
    /// Isotope Ra205 of Radium
    Ra205,
    /// Isotope Ra206 of Radium
    Ra206,
    /// Isotope Ra207 of Radium
    Ra207,
    /// Isotope Ra208 of Radium
    Ra208,
    /// Isotope Ra209 of Radium
    Ra209,
    /// Isotope Ra210 of Radium
    Ra210,
    /// Isotope Ra211 of Radium
    Ra211,
    /// Isotope Ra212 of Radium
    Ra212,
    /// Isotope Ra213 of Radium
    Ra213,
    /// Isotope Ra214 of Radium
    Ra214,
    /// Isotope Ra215 of Radium
    Ra215,
    /// Isotope Ra216 of Radium
    Ra216,
    /// Isotope Ra217 of Radium
    Ra217,
    /// Isotope Ra218 of Radium
    Ra218,
    /// Isotope Ra219 of Radium
    Ra219,
    /// Isotope Ra220 of Radium
    Ra220,
    /// Isotope Ra221 of Radium
    Ra221,
    /// Isotope Ra222 of Radium
    Ra222,
    /// Isotope Ra223 of Radium
    Ra223,
    /// Isotope Ra224 of Radium
    Ra224,
    /// Isotope Ra225 of Radium
    Ra225,
    /// Isotope Ra226 of Radium
    Ra226,
    /// Isotope Ra227 of Radium
    Ra227,
    /// Isotope Ra228 of Radium
    Ra228,
    /// Isotope Ra229 of Radium
    Ra229,
    /// Isotope Ra230 of Radium
    Ra230,
    /// Isotope Ra231 of Radium
    Ra231,
    /// Isotope Ra232 of Radium
    Ra232,
    /// Isotope Ra233 of Radium
    Ra233,
    /// Isotope Ra234 of Radium
    Ra234,
    /// Isotope Ra235 of Radium
    Ra235,
}
impl super::RelativeAtomicMass for RadiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ra201 => 201.01271f64,
            Self::Ra202 => 202.00976f64,
            Self::Ra203 => 203.009304f64,
            Self::Ra204 => 204.006492f64,
            Self::Ra205 => 205.006268f64,
            Self::Ra206 => 206.003828f64,
            Self::Ra207 => 207.003799f64,
            Self::Ra208 => 208.001841f64,
            Self::Ra209 => 209.00199f64,
            Self::Ra210 => 210.000494f64,
            Self::Ra211 => 211.0008932f64,
            Self::Ra212 => 211.999787f64,
            Self::Ra213 => 213.000384f64,
            Self::Ra214 => 214.0000997f64,
            Self::Ra215 => 215.0027204f64,
            Self::Ra216 => 216.0035334f64,
            Self::Ra217 => 217.0063207f64,
            Self::Ra218 => 218.007141f64,
            Self::Ra219 => 219.0100855f64,
            Self::Ra220 => 220.0110259f64,
            Self::Ra221 => 221.0139177f64,
            Self::Ra222 => 222.0153748f64,
            Self::Ra223 => 223.0185023f64,
            Self::Ra224 => 224.020212f64,
            Self::Ra225 => 225.0236119f64,
            Self::Ra226 => 226.0254103f64,
            Self::Ra227 => 227.0291783f64,
            Self::Ra228 => 228.0310707f64,
            Self::Ra229 => 229.034942f64,
            Self::Ra230 => 230.037055f64,
            Self::Ra231 => 231.041027f64,
            Self::Ra232 => 232.0434753f64,
            Self::Ra233 => 233.047582f64,
            Self::Ra234 => 234.050342f64,
            Self::Ra235 => 235.05497f64,
        }
    }
}
impl super::ElementVariant for RadiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ra
    }
}
impl super::MassNumber for RadiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ra201 => 201u16,
            Self::Ra202 => 202u16,
            Self::Ra203 => 203u16,
            Self::Ra204 => 204u16,
            Self::Ra205 => 205u16,
            Self::Ra206 => 206u16,
            Self::Ra207 => 207u16,
            Self::Ra208 => 208u16,
            Self::Ra209 => 209u16,
            Self::Ra210 => 210u16,
            Self::Ra211 => 211u16,
            Self::Ra212 => 212u16,
            Self::Ra213 => 213u16,
            Self::Ra214 => 214u16,
            Self::Ra215 => 215u16,
            Self::Ra216 => 216u16,
            Self::Ra217 => 217u16,
            Self::Ra218 => 218u16,
            Self::Ra219 => 219u16,
            Self::Ra220 => 220u16,
            Self::Ra221 => 221u16,
            Self::Ra222 => 222u16,
            Self::Ra223 => 223u16,
            Self::Ra224 => 224u16,
            Self::Ra225 => 225u16,
            Self::Ra226 => 226u16,
            Self::Ra227 => 227u16,
            Self::Ra228 => 228u16,
            Self::Ra229 => 229u16,
            Self::Ra230 => 230u16,
            Self::Ra231 => 231u16,
            Self::Ra232 => 232u16,
            Self::Ra233 => 233u16,
            Self::Ra234 => 234u16,
            Self::Ra235 => 235u16,
        }
    }
}
impl super::IsotopicComposition for RadiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for RadiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ra235
    }
}
impl From<RadiumIsotope> for crate::Isotope {
    fn from(isotope: RadiumIsotope) -> Self {
        crate::Isotope::Ra(isotope)
    }
}
impl From<RadiumIsotope> for crate::Element {
    fn from(_isotope: RadiumIsotope) -> Self {
        crate::Element::Ra
    }
}
impl TryFrom<u16> for RadiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            201u16 => Ok(Self::Ra201),
            202u16 => Ok(Self::Ra202),
            203u16 => Ok(Self::Ra203),
            204u16 => Ok(Self::Ra204),
            205u16 => Ok(Self::Ra205),
            206u16 => Ok(Self::Ra206),
            207u16 => Ok(Self::Ra207),
            208u16 => Ok(Self::Ra208),
            209u16 => Ok(Self::Ra209),
            210u16 => Ok(Self::Ra210),
            211u16 => Ok(Self::Ra211),
            212u16 => Ok(Self::Ra212),
            213u16 => Ok(Self::Ra213),
            214u16 => Ok(Self::Ra214),
            215u16 => Ok(Self::Ra215),
            216u16 => Ok(Self::Ra216),
            217u16 => Ok(Self::Ra217),
            218u16 => Ok(Self::Ra218),
            219u16 => Ok(Self::Ra219),
            220u16 => Ok(Self::Ra220),
            221u16 => Ok(Self::Ra221),
            222u16 => Ok(Self::Ra222),
            223u16 => Ok(Self::Ra223),
            224u16 => Ok(Self::Ra224),
            225u16 => Ok(Self::Ra225),
            226u16 => Ok(Self::Ra226),
            227u16 => Ok(Self::Ra227),
            228u16 => Ok(Self::Ra228),
            229u16 => Ok(Self::Ra229),
            230u16 => Ok(Self::Ra230),
            231u16 => Ok(Self::Ra231),
            232u16 => Ok(Self::Ra232),
            233u16 => Ok(Self::Ra233),
            234u16 => Ok(Self::Ra234),
            235u16 => Ok(Self::Ra235),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ra, value)),
        }
    }
}
impl std::fmt::Display for RadiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ra201 => write!(f, "Ra201"),
            Self::Ra202 => write!(f, "Ra202"),
            Self::Ra203 => write!(f, "Ra203"),
            Self::Ra204 => write!(f, "Ra204"),
            Self::Ra205 => write!(f, "Ra205"),
            Self::Ra206 => write!(f, "Ra206"),
            Self::Ra207 => write!(f, "Ra207"),
            Self::Ra208 => write!(f, "Ra208"),
            Self::Ra209 => write!(f, "Ra209"),
            Self::Ra210 => write!(f, "Ra210"),
            Self::Ra211 => write!(f, "Ra211"),
            Self::Ra212 => write!(f, "Ra212"),
            Self::Ra213 => write!(f, "Ra213"),
            Self::Ra214 => write!(f, "Ra214"),
            Self::Ra215 => write!(f, "Ra215"),
            Self::Ra216 => write!(f, "Ra216"),
            Self::Ra217 => write!(f, "Ra217"),
            Self::Ra218 => write!(f, "Ra218"),
            Self::Ra219 => write!(f, "Ra219"),
            Self::Ra220 => write!(f, "Ra220"),
            Self::Ra221 => write!(f, "Ra221"),
            Self::Ra222 => write!(f, "Ra222"),
            Self::Ra223 => write!(f, "Ra223"),
            Self::Ra224 => write!(f, "Ra224"),
            Self::Ra225 => write!(f, "Ra225"),
            Self::Ra226 => write!(f, "Ra226"),
            Self::Ra227 => write!(f, "Ra227"),
            Self::Ra228 => write!(f, "Ra228"),
            Self::Ra229 => write!(f, "Ra229"),
            Self::Ra230 => write!(f, "Ra230"),
            Self::Ra231 => write!(f, "Ra231"),
            Self::Ra232 => write!(f, "Ra232"),
            Self::Ra233 => write!(f, "Ra233"),
            Self::Ra234 => write!(f, "Ra234"),
            Self::Ra235 => write!(f, "Ra235"),
        }
    }
}
