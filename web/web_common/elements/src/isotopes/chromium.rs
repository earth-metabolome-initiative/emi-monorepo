#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ChromiumIsotope {
    Cr42,
    Cr43,
    Cr44,
    Cr45,
    Cr46,
    Cr47,
    Cr48,
    Cr49,
    Cr50,
    Cr51,
    Cr52,
    Cr53,
    Cr54,
    Cr55,
    Cr56,
    Cr57,
    Cr58,
    Cr59,
    Cr60,
    Cr61,
    Cr62,
    Cr63,
    Cr64,
    Cr65,
    Cr66,
    Cr67,
    Cr68,
}
impl super::RelativeAtomicMass for ChromiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cr42 => 42.0067f64,
            Self::Cr43 => 42.99753f64,
            Self::Cr44 => 43.98536f64,
            Self::Cr45 => 44.97905f64,
            Self::Cr46 => 45.968359f64,
            Self::Cr47 => 46.9628974f64,
            Self::Cr48 => 47.9540291f64,
            Self::Cr49 => 48.9513333f64,
            Self::Cr50 => 49.94604183f64,
            Self::Cr51 => 50.94476502f64,
            Self::Cr52 => 51.94050623f64,
            Self::Cr53 => 52.94064815f64,
            Self::Cr54 => 53.93887916f64,
            Self::Cr55 => 54.94083843f64,
            Self::Cr56 => 55.9406531f64,
            Self::Cr57 => 56.943613f64,
            Self::Cr58 => 57.94435f64,
            Self::Cr59 => 58.94859f64,
            Self::Cr60 => 59.95008f64,
            Self::Cr61 => 60.95442f64,
            Self::Cr62 => 61.9561f64,
            Self::Cr63 => 62.96165f64,
            Self::Cr64 => 63.96408f64,
            Self::Cr65 => 64.96996f64,
            Self::Cr66 => 65.97366f64,
            Self::Cr67 => 66.98016f64,
            Self::Cr68 => 67.98403f64,
        }
    }
}
impl super::ElementVariant for ChromiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cr
    }
}
impl super::MassNumber for ChromiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cr42 => 42u16,
            Self::Cr43 => 43u16,
            Self::Cr44 => 44u16,
            Self::Cr45 => 45u16,
            Self::Cr46 => 46u16,
            Self::Cr47 => 47u16,
            Self::Cr48 => 48u16,
            Self::Cr49 => 49u16,
            Self::Cr50 => 50u16,
            Self::Cr51 => 51u16,
            Self::Cr52 => 52u16,
            Self::Cr53 => 53u16,
            Self::Cr54 => 54u16,
            Self::Cr55 => 55u16,
            Self::Cr56 => 56u16,
            Self::Cr57 => 57u16,
            Self::Cr58 => 58u16,
            Self::Cr59 => 59u16,
            Self::Cr60 => 60u16,
            Self::Cr61 => 61u16,
            Self::Cr62 => 62u16,
            Self::Cr63 => 63u16,
            Self::Cr64 => 64u16,
            Self::Cr65 => 65u16,
            Self::Cr66 => 66u16,
            Self::Cr67 => 67u16,
            Self::Cr68 => 68u16,
        }
    }
}
impl super::IsotopicComposition for ChromiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cr42 => None,
            Self::Cr43 => None,
            Self::Cr44 => None,
            Self::Cr45 => None,
            Self::Cr46 => None,
            Self::Cr47 => None,
            Self::Cr48 => None,
            Self::Cr49 => None,
            Self::Cr50 => Some(0.04345f64),
            Self::Cr51 => None,
            Self::Cr52 => Some(0.83789f64),
            Self::Cr53 => Some(0.09501f64),
            Self::Cr54 => Some(0.02365f64),
            Self::Cr55 => None,
            Self::Cr56 => None,
            Self::Cr57 => None,
            Self::Cr58 => None,
            Self::Cr59 => None,
            Self::Cr60 => None,
            Self::Cr61 => None,
            Self::Cr62 => None,
            Self::Cr63 => None,
            Self::Cr64 => None,
            Self::Cr65 => None,
            Self::Cr66 => None,
            Self::Cr67 => None,
            Self::Cr68 => None,
        }
    }
}
impl super::MostAbundantIsotope for ChromiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cr52
    }
}
impl TryFrom<u16> for ChromiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            42u16 => Ok(Self::Cr42),
            43u16 => Ok(Self::Cr43),
            44u16 => Ok(Self::Cr44),
            45u16 => Ok(Self::Cr45),
            46u16 => Ok(Self::Cr46),
            47u16 => Ok(Self::Cr47),
            48u16 => Ok(Self::Cr48),
            49u16 => Ok(Self::Cr49),
            50u16 => Ok(Self::Cr50),
            51u16 => Ok(Self::Cr51),
            52u16 => Ok(Self::Cr52),
            53u16 => Ok(Self::Cr53),
            54u16 => Ok(Self::Cr54),
            55u16 => Ok(Self::Cr55),
            56u16 => Ok(Self::Cr56),
            57u16 => Ok(Self::Cr57),
            58u16 => Ok(Self::Cr58),
            59u16 => Ok(Self::Cr59),
            60u16 => Ok(Self::Cr60),
            61u16 => Ok(Self::Cr61),
            62u16 => Ok(Self::Cr62),
            63u16 => Ok(Self::Cr63),
            64u16 => Ok(Self::Cr64),
            65u16 => Ok(Self::Cr65),
            66u16 => Ok(Self::Cr66),
            67u16 => Ok(Self::Cr67),
            68u16 => Ok(Self::Cr68),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cr, value)),
        }
    }
}
impl std::fmt::Display for ChromiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cr42 => write!(f, "Cr42"),
            Self::Cr43 => write!(f, "Cr43"),
            Self::Cr44 => write!(f, "Cr44"),
            Self::Cr45 => write!(f, "Cr45"),
            Self::Cr46 => write!(f, "Cr46"),
            Self::Cr47 => write!(f, "Cr47"),
            Self::Cr48 => write!(f, "Cr48"),
            Self::Cr49 => write!(f, "Cr49"),
            Self::Cr50 => write!(f, "Cr50"),
            Self::Cr51 => write!(f, "Cr51"),
            Self::Cr52 => write!(f, "Cr52"),
            Self::Cr53 => write!(f, "Cr53"),
            Self::Cr54 => write!(f, "Cr54"),
            Self::Cr55 => write!(f, "Cr55"),
            Self::Cr56 => write!(f, "Cr56"),
            Self::Cr57 => write!(f, "Cr57"),
            Self::Cr58 => write!(f, "Cr58"),
            Self::Cr59 => write!(f, "Cr59"),
            Self::Cr60 => write!(f, "Cr60"),
            Self::Cr61 => write!(f, "Cr61"),
            Self::Cr62 => write!(f, "Cr62"),
            Self::Cr63 => write!(f, "Cr63"),
            Self::Cr64 => write!(f, "Cr64"),
            Self::Cr65 => write!(f, "Cr65"),
            Self::Cr66 => write!(f, "Cr66"),
            Self::Cr67 => write!(f, "Cr67"),
            Self::Cr68 => write!(f, "Cr68"),
        }
    }
}
