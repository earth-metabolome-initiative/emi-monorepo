//! Isotopes of the element Bohrium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Bohrium
pub enum BohriumIsotope {
    /// Isotope Bh260 of Bohrium
    Bh260,
    /// Isotope Bh261 of Bohrium
    Bh261,
    /// Isotope Bh262 of Bohrium
    Bh262,
    /// Isotope Bh263 of Bohrium
    Bh263,
    /// Isotope Bh264 of Bohrium
    Bh264,
    /// Isotope Bh265 of Bohrium
    Bh265,
    /// Isotope Bh266 of Bohrium
    Bh266,
    /// Isotope Bh267 of Bohrium
    Bh267,
    /// Isotope Bh268 of Bohrium
    Bh268,
    /// Isotope Bh269 of Bohrium
    Bh269,
    /// Isotope Bh270 of Bohrium
    Bh270,
    /// Isotope Bh271 of Bohrium
    Bh271,
    /// Isotope Bh272 of Bohrium
    Bh272,
    /// Isotope Bh273 of Bohrium
    Bh273,
    /// Isotope Bh274 of Bohrium
    Bh274,
    /// Isotope Bh275 of Bohrium
    Bh275,
}
impl super::RelativeAtomicMass for BohriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Bh260 => 260.12166f64,
            Self::Bh261 => 261.12145f64,
            Self::Bh262 => 262.12297f64,
            Self::Bh263 => 263.12292f64,
            Self::Bh264 => 264.12459f64,
            Self::Bh265 => 265.12491f64,
            Self::Bh266 => 266.12679f64,
            Self::Bh267 => 267.1275f64,
            Self::Bh268 => 268.12969f64,
            Self::Bh269 => 269.13042f64,
            Self::Bh270 => 270.13336f64,
            Self::Bh271 => 271.13526f64,
            Self::Bh272 => 272.13826f64,
            Self::Bh273 => 273.14024f64,
            Self::Bh274 => 274.14355f64,
            Self::Bh275 => 275.14567f64,
        }
    }
}
impl super::ElementVariant for BohriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Bh
    }
}
impl super::MassNumber for BohriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Bh260 => 260u16,
            Self::Bh261 => 261u16,
            Self::Bh262 => 262u16,
            Self::Bh263 => 263u16,
            Self::Bh264 => 264u16,
            Self::Bh265 => 265u16,
            Self::Bh266 => 266u16,
            Self::Bh267 => 267u16,
            Self::Bh268 => 268u16,
            Self::Bh269 => 269u16,
            Self::Bh270 => 270u16,
            Self::Bh271 => 271u16,
            Self::Bh272 => 272u16,
            Self::Bh273 => 273u16,
            Self::Bh274 => 274u16,
            Self::Bh275 => 275u16,
        }
    }
}
impl super::IsotopicComposition for BohriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for BohriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Bh275
    }
}
impl From<BohriumIsotope> for crate::Isotope {
    fn from(isotope: BohriumIsotope) -> Self {
        crate::Isotope::Bh(isotope)
    }
}
impl From<BohriumIsotope> for crate::Element {
    fn from(_isotope: BohriumIsotope) -> Self {
        crate::Element::Bh
    }
}
impl TryFrom<u16> for BohriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            260u16 => Ok(Self::Bh260),
            261u16 => Ok(Self::Bh261),
            262u16 => Ok(Self::Bh262),
            263u16 => Ok(Self::Bh263),
            264u16 => Ok(Self::Bh264),
            265u16 => Ok(Self::Bh265),
            266u16 => Ok(Self::Bh266),
            267u16 => Ok(Self::Bh267),
            268u16 => Ok(Self::Bh268),
            269u16 => Ok(Self::Bh269),
            270u16 => Ok(Self::Bh270),
            271u16 => Ok(Self::Bh271),
            272u16 => Ok(Self::Bh272),
            273u16 => Ok(Self::Bh273),
            274u16 => Ok(Self::Bh274),
            275u16 => Ok(Self::Bh275),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Bh, value)),
        }
    }
}
impl std::fmt::Display for BohriumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bh260 => write!(f, "Bh260"),
            Self::Bh261 => write!(f, "Bh261"),
            Self::Bh262 => write!(f, "Bh262"),
            Self::Bh263 => write!(f, "Bh263"),
            Self::Bh264 => write!(f, "Bh264"),
            Self::Bh265 => write!(f, "Bh265"),
            Self::Bh266 => write!(f, "Bh266"),
            Self::Bh267 => write!(f, "Bh267"),
            Self::Bh268 => write!(f, "Bh268"),
            Self::Bh269 => write!(f, "Bh269"),
            Self::Bh270 => write!(f, "Bh270"),
            Self::Bh271 => write!(f, "Bh271"),
            Self::Bh272 => write!(f, "Bh272"),
            Self::Bh273 => write!(f, "Bh273"),
            Self::Bh274 => write!(f, "Bh274"),
            Self::Bh275 => write!(f, "Bh275"),
        }
    }
}
