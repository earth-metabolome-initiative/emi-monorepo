//! Isotopes of the element Iron
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Iron
pub enum IronIsotope {
    /// Isotope Fe45 of Iron
    Fe45,
    /// Isotope Fe46 of Iron
    Fe46,
    /// Isotope Fe47 of Iron
    Fe47,
    /// Isotope Fe48 of Iron
    Fe48,
    /// Isotope Fe49 of Iron
    Fe49,
    /// Isotope Fe50 of Iron
    Fe50,
    /// Isotope Fe51 of Iron
    Fe51,
    /// Isotope Fe52 of Iron
    Fe52,
    /// Isotope Fe53 of Iron
    Fe53,
    /// Isotope Fe54 of Iron
    Fe54,
    /// Isotope Fe55 of Iron
    Fe55,
    /// Isotope Fe56 of Iron
    Fe56,
    /// Isotope Fe57 of Iron
    Fe57,
    /// Isotope Fe58 of Iron
    Fe58,
    /// Isotope Fe59 of Iron
    Fe59,
    /// Isotope Fe60 of Iron
    Fe60,
    /// Isotope Fe61 of Iron
    Fe61,
    /// Isotope Fe62 of Iron
    Fe62,
    /// Isotope Fe63 of Iron
    Fe63,
    /// Isotope Fe64 of Iron
    Fe64,
    /// Isotope Fe65 of Iron
    Fe65,
    /// Isotope Fe66 of Iron
    Fe66,
    /// Isotope Fe67 of Iron
    Fe67,
    /// Isotope Fe68 of Iron
    Fe68,
    /// Isotope Fe69 of Iron
    Fe69,
    /// Isotope Fe70 of Iron
    Fe70,
    /// Isotope Fe71 of Iron
    Fe71,
    /// Isotope Fe72 of Iron
    Fe72,
    /// Isotope Fe73 of Iron
    Fe73,
    /// Isotope Fe74 of Iron
    Fe74,
}
impl super::RelativeAtomicMass for IronIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Fe45 => 45.01442f64,
            Self::Fe46 => 46.00063f64,
            Self::Fe47 => 46.99185f64,
            Self::Fe48 => 47.98023f64,
            Self::Fe49 => 48.973429f64,
            Self::Fe50 => 49.962975f64,
            Self::Fe51 => 50.956841f64,
            Self::Fe52 => 51.9481131f64,
            Self::Fe53 => 52.9453064f64,
            Self::Fe54 => 53.93960899f64,
            Self::Fe55 => 54.93829199f64,
            Self::Fe56 => 55.93493633f64,
            Self::Fe57 => 56.93539284f64,
            Self::Fe58 => 57.93327443f64,
            Self::Fe59 => 58.93487434f64,
            Self::Fe60 => 59.9340711f64,
            Self::Fe61 => 60.9367462f64,
            Self::Fe62 => 61.9367918f64,
            Self::Fe63 => 62.9402727f64,
            Self::Fe64 => 63.9409878f64,
            Self::Fe65 => 64.9450115f64,
            Self::Fe66 => 65.94625f64,
            Self::Fe67 => 66.95054f64,
            Self::Fe68 => 67.95295f64,
            Self::Fe69 => 68.95807f64,
            Self::Fe70 => 69.96102f64,
            Self::Fe71 => 70.96672f64,
            Self::Fe72 => 71.96983f64,
            Self::Fe73 => 72.97572f64,
            Self::Fe74 => 73.97935f64,
        }
    }
}
impl super::ElementVariant for IronIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Fe
    }
}
impl super::MassNumber for IronIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Fe45 => 45u16,
            Self::Fe46 => 46u16,
            Self::Fe47 => 47u16,
            Self::Fe48 => 48u16,
            Self::Fe49 => 49u16,
            Self::Fe50 => 50u16,
            Self::Fe51 => 51u16,
            Self::Fe52 => 52u16,
            Self::Fe53 => 53u16,
            Self::Fe54 => 54u16,
            Self::Fe55 => 55u16,
            Self::Fe56 => 56u16,
            Self::Fe57 => 57u16,
            Self::Fe58 => 58u16,
            Self::Fe59 => 59u16,
            Self::Fe60 => 60u16,
            Self::Fe61 => 61u16,
            Self::Fe62 => 62u16,
            Self::Fe63 => 63u16,
            Self::Fe64 => 64u16,
            Self::Fe65 => 65u16,
            Self::Fe66 => 66u16,
            Self::Fe67 => 67u16,
            Self::Fe68 => 68u16,
            Self::Fe69 => 69u16,
            Self::Fe70 => 70u16,
            Self::Fe71 => 71u16,
            Self::Fe72 => 72u16,
            Self::Fe73 => 73u16,
            Self::Fe74 => 74u16,
        }
    }
}
impl super::IsotopicComposition for IronIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Fe54 => Some(0.05845f64),
            Self::Fe56 => Some(0.91754f64),
            Self::Fe57 => Some(0.02119f64),
            Self::Fe58 => Some(0.00282f64),
            Self::Fe45
            | Self::Fe46
            | Self::Fe47
            | Self::Fe48
            | Self::Fe49
            | Self::Fe50
            | Self::Fe51
            | Self::Fe52
            | Self::Fe53
            | Self::Fe55
            | Self::Fe59
            | Self::Fe60
            | Self::Fe61
            | Self::Fe62
            | Self::Fe63
            | Self::Fe64
            | Self::Fe65
            | Self::Fe66
            | Self::Fe67
            | Self::Fe68
            | Self::Fe69
            | Self::Fe70
            | Self::Fe71
            | Self::Fe72
            | Self::Fe73
            | Self::Fe74 => None,
        }
    }
}
impl super::MostAbundantIsotope for IronIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Fe56
    }
}
impl From<IronIsotope> for crate::Isotope {
    fn from(isotope: IronIsotope) -> Self {
        crate::Isotope::Fe(isotope)
    }
}
impl From<IronIsotope> for crate::Element {
    fn from(_isotope: IronIsotope) -> Self {
        crate::Element::Fe
    }
}
impl TryFrom<u16> for IronIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            45u16 => Ok(Self::Fe45),
            46u16 => Ok(Self::Fe46),
            47u16 => Ok(Self::Fe47),
            48u16 => Ok(Self::Fe48),
            49u16 => Ok(Self::Fe49),
            50u16 => Ok(Self::Fe50),
            51u16 => Ok(Self::Fe51),
            52u16 => Ok(Self::Fe52),
            53u16 => Ok(Self::Fe53),
            54u16 => Ok(Self::Fe54),
            55u16 => Ok(Self::Fe55),
            56u16 => Ok(Self::Fe56),
            57u16 => Ok(Self::Fe57),
            58u16 => Ok(Self::Fe58),
            59u16 => Ok(Self::Fe59),
            60u16 => Ok(Self::Fe60),
            61u16 => Ok(Self::Fe61),
            62u16 => Ok(Self::Fe62),
            63u16 => Ok(Self::Fe63),
            64u16 => Ok(Self::Fe64),
            65u16 => Ok(Self::Fe65),
            66u16 => Ok(Self::Fe66),
            67u16 => Ok(Self::Fe67),
            68u16 => Ok(Self::Fe68),
            69u16 => Ok(Self::Fe69),
            70u16 => Ok(Self::Fe70),
            71u16 => Ok(Self::Fe71),
            72u16 => Ok(Self::Fe72),
            73u16 => Ok(Self::Fe73),
            74u16 => Ok(Self::Fe74),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Fe, value)),
        }
    }
}
impl std::fmt::Display for IronIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fe45 => write!(f, "Fe45"),
            Self::Fe46 => write!(f, "Fe46"),
            Self::Fe47 => write!(f, "Fe47"),
            Self::Fe48 => write!(f, "Fe48"),
            Self::Fe49 => write!(f, "Fe49"),
            Self::Fe50 => write!(f, "Fe50"),
            Self::Fe51 => write!(f, "Fe51"),
            Self::Fe52 => write!(f, "Fe52"),
            Self::Fe53 => write!(f, "Fe53"),
            Self::Fe54 => write!(f, "Fe54"),
            Self::Fe55 => write!(f, "Fe55"),
            Self::Fe56 => write!(f, "Fe56"),
            Self::Fe57 => write!(f, "Fe57"),
            Self::Fe58 => write!(f, "Fe58"),
            Self::Fe59 => write!(f, "Fe59"),
            Self::Fe60 => write!(f, "Fe60"),
            Self::Fe61 => write!(f, "Fe61"),
            Self::Fe62 => write!(f, "Fe62"),
            Self::Fe63 => write!(f, "Fe63"),
            Self::Fe64 => write!(f, "Fe64"),
            Self::Fe65 => write!(f, "Fe65"),
            Self::Fe66 => write!(f, "Fe66"),
            Self::Fe67 => write!(f, "Fe67"),
            Self::Fe68 => write!(f, "Fe68"),
            Self::Fe69 => write!(f, "Fe69"),
            Self::Fe70 => write!(f, "Fe70"),
            Self::Fe71 => write!(f, "Fe71"),
            Self::Fe72 => write!(f, "Fe72"),
            Self::Fe73 => write!(f, "Fe73"),
            Self::Fe74 => write!(f, "Fe74"),
        }
    }
}
