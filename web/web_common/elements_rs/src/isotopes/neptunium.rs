//! Isotopes of the element Neptunium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Neptunium
pub enum NeptuniumIsotope {
    /// Isotope Np219 of Neptunium
    Np219,
    /// Isotope Np220 of Neptunium
    Np220,
    /// Isotope Np221 of Neptunium
    Np221,
    /// Isotope Np222 of Neptunium
    Np222,
    /// Isotope Np223 of Neptunium
    Np223,
    /// Isotope Np224 of Neptunium
    Np224,
    /// Isotope Np225 of Neptunium
    Np225,
    /// Isotope Np226 of Neptunium
    Np226,
    /// Isotope Np227 of Neptunium
    Np227,
    /// Isotope Np228 of Neptunium
    Np228,
    /// Isotope Np229 of Neptunium
    Np229,
    /// Isotope Np230 of Neptunium
    Np230,
    /// Isotope Np231 of Neptunium
    Np231,
    /// Isotope Np232 of Neptunium
    Np232,
    /// Isotope Np233 of Neptunium
    Np233,
    /// Isotope Np234 of Neptunium
    Np234,
    /// Isotope Np235 of Neptunium
    Np235,
    /// Isotope Np236 of Neptunium
    Np236,
    /// Isotope Np237 of Neptunium
    Np237,
    /// Isotope Np238 of Neptunium
    Np238,
    /// Isotope Np239 of Neptunium
    Np239,
    /// Isotope Np240 of Neptunium
    Np240,
    /// Isotope Np241 of Neptunium
    Np241,
    /// Isotope Np242 of Neptunium
    Np242,
    /// Isotope Np243 of Neptunium
    Np243,
    /// Isotope Np244 of Neptunium
    Np244,
    /// Isotope Np245 of Neptunium
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
        None
    }
}
impl super::MostAbundantIsotope for NeptuniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Np245
    }
}
impl From<NeptuniumIsotope> for crate::Isotope {
    fn from(isotope: NeptuniumIsotope) -> Self {
        crate::Isotope::Np(isotope)
    }
}
impl From<NeptuniumIsotope> for crate::Element {
    fn from(_isotope: NeptuniumIsotope) -> Self {
        crate::Element::Np
    }
}
impl TryFrom<u16> for NeptuniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            219u16 => Ok(Self::Np219),
            220u16 => Ok(Self::Np220),
            221u16 => Ok(Self::Np221),
            222u16 => Ok(Self::Np222),
            223u16 => Ok(Self::Np223),
            224u16 => Ok(Self::Np224),
            225u16 => Ok(Self::Np225),
            226u16 => Ok(Self::Np226),
            227u16 => Ok(Self::Np227),
            228u16 => Ok(Self::Np228),
            229u16 => Ok(Self::Np229),
            230u16 => Ok(Self::Np230),
            231u16 => Ok(Self::Np231),
            232u16 => Ok(Self::Np232),
            233u16 => Ok(Self::Np233),
            234u16 => Ok(Self::Np234),
            235u16 => Ok(Self::Np235),
            236u16 => Ok(Self::Np236),
            237u16 => Ok(Self::Np237),
            238u16 => Ok(Self::Np238),
            239u16 => Ok(Self::Np239),
            240u16 => Ok(Self::Np240),
            241u16 => Ok(Self::Np241),
            242u16 => Ok(Self::Np242),
            243u16 => Ok(Self::Np243),
            244u16 => Ok(Self::Np244),
            245u16 => Ok(Self::Np245),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Np, value)),
        }
    }
}
impl std::fmt::Display for NeptuniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Np219 => write!(f, "Np219"),
            Self::Np220 => write!(f, "Np220"),
            Self::Np221 => write!(f, "Np221"),
            Self::Np222 => write!(f, "Np222"),
            Self::Np223 => write!(f, "Np223"),
            Self::Np224 => write!(f, "Np224"),
            Self::Np225 => write!(f, "Np225"),
            Self::Np226 => write!(f, "Np226"),
            Self::Np227 => write!(f, "Np227"),
            Self::Np228 => write!(f, "Np228"),
            Self::Np229 => write!(f, "Np229"),
            Self::Np230 => write!(f, "Np230"),
            Self::Np231 => write!(f, "Np231"),
            Self::Np232 => write!(f, "Np232"),
            Self::Np233 => write!(f, "Np233"),
            Self::Np234 => write!(f, "Np234"),
            Self::Np235 => write!(f, "Np235"),
            Self::Np236 => write!(f, "Np236"),
            Self::Np237 => write!(f, "Np237"),
            Self::Np238 => write!(f, "Np238"),
            Self::Np239 => write!(f, "Np239"),
            Self::Np240 => write!(f, "Np240"),
            Self::Np241 => write!(f, "Np241"),
            Self::Np242 => write!(f, "Np242"),
            Self::Np243 => write!(f, "Np243"),
            Self::Np244 => write!(f, "Np244"),
            Self::Np245 => write!(f, "Np245"),
        }
    }
}
