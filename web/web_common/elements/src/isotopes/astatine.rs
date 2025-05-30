//! Isotopes of the element Astatine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Astatine
pub enum AstatineIsotope {
    /// Isotope At191 of Astatine
    At191,
    /// Isotope At192 of Astatine
    At192,
    /// Isotope At193 of Astatine
    At193,
    /// Isotope At194 of Astatine
    At194,
    /// Isotope At195 of Astatine
    At195,
    /// Isotope At196 of Astatine
    At196,
    /// Isotope At197 of Astatine
    At197,
    /// Isotope At198 of Astatine
    At198,
    /// Isotope At199 of Astatine
    At199,
    /// Isotope At200 of Astatine
    At200,
    /// Isotope At201 of Astatine
    At201,
    /// Isotope At202 of Astatine
    At202,
    /// Isotope At203 of Astatine
    At203,
    /// Isotope At204 of Astatine
    At204,
    /// Isotope At205 of Astatine
    At205,
    /// Isotope At206 of Astatine
    At206,
    /// Isotope At207 of Astatine
    At207,
    /// Isotope At208 of Astatine
    At208,
    /// Isotope At209 of Astatine
    At209,
    /// Isotope At210 of Astatine
    At210,
    /// Isotope At211 of Astatine
    At211,
    /// Isotope At212 of Astatine
    At212,
    /// Isotope At213 of Astatine
    At213,
    /// Isotope At214 of Astatine
    At214,
    /// Isotope At215 of Astatine
    At215,
    /// Isotope At216 of Astatine
    At216,
    /// Isotope At217 of Astatine
    At217,
    /// Isotope At218 of Astatine
    At218,
    /// Isotope At219 of Astatine
    At219,
    /// Isotope At220 of Astatine
    At220,
    /// Isotope At221 of Astatine
    At221,
    /// Isotope At222 of Astatine
    At222,
    /// Isotope At223 of Astatine
    At223,
    /// Isotope At224 of Astatine
    At224,
    /// Isotope At225 of Astatine
    At225,
    /// Isotope At226 of Astatine
    At226,
    /// Isotope At227 of Astatine
    At227,
    /// Isotope At228 of Astatine
    At228,
    /// Isotope At229 of Astatine
    At229,
}
impl super::RelativeAtomicMass for AstatineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::At191 => 191.004148f64,
            Self::At192 => 192.003152f64,
            Self::At193 => 192.999927f64,
            Self::At194 => 193.999236f64,
            Self::At195 => 194.9962685f64,
            Self::At196 => 195.9958f64,
            Self::At197 => 196.993189f64,
            Self::At198 => 197.992784f64,
            Self::At199 => 198.9905277f64,
            Self::At200 => 199.990351f64,
            Self::At201 => 200.9884171f64,
            Self::At202 => 201.98863f64,
            Self::At203 => 202.986943f64,
            Self::At204 => 203.987251f64,
            Self::At205 => 204.986076f64,
            Self::At206 => 205.986657f64,
            Self::At207 => 206.9858f64,
            Self::At208 => 207.9866133f64,
            Self::At209 => 208.9861702f64,
            Self::At210 => 209.9871479f64,
            Self::At211 => 210.9874966f64,
            Self::At212 => 211.9907377f64,
            Self::At213 => 212.992937f64,
            Self::At214 => 213.9963721f64,
            Self::At215 => 214.9986528f64,
            Self::At216 => 216.0024236f64,
            Self::At217 => 217.0047192f64,
            Self::At218 => 218.008695f64,
            Self::At219 => 219.0111618f64,
            Self::At220 => 220.015433f64,
            Self::At221 => 221.018017f64,
            Self::At222 => 222.022494f64,
            Self::At223 => 223.025151f64,
            Self::At224 => 224.029749f64,
            Self::At225 => 225.03263f64,
            Self::At226 => 226.03716f64,
            Self::At227 => 227.04024f64,
            Self::At228 => 228.04475f64,
            Self::At229 => 229.04812f64,
        }
    }
}
impl super::ElementVariant for AstatineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::At
    }
}
impl super::MassNumber for AstatineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::At191 => 191u16,
            Self::At192 => 192u16,
            Self::At193 => 193u16,
            Self::At194 => 194u16,
            Self::At195 => 195u16,
            Self::At196 => 196u16,
            Self::At197 => 197u16,
            Self::At198 => 198u16,
            Self::At199 => 199u16,
            Self::At200 => 200u16,
            Self::At201 => 201u16,
            Self::At202 => 202u16,
            Self::At203 => 203u16,
            Self::At204 => 204u16,
            Self::At205 => 205u16,
            Self::At206 => 206u16,
            Self::At207 => 207u16,
            Self::At208 => 208u16,
            Self::At209 => 209u16,
            Self::At210 => 210u16,
            Self::At211 => 211u16,
            Self::At212 => 212u16,
            Self::At213 => 213u16,
            Self::At214 => 214u16,
            Self::At215 => 215u16,
            Self::At216 => 216u16,
            Self::At217 => 217u16,
            Self::At218 => 218u16,
            Self::At219 => 219u16,
            Self::At220 => 220u16,
            Self::At221 => 221u16,
            Self::At222 => 222u16,
            Self::At223 => 223u16,
            Self::At224 => 224u16,
            Self::At225 => 225u16,
            Self::At226 => 226u16,
            Self::At227 => 227u16,
            Self::At228 => 228u16,
            Self::At229 => 229u16,
        }
    }
}
impl super::IsotopicComposition for AstatineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for AstatineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::At229
    }
}
impl From<AstatineIsotope> for crate::Isotope {
    fn from(isotope: AstatineIsotope) -> Self {
        crate::Isotope::At(isotope)
    }
}
impl From<AstatineIsotope> for crate::Element {
    fn from(_isotope: AstatineIsotope) -> Self {
        crate::Element::At
    }
}
impl TryFrom<u16> for AstatineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            191u16 => Ok(Self::At191),
            192u16 => Ok(Self::At192),
            193u16 => Ok(Self::At193),
            194u16 => Ok(Self::At194),
            195u16 => Ok(Self::At195),
            196u16 => Ok(Self::At196),
            197u16 => Ok(Self::At197),
            198u16 => Ok(Self::At198),
            199u16 => Ok(Self::At199),
            200u16 => Ok(Self::At200),
            201u16 => Ok(Self::At201),
            202u16 => Ok(Self::At202),
            203u16 => Ok(Self::At203),
            204u16 => Ok(Self::At204),
            205u16 => Ok(Self::At205),
            206u16 => Ok(Self::At206),
            207u16 => Ok(Self::At207),
            208u16 => Ok(Self::At208),
            209u16 => Ok(Self::At209),
            210u16 => Ok(Self::At210),
            211u16 => Ok(Self::At211),
            212u16 => Ok(Self::At212),
            213u16 => Ok(Self::At213),
            214u16 => Ok(Self::At214),
            215u16 => Ok(Self::At215),
            216u16 => Ok(Self::At216),
            217u16 => Ok(Self::At217),
            218u16 => Ok(Self::At218),
            219u16 => Ok(Self::At219),
            220u16 => Ok(Self::At220),
            221u16 => Ok(Self::At221),
            222u16 => Ok(Self::At222),
            223u16 => Ok(Self::At223),
            224u16 => Ok(Self::At224),
            225u16 => Ok(Self::At225),
            226u16 => Ok(Self::At226),
            227u16 => Ok(Self::At227),
            228u16 => Ok(Self::At228),
            229u16 => Ok(Self::At229),
            _ => Err(crate::errors::Error::Isotope(crate::Element::At, value)),
        }
    }
}
impl std::fmt::Display for AstatineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::At191 => write!(f, "At191"),
            Self::At192 => write!(f, "At192"),
            Self::At193 => write!(f, "At193"),
            Self::At194 => write!(f, "At194"),
            Self::At195 => write!(f, "At195"),
            Self::At196 => write!(f, "At196"),
            Self::At197 => write!(f, "At197"),
            Self::At198 => write!(f, "At198"),
            Self::At199 => write!(f, "At199"),
            Self::At200 => write!(f, "At200"),
            Self::At201 => write!(f, "At201"),
            Self::At202 => write!(f, "At202"),
            Self::At203 => write!(f, "At203"),
            Self::At204 => write!(f, "At204"),
            Self::At205 => write!(f, "At205"),
            Self::At206 => write!(f, "At206"),
            Self::At207 => write!(f, "At207"),
            Self::At208 => write!(f, "At208"),
            Self::At209 => write!(f, "At209"),
            Self::At210 => write!(f, "At210"),
            Self::At211 => write!(f, "At211"),
            Self::At212 => write!(f, "At212"),
            Self::At213 => write!(f, "At213"),
            Self::At214 => write!(f, "At214"),
            Self::At215 => write!(f, "At215"),
            Self::At216 => write!(f, "At216"),
            Self::At217 => write!(f, "At217"),
            Self::At218 => write!(f, "At218"),
            Self::At219 => write!(f, "At219"),
            Self::At220 => write!(f, "At220"),
            Self::At221 => write!(f, "At221"),
            Self::At222 => write!(f, "At222"),
            Self::At223 => write!(f, "At223"),
            Self::At224 => write!(f, "At224"),
            Self::At225 => write!(f, "At225"),
            Self::At226 => write!(f, "At226"),
            Self::At227 => write!(f, "At227"),
            Self::At228 => write!(f, "At228"),
            Self::At229 => write!(f, "At229"),
        }
    }
}
