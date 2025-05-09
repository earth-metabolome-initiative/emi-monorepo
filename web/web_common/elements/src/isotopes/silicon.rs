#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum SiliconIsotope {
    Si22,
    Si23,
    Si24,
    Si25,
    Si26,
    Si27,
    Si28,
    Si29,
    Si30,
    Si31,
    Si32,
    Si33,
    Si34,
    Si35,
    Si36,
    Si37,
    Si38,
    Si39,
    Si40,
    Si41,
    Si42,
    Si43,
    Si44,
    Si45,
}
impl super::RelativeAtomicMass for SiliconIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Si22 => 22.03579f64,
            Self::Si23 => 23.02544f64,
            Self::Si24 => 24.011535f64,
            Self::Si25 => 25.004109f64,
            Self::Si26 => 25.99233384f64,
            Self::Si27 => 26.98670481f64,
            Self::Si28 => 27.97692653465f64,
            Self::Si29 => 28.9764946649f64,
            Self::Si30 => 29.973770136f64,
            Self::Si31 => 30.975363194f64,
            Self::Si32 => 31.97415154f64,
            Self::Si33 => 32.97797696f64,
            Self::Si34 => 33.978576f64,
            Self::Si35 => 34.984583f64,
            Self::Si36 => 35.986695f64,
            Self::Si37 => 36.992921f64,
            Self::Si38 => 37.995523f64,
            Self::Si39 => 39.002491f64,
            Self::Si40 => 40.00583f64,
            Self::Si41 => 41.01301f64,
            Self::Si42 => 42.01778f64,
            Self::Si43 => 43.0248f64,
            Self::Si44 => 44.03061f64,
            Self::Si45 => 45.03995f64,
        }
    }
}
impl super::ElementVariant for SiliconIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Si
    }
}
impl super::MassNumber for SiliconIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Si22 => 22u16,
            Self::Si23 => 23u16,
            Self::Si24 => 24u16,
            Self::Si25 => 25u16,
            Self::Si26 => 26u16,
            Self::Si27 => 27u16,
            Self::Si28 => 28u16,
            Self::Si29 => 29u16,
            Self::Si30 => 30u16,
            Self::Si31 => 31u16,
            Self::Si32 => 32u16,
            Self::Si33 => 33u16,
            Self::Si34 => 34u16,
            Self::Si35 => 35u16,
            Self::Si36 => 36u16,
            Self::Si37 => 37u16,
            Self::Si38 => 38u16,
            Self::Si39 => 39u16,
            Self::Si40 => 40u16,
            Self::Si41 => 41u16,
            Self::Si42 => 42u16,
            Self::Si43 => 43u16,
            Self::Si44 => 44u16,
            Self::Si45 => 45u16,
        }
    }
}
impl super::IsotopicComposition for SiliconIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Si22 => None,
            Self::Si23 => None,
            Self::Si24 => None,
            Self::Si25 => None,
            Self::Si26 => None,
            Self::Si27 => None,
            Self::Si28 => Some(0.92223f64),
            Self::Si29 => Some(0.04685f64),
            Self::Si30 => Some(0.03092f64),
            Self::Si31 => None,
            Self::Si32 => None,
            Self::Si33 => None,
            Self::Si34 => None,
            Self::Si35 => None,
            Self::Si36 => None,
            Self::Si37 => None,
            Self::Si38 => None,
            Self::Si39 => None,
            Self::Si40 => None,
            Self::Si41 => None,
            Self::Si42 => None,
            Self::Si43 => None,
            Self::Si44 => None,
            Self::Si45 => None,
        }
    }
}
impl super::MostAbundantIsotope for SiliconIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Si28
    }
}
impl TryFrom<u16> for SiliconIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            22u16 => Ok(Self::Si22),
            23u16 => Ok(Self::Si23),
            24u16 => Ok(Self::Si24),
            25u16 => Ok(Self::Si25),
            26u16 => Ok(Self::Si26),
            27u16 => Ok(Self::Si27),
            28u16 => Ok(Self::Si28),
            29u16 => Ok(Self::Si29),
            30u16 => Ok(Self::Si30),
            31u16 => Ok(Self::Si31),
            32u16 => Ok(Self::Si32),
            33u16 => Ok(Self::Si33),
            34u16 => Ok(Self::Si34),
            35u16 => Ok(Self::Si35),
            36u16 => Ok(Self::Si36),
            37u16 => Ok(Self::Si37),
            38u16 => Ok(Self::Si38),
            39u16 => Ok(Self::Si39),
            40u16 => Ok(Self::Si40),
            41u16 => Ok(Self::Si41),
            42u16 => Ok(Self::Si42),
            43u16 => Ok(Self::Si43),
            44u16 => Ok(Self::Si44),
            45u16 => Ok(Self::Si45),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Si, value)),
        }
    }
}
impl std::fmt::Display for SiliconIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Si22 => write!(f, "Si22"),
            Self::Si23 => write!(f, "Si23"),
            Self::Si24 => write!(f, "Si24"),
            Self::Si25 => write!(f, "Si25"),
            Self::Si26 => write!(f, "Si26"),
            Self::Si27 => write!(f, "Si27"),
            Self::Si28 => write!(f, "Si28"),
            Self::Si29 => write!(f, "Si29"),
            Self::Si30 => write!(f, "Si30"),
            Self::Si31 => write!(f, "Si31"),
            Self::Si32 => write!(f, "Si32"),
            Self::Si33 => write!(f, "Si33"),
            Self::Si34 => write!(f, "Si34"),
            Self::Si35 => write!(f, "Si35"),
            Self::Si36 => write!(f, "Si36"),
            Self::Si37 => write!(f, "Si37"),
            Self::Si38 => write!(f, "Si38"),
            Self::Si39 => write!(f, "Si39"),
            Self::Si40 => write!(f, "Si40"),
            Self::Si41 => write!(f, "Si41"),
            Self::Si42 => write!(f, "Si42"),
            Self::Si43 => write!(f, "Si43"),
            Self::Si44 => write!(f, "Si44"),
            Self::Si45 => write!(f, "Si45"),
        }
    }
}
