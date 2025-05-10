//! Isotopes of the element Potassium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Potassium
pub enum PotassiumIsotope {
    /// Isotope K32 of Potassium
    K32,
    /// Isotope K33 of Potassium
    K33,
    /// Isotope K34 of Potassium
    K34,
    /// Isotope K35 of Potassium
    K35,
    /// Isotope K36 of Potassium
    K36,
    /// Isotope K37 of Potassium
    K37,
    /// Isotope K38 of Potassium
    K38,
    /// Isotope K39 of Potassium
    K39,
    /// Isotope K40 of Potassium
    K40,
    /// Isotope K41 of Potassium
    K41,
    /// Isotope K42 of Potassium
    K42,
    /// Isotope K43 of Potassium
    K43,
    /// Isotope K44 of Potassium
    K44,
    /// Isotope K45 of Potassium
    K45,
    /// Isotope K46 of Potassium
    K46,
    /// Isotope K47 of Potassium
    K47,
    /// Isotope K48 of Potassium
    K48,
    /// Isotope K49 of Potassium
    K49,
    /// Isotope K50 of Potassium
    K50,
    /// Isotope K51 of Potassium
    K51,
    /// Isotope K52 of Potassium
    K52,
    /// Isotope K53 of Potassium
    K53,
    /// Isotope K54 of Potassium
    K54,
    /// Isotope K55 of Potassium
    K55,
    /// Isotope K56 of Potassium
    K56,
}
impl super::RelativeAtomicMass for PotassiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::K32 => 32.02265f64,
            Self::K33 => 33.00756f64,
            Self::K34 => 33.99869f64,
            Self::K35 => 34.98800541f64,
            Self::K36 => 35.98130201f64,
            Self::K37 => 36.97337589f64,
            Self::K38 => 37.96908112f64,
            Self::K39 => 38.9637064864f64,
            Self::K40 => 39.963998166f64,
            Self::K41 => 40.9618252579f64,
            Self::K42 => 41.96240231f64,
            Self::K43 => 42.9607347f64,
            Self::K44 => 43.96158699f64,
            Self::K45 => 44.96069149f64,
            Self::K46 => 45.96198159f64,
            Self::K47 => 46.9616616f64,
            Self::K48 => 47.96534119f64,
            Self::K49 => 48.96821075f64,
            Self::K50 => 49.97238f64,
            Self::K51 => 50.975828f64,
            Self::K52 => 51.98224f64,
            Self::K53 => 52.98746f64,
            Self::K54 => 53.99463f64,
            Self::K55 => 55.00076f64,
            Self::K56 => 56.00851f64,
        }
    }
}
impl super::ElementVariant for PotassiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::K
    }
}
impl super::MassNumber for PotassiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::K32 => 32u16,
            Self::K33 => 33u16,
            Self::K34 => 34u16,
            Self::K35 => 35u16,
            Self::K36 => 36u16,
            Self::K37 => 37u16,
            Self::K38 => 38u16,
            Self::K39 => 39u16,
            Self::K40 => 40u16,
            Self::K41 => 41u16,
            Self::K42 => 42u16,
            Self::K43 => 43u16,
            Self::K44 => 44u16,
            Self::K45 => 45u16,
            Self::K46 => 46u16,
            Self::K47 => 47u16,
            Self::K48 => 48u16,
            Self::K49 => 49u16,
            Self::K50 => 50u16,
            Self::K51 => 51u16,
            Self::K52 => 52u16,
            Self::K53 => 53u16,
            Self::K54 => 54u16,
            Self::K55 => 55u16,
            Self::K56 => 56u16,
        }
    }
}
impl super::IsotopicComposition for PotassiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::K39 => Some(0.932581f64),
            Self::K40 => Some(0.000117f64),
            Self::K41 => Some(0.067302f64),
            Self::K32
            | Self::K33
            | Self::K34
            | Self::K35
            | Self::K36
            | Self::K37
            | Self::K38
            | Self::K42
            | Self::K43
            | Self::K44
            | Self::K45
            | Self::K46
            | Self::K47
            | Self::K48
            | Self::K49
            | Self::K50
            | Self::K51
            | Self::K52
            | Self::K53
            | Self::K54
            | Self::K55
            | Self::K56 => None,
        }
    }
}
impl super::MostAbundantIsotope for PotassiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::K39
    }
}
impl TryFrom<u16> for PotassiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            32u16 => Ok(Self::K32),
            33u16 => Ok(Self::K33),
            34u16 => Ok(Self::K34),
            35u16 => Ok(Self::K35),
            36u16 => Ok(Self::K36),
            37u16 => Ok(Self::K37),
            38u16 => Ok(Self::K38),
            39u16 => Ok(Self::K39),
            40u16 => Ok(Self::K40),
            41u16 => Ok(Self::K41),
            42u16 => Ok(Self::K42),
            43u16 => Ok(Self::K43),
            44u16 => Ok(Self::K44),
            45u16 => Ok(Self::K45),
            46u16 => Ok(Self::K46),
            47u16 => Ok(Self::K47),
            48u16 => Ok(Self::K48),
            49u16 => Ok(Self::K49),
            50u16 => Ok(Self::K50),
            51u16 => Ok(Self::K51),
            52u16 => Ok(Self::K52),
            53u16 => Ok(Self::K53),
            54u16 => Ok(Self::K54),
            55u16 => Ok(Self::K55),
            56u16 => Ok(Self::K56),
            _ => Err(crate::errors::Error::Isotope(crate::Element::K, value)),
        }
    }
}
impl std::fmt::Display for PotassiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::K32 => write!(f, "K32"),
            Self::K33 => write!(f, "K33"),
            Self::K34 => write!(f, "K34"),
            Self::K35 => write!(f, "K35"),
            Self::K36 => write!(f, "K36"),
            Self::K37 => write!(f, "K37"),
            Self::K38 => write!(f, "K38"),
            Self::K39 => write!(f, "K39"),
            Self::K40 => write!(f, "K40"),
            Self::K41 => write!(f, "K41"),
            Self::K42 => write!(f, "K42"),
            Self::K43 => write!(f, "K43"),
            Self::K44 => write!(f, "K44"),
            Self::K45 => write!(f, "K45"),
            Self::K46 => write!(f, "K46"),
            Self::K47 => write!(f, "K47"),
            Self::K48 => write!(f, "K48"),
            Self::K49 => write!(f, "K49"),
            Self::K50 => write!(f, "K50"),
            Self::K51 => write!(f, "K51"),
            Self::K52 => write!(f, "K52"),
            Self::K53 => write!(f, "K53"),
            Self::K54 => write!(f, "K54"),
            Self::K55 => write!(f, "K55"),
            Self::K56 => write!(f, "K56"),
        }
    }
}
