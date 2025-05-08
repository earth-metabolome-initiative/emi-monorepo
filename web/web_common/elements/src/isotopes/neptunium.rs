#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum NeptuniumIsotope {
    Np219,
    Np220,
    Np221,
    Np222,
    Np223,
    Np224,
    Np225,
    Np226,
    Np227,
    Np228,
    Np229,
    Np230,
    Np231,
    Np232,
    Np233,
    Np234,
    Np235,
    Np236,
    Np237,
    Np238,
    Np239,
    Np240,
    Np241,
    Np242,
    Np243,
    Np244,
    Np245,
}
impl super::RelativeAtomicMass for NeptuniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Np219 => 219.03143f64,
            Self::Np220 => 220.03254f64,
            Self::Np221 => 221.03204f64,
            Self::Np222 => 222.0333f64,
            Self::Np223 => 223.03285f64,
            Self::Np224 => 224.03422f64,
            Self::Np225 => 225.033911f64,
            Self::Np226 => 226.035188f64,
            Self::Np227 => 227.034957f64,
            Self::Np228 => 228.036067f64,
            Self::Np229 => 229.036264f64,
            Self::Np230 => 230.037828f64,
            Self::Np231 => 231.038245f64,
            Self::Np232 => 232.04011f64,
            Self::Np233 => 233.040741f64,
            Self::Np234 => 234.0428953f64,
            Self::Np235 => 235.0440635f64,
            Self::Np236 => 236.04657f64,
            Self::Np237 => 237.0481736f64,
            Self::Np238 => 238.0509466f64,
            Self::Np239 => 239.0529392f64,
            Self::Np240 => 240.056165f64,
            Self::Np241 => 241.058253f64,
            Self::Np242 => 242.06164f64,
            Self::Np243 => 243.06428f64,
            Self::Np244 => 244.06785f64,
            Self::Np245 => 245.0708f64,
        }
    }
}
impl super::ElementVariant for NeptuniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Np
    }
}
impl super::MassNumber for NeptuniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Np219 => 219u16,
            Self::Np220 => 220u16,
            Self::Np221 => 221u16,
            Self::Np222 => 222u16,
            Self::Np223 => 223u16,
            Self::Np224 => 224u16,
            Self::Np225 => 225u16,
            Self::Np226 => 226u16,
            Self::Np227 => 227u16,
            Self::Np228 => 228u16,
            Self::Np229 => 229u16,
            Self::Np230 => 230u16,
            Self::Np231 => 231u16,
            Self::Np232 => 232u16,
            Self::Np233 => 233u16,
            Self::Np234 => 234u16,
            Self::Np235 => 235u16,
            Self::Np236 => 236u16,
            Self::Np237 => 237u16,
            Self::Np238 => 238u16,
            Self::Np239 => 239u16,
            Self::Np240 => 240u16,
            Self::Np241 => 241u16,
            Self::Np242 => 242u16,
            Self::Np243 => 243u16,
            Self::Np244 => 244u16,
            Self::Np245 => 245u16,
        }
    }
}
impl super::IsotopicComposition for NeptuniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Np219 => None,
            Self::Np220 => None,
            Self::Np221 => None,
            Self::Np222 => None,
            Self::Np223 => None,
            Self::Np224 => None,
            Self::Np225 => None,
            Self::Np226 => None,
            Self::Np227 => None,
            Self::Np228 => None,
            Self::Np229 => None,
            Self::Np230 => None,
            Self::Np231 => None,
            Self::Np232 => None,
            Self::Np233 => None,
            Self::Np234 => None,
            Self::Np235 => None,
            Self::Np236 => None,
            Self::Np237 => None,
            Self::Np238 => None,
            Self::Np239 => None,
            Self::Np240 => None,
            Self::Np241 => None,
            Self::Np242 => None,
            Self::Np243 => None,
            Self::Np244 => None,
            Self::Np245 => None,
        }
    }
}
impl super::MostCommonIsotope for NeptuniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Np245
    }
}
