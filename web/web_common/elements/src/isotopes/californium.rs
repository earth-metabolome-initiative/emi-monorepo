#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum CaliforniumIsotope {
    Cf237,
    Cf238,
    Cf239,
    Cf240,
    Cf241,
    Cf242,
    Cf243,
    Cf244,
    Cf245,
    Cf246,
    Cf247,
    Cf248,
    Cf249,
    Cf250,
    Cf251,
    Cf252,
    Cf253,
    Cf254,
    Cf255,
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
        match self {
            Self::Cf237 => None,
            Self::Cf238 => None,
            Self::Cf239 => None,
            Self::Cf240 => None,
            Self::Cf241 => None,
            Self::Cf242 => None,
            Self::Cf243 => None,
            Self::Cf244 => None,
            Self::Cf245 => None,
            Self::Cf246 => None,
            Self::Cf247 => None,
            Self::Cf248 => None,
            Self::Cf249 => None,
            Self::Cf250 => None,
            Self::Cf251 => None,
            Self::Cf252 => None,
            Self::Cf253 => None,
            Self::Cf254 => None,
            Self::Cf255 => None,
            Self::Cf256 => None,
        }
    }
}
impl super::MostCommonIsotope for CaliforniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Cf256
    }
}
