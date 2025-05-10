//! Isotopes of the element Californium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Californium
pub enum CaliforniumIsotope {
    /// Isotope Cf237 of Californium
    Cf237,
    /// Isotope Cf238 of Californium
    Cf238,
    /// Isotope Cf239 of Californium
    Cf239,
    /// Isotope Cf240 of Californium
    Cf240,
    /// Isotope Cf241 of Californium
    Cf241,
    /// Isotope Cf242 of Californium
    Cf242,
    /// Isotope Cf243 of Californium
    Cf243,
    /// Isotope Cf244 of Californium
    Cf244,
    /// Isotope Cf245 of Californium
    Cf245,
    /// Isotope Cf246 of Californium
    Cf246,
    /// Isotope Cf247 of Californium
    Cf247,
    /// Isotope Cf248 of Californium
    Cf248,
    /// Isotope Cf249 of Californium
    Cf249,
    /// Isotope Cf250 of Californium
    Cf250,
    /// Isotope Cf251 of Californium
    Cf251,
    /// Isotope Cf252 of Californium
    Cf252,
    /// Isotope Cf253 of Californium
    Cf253,
    /// Isotope Cf254 of Californium
    Cf254,
    /// Isotope Cf255 of Californium
    Cf255,
    /// Isotope Cf256 of Californium
    Cf256,
}
impl super::RelativeAtomicMass for CaliforniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cf237 => 237.062198f64,
            Self::Cf238 => 238.06149f64,
            Self::Cf239 => 239.06253f64,
            Self::Cf240 => 240.062256f64,
            Self::Cf241 => 241.06369f64,
            Self::Cf242 => 242.063754f64,
            Self::Cf243 => 243.06548f64,
            Self::Cf244 => 244.0660008f64,
            Self::Cf245 => 245.0680487f64,
            Self::Cf246 => 246.0688055f64,
            Self::Cf247 => 247.070965f64,
            Self::Cf248 => 248.0721851f64,
            Self::Cf249 => 249.0748539f64,
            Self::Cf250 => 250.0764062f64,
            Self::Cf251 => 251.0795886f64,
            Self::Cf252 => 252.0816272f64,
            Self::Cf253 => 253.0851345f64,
            Self::Cf254 => 254.087324f64,
            Self::Cf255 => 255.09105f64,
            Self::Cf256 => 256.09344f64,
        }
    }
}
impl super::ElementVariant for CaliforniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cf
    }
}
impl super::MassNumber for CaliforniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cf237 => 237u16,
            Self::Cf238 => 238u16,
            Self::Cf239 => 239u16,
            Self::Cf240 => 240u16,
            Self::Cf241 => 241u16,
            Self::Cf242 => 242u16,
            Self::Cf243 => 243u16,
            Self::Cf244 => 244u16,
            Self::Cf245 => 245u16,
            Self::Cf246 => 246u16,
            Self::Cf247 => 247u16,
            Self::Cf248 => 248u16,
            Self::Cf249 => 249u16,
            Self::Cf250 => 250u16,
            Self::Cf251 => 251u16,
            Self::Cf252 => 252u16,
            Self::Cf253 => 253u16,
            Self::Cf254 => 254u16,
            Self::Cf255 => 255u16,
            Self::Cf256 => 256u16,
        }
    }
}
impl super::IsotopicComposition for CaliforniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for CaliforniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cf256
    }
}
impl TryFrom<u16> for CaliforniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            237u16 => Ok(Self::Cf237),
            238u16 => Ok(Self::Cf238),
            239u16 => Ok(Self::Cf239),
            240u16 => Ok(Self::Cf240),
            241u16 => Ok(Self::Cf241),
            242u16 => Ok(Self::Cf242),
            243u16 => Ok(Self::Cf243),
            244u16 => Ok(Self::Cf244),
            245u16 => Ok(Self::Cf245),
            246u16 => Ok(Self::Cf246),
            247u16 => Ok(Self::Cf247),
            248u16 => Ok(Self::Cf248),
            249u16 => Ok(Self::Cf249),
            250u16 => Ok(Self::Cf250),
            251u16 => Ok(Self::Cf251),
            252u16 => Ok(Self::Cf252),
            253u16 => Ok(Self::Cf253),
            254u16 => Ok(Self::Cf254),
            255u16 => Ok(Self::Cf255),
            256u16 => Ok(Self::Cf256),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cf, value)),
        }
    }
}
impl std::fmt::Display for CaliforniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cf237 => write!(f, "Cf237"),
            Self::Cf238 => write!(f, "Cf238"),
            Self::Cf239 => write!(f, "Cf239"),
            Self::Cf240 => write!(f, "Cf240"),
            Self::Cf241 => write!(f, "Cf241"),
            Self::Cf242 => write!(f, "Cf242"),
            Self::Cf243 => write!(f, "Cf243"),
            Self::Cf244 => write!(f, "Cf244"),
            Self::Cf245 => write!(f, "Cf245"),
            Self::Cf246 => write!(f, "Cf246"),
            Self::Cf247 => write!(f, "Cf247"),
            Self::Cf248 => write!(f, "Cf248"),
            Self::Cf249 => write!(f, "Cf249"),
            Self::Cf250 => write!(f, "Cf250"),
            Self::Cf251 => write!(f, "Cf251"),
            Self::Cf252 => write!(f, "Cf252"),
            Self::Cf253 => write!(f, "Cf253"),
            Self::Cf254 => write!(f, "Cf254"),
            Self::Cf255 => write!(f, "Cf255"),
            Self::Cf256 => write!(f, "Cf256"),
        }
    }
}
