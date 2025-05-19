//! Isotopes of the element Nobelium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Nobelium
pub enum NobeliumIsotope {
    /// Isotope No248 of Nobelium
    No248,
    /// Isotope No249 of Nobelium
    No249,
    /// Isotope No250 of Nobelium
    No250,
    /// Isotope No251 of Nobelium
    No251,
    /// Isotope No252 of Nobelium
    No252,
    /// Isotope No253 of Nobelium
    No253,
    /// Isotope No254 of Nobelium
    No254,
    /// Isotope No255 of Nobelium
    No255,
    /// Isotope No256 of Nobelium
    No256,
    /// Isotope No257 of Nobelium
    No257,
    /// Isotope No258 of Nobelium
    No258,
    /// Isotope No259 of Nobelium
    No259,
    /// Isotope No260 of Nobelium
    No260,
    /// Isotope No261 of Nobelium
    No261,
    /// Isotope No262 of Nobelium
    No262,
    /// Isotope No263 of Nobelium
    No263,
    /// Isotope No264 of Nobelium
    No264,
}
impl super::RelativeAtomicMass for NobeliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::No248 => 248.08655f64,
            Self::No249 => 249.0878f64,
            Self::No250 => 250.08756f64,
            Self::No251 => 251.08894f64,
            Self::No252 => 252.088967f64,
            Self::No253 => 253.0905641f64,
            Self::No254 => 254.090956f64,
            Self::No255 => 255.093191f64,
            Self::No256 => 256.0942829f64,
            Self::No257 => 257.0968878f64,
            Self::No258 => 258.09821f64,
            Self::No259 => 259.10103f64,
            Self::No260 => 260.10264f64,
            Self::No261 => 261.1057f64,
            Self::No262 => 262.10746f64,
            Self::No263 => 263.11071f64,
            Self::No264 => 264.11273f64,
        }
    }
}
impl super::ElementVariant for NobeliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::No
    }
}
impl super::MassNumber for NobeliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::No248 => 248u16,
            Self::No249 => 249u16,
            Self::No250 => 250u16,
            Self::No251 => 251u16,
            Self::No252 => 252u16,
            Self::No253 => 253u16,
            Self::No254 => 254u16,
            Self::No255 => 255u16,
            Self::No256 => 256u16,
            Self::No257 => 257u16,
            Self::No258 => 258u16,
            Self::No259 => 259u16,
            Self::No260 => 260u16,
            Self::No261 => 261u16,
            Self::No262 => 262u16,
            Self::No263 => 263u16,
            Self::No264 => 264u16,
        }
    }
}
impl super::IsotopicComposition for NobeliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for NobeliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::No264
    }
}
impl TryFrom<u16> for NobeliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            248u16 => Ok(Self::No248),
            249u16 => Ok(Self::No249),
            250u16 => Ok(Self::No250),
            251u16 => Ok(Self::No251),
            252u16 => Ok(Self::No252),
            253u16 => Ok(Self::No253),
            254u16 => Ok(Self::No254),
            255u16 => Ok(Self::No255),
            256u16 => Ok(Self::No256),
            257u16 => Ok(Self::No257),
            258u16 => Ok(Self::No258),
            259u16 => Ok(Self::No259),
            260u16 => Ok(Self::No260),
            261u16 => Ok(Self::No261),
            262u16 => Ok(Self::No262),
            263u16 => Ok(Self::No263),
            264u16 => Ok(Self::No264),
            _ => Err(crate::errors::Error::Isotope(crate::Element::No, value)),
        }
    }
}
impl std::fmt::Display for NobeliumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No248 => write!(f, "No248"),
            Self::No249 => write!(f, "No249"),
            Self::No250 => write!(f, "No250"),
            Self::No251 => write!(f, "No251"),
            Self::No252 => write!(f, "No252"),
            Self::No253 => write!(f, "No253"),
            Self::No254 => write!(f, "No254"),
            Self::No255 => write!(f, "No255"),
            Self::No256 => write!(f, "No256"),
            Self::No257 => write!(f, "No257"),
            Self::No258 => write!(f, "No258"),
            Self::No259 => write!(f, "No259"),
            Self::No260 => write!(f, "No260"),
            Self::No261 => write!(f, "No261"),
            Self::No262 => write!(f, "No262"),
            Self::No263 => write!(f, "No263"),
            Self::No264 => write!(f, "No264"),
        }
    }
}
