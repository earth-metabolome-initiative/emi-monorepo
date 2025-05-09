#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum CalciumIsotope {
    Ca34,
    Ca35,
    Ca36,
    Ca37,
    Ca38,
    Ca39,
    Ca40,
    Ca41,
    Ca42,
    Ca43,
    Ca44,
    Ca45,
    Ca46,
    Ca47,
    Ca48,
    Ca49,
    Ca50,
    Ca51,
    Ca52,
    Ca53,
    Ca54,
    Ca55,
    Ca56,
    Ca57,
    Ca58,
}
impl super::RelativeAtomicMass for CalciumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ca34 => 34.01487f64,
            Self::Ca35 => 35.00514f64,
            Self::Ca36 => 35.993074f64,
            Self::Ca37 => 36.98589785f64,
            Self::Ca38 => 37.97631922f64,
            Self::Ca39 => 38.97071081f64,
            Self::Ca40 => 39.962590863f64,
            Self::Ca41 => 40.96227792f64,
            Self::Ca42 => 41.95861783f64,
            Self::Ca43 => 42.95876644f64,
            Self::Ca44 => 43.95548156f64,
            Self::Ca45 => 44.95618635f64,
            Self::Ca46 => 45.953689f64,
            Self::Ca47 => 46.9545424f64,
            Self::Ca48 => 47.95252276f64,
            Self::Ca49 => 48.95566274f64,
            Self::Ca50 => 49.9574992f64,
            Self::Ca51 => 50.960989f64,
            Self::Ca52 => 51.963217f64,
            Self::Ca53 => 52.96945f64,
            Self::Ca54 => 53.9734f64,
            Self::Ca55 => 54.9803f64,
            Self::Ca56 => 55.98508f64,
            Self::Ca57 => 56.99262f64,
            Self::Ca58 => 57.99794f64,
        }
    }
}
impl super::ElementVariant for CalciumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ca
    }
}
impl super::MassNumber for CalciumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ca34 => 34u16,
            Self::Ca35 => 35u16,
            Self::Ca36 => 36u16,
            Self::Ca37 => 37u16,
            Self::Ca38 => 38u16,
            Self::Ca39 => 39u16,
            Self::Ca40 => 40u16,
            Self::Ca41 => 41u16,
            Self::Ca42 => 42u16,
            Self::Ca43 => 43u16,
            Self::Ca44 => 44u16,
            Self::Ca45 => 45u16,
            Self::Ca46 => 46u16,
            Self::Ca47 => 47u16,
            Self::Ca48 => 48u16,
            Self::Ca49 => 49u16,
            Self::Ca50 => 50u16,
            Self::Ca51 => 51u16,
            Self::Ca52 => 52u16,
            Self::Ca53 => 53u16,
            Self::Ca54 => 54u16,
            Self::Ca55 => 55u16,
            Self::Ca56 => 56u16,
            Self::Ca57 => 57u16,
            Self::Ca58 => 58u16,
        }
    }
}
impl super::IsotopicComposition for CalciumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ca34 => None,
            Self::Ca35 => None,
            Self::Ca36 => None,
            Self::Ca37 => None,
            Self::Ca38 => None,
            Self::Ca39 => None,
            Self::Ca40 => Some(0.96941f64),
            Self::Ca41 => None,
            Self::Ca42 => Some(0.00647f64),
            Self::Ca43 => Some(0.00135f64),
            Self::Ca44 => Some(0.02086f64),
            Self::Ca45 => None,
            Self::Ca46 => Some(0.00004f64),
            Self::Ca47 => None,
            Self::Ca48 => Some(0.00187f64),
            Self::Ca49 => None,
            Self::Ca50 => None,
            Self::Ca51 => None,
            Self::Ca52 => None,
            Self::Ca53 => None,
            Self::Ca54 => None,
            Self::Ca55 => None,
            Self::Ca56 => None,
            Self::Ca57 => None,
            Self::Ca58 => None,
        }
    }
}
impl super::MostAbundantIsotope for CalciumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ca40
    }
}
impl TryFrom<u16> for CalciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            34u16 => Ok(Self::Ca34),
            35u16 => Ok(Self::Ca35),
            36u16 => Ok(Self::Ca36),
            37u16 => Ok(Self::Ca37),
            38u16 => Ok(Self::Ca38),
            39u16 => Ok(Self::Ca39),
            40u16 => Ok(Self::Ca40),
            41u16 => Ok(Self::Ca41),
            42u16 => Ok(Self::Ca42),
            43u16 => Ok(Self::Ca43),
            44u16 => Ok(Self::Ca44),
            45u16 => Ok(Self::Ca45),
            46u16 => Ok(Self::Ca46),
            47u16 => Ok(Self::Ca47),
            48u16 => Ok(Self::Ca48),
            49u16 => Ok(Self::Ca49),
            50u16 => Ok(Self::Ca50),
            51u16 => Ok(Self::Ca51),
            52u16 => Ok(Self::Ca52),
            53u16 => Ok(Self::Ca53),
            54u16 => Ok(Self::Ca54),
            55u16 => Ok(Self::Ca55),
            56u16 => Ok(Self::Ca56),
            57u16 => Ok(Self::Ca57),
            58u16 => Ok(Self::Ca58),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ca, value)),
        }
    }
}
impl std::fmt::Display for CalciumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ca34 => write!(f, "Ca34"),
            Self::Ca35 => write!(f, "Ca35"),
            Self::Ca36 => write!(f, "Ca36"),
            Self::Ca37 => write!(f, "Ca37"),
            Self::Ca38 => write!(f, "Ca38"),
            Self::Ca39 => write!(f, "Ca39"),
            Self::Ca40 => write!(f, "Ca40"),
            Self::Ca41 => write!(f, "Ca41"),
            Self::Ca42 => write!(f, "Ca42"),
            Self::Ca43 => write!(f, "Ca43"),
            Self::Ca44 => write!(f, "Ca44"),
            Self::Ca45 => write!(f, "Ca45"),
            Self::Ca46 => write!(f, "Ca46"),
            Self::Ca47 => write!(f, "Ca47"),
            Self::Ca48 => write!(f, "Ca48"),
            Self::Ca49 => write!(f, "Ca49"),
            Self::Ca50 => write!(f, "Ca50"),
            Self::Ca51 => write!(f, "Ca51"),
            Self::Ca52 => write!(f, "Ca52"),
            Self::Ca53 => write!(f, "Ca53"),
            Self::Ca54 => write!(f, "Ca54"),
            Self::Ca55 => write!(f, "Ca55"),
            Self::Ca56 => write!(f, "Ca56"),
            Self::Ca57 => write!(f, "Ca57"),
            Self::Ca58 => write!(f, "Ca58"),
        }
    }
}
