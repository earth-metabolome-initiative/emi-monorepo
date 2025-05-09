#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum NickelIsotope {
    Ni48,
    Ni49,
    Ni50,
    Ni51,
    Ni52,
    Ni53,
    Ni54,
    Ni55,
    Ni56,
    Ni57,
    Ni58,
    Ni59,
    Ni60,
    Ni61,
    Ni62,
    Ni63,
    Ni64,
    Ni65,
    Ni66,
    Ni67,
    Ni68,
    Ni69,
    Ni70,
    Ni71,
    Ni72,
    Ni73,
    Ni74,
    Ni75,
    Ni76,
    Ni77,
    Ni78,
    Ni79,
}
impl super::RelativeAtomicMass for NickelIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ni48 => 48.01769f64,
            Self::Ni49 => 49.0077f64,
            Self::Ni50 => 49.99474f64,
            Self::Ni51 => 50.98611f64,
            Self::Ni52 => 51.9748f64,
            Self::Ni53 => 52.96819f64,
            Self::Ni54 => 53.957892f64,
            Self::Ni55 => 54.95133063f64,
            Self::Ni56 => 55.94212855f64,
            Self::Ni57 => 56.93979218f64,
            Self::Ni58 => 57.93534241f64,
            Self::Ni59 => 58.9343462f64,
            Self::Ni60 => 59.93078588f64,
            Self::Ni61 => 60.93105557f64,
            Self::Ni62 => 61.92834537f64,
            Self::Ni63 => 62.92966963f64,
            Self::Ni64 => 63.92796682f64,
            Self::Ni65 => 64.93008517f64,
            Self::Ni66 => 65.9291393f64,
            Self::Ni67 => 66.9315694f64,
            Self::Ni68 => 67.9318688f64,
            Self::Ni69 => 68.9356103f64,
            Self::Ni70 => 69.9364313f64,
            Self::Ni71 => 70.940519f64,
            Self::Ni72 => 71.9417859f64,
            Self::Ni73 => 72.9462067f64,
            Self::Ni74 => 73.94798f64,
            Self::Ni75 => 74.9525f64,
            Self::Ni76 => 75.95533f64,
            Self::Ni77 => 76.96055f64,
            Self::Ni78 => 77.96336f64,
            Self::Ni79 => 78.97025f64,
        }
    }
}
impl super::ElementVariant for NickelIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ni
    }
}
impl super::MassNumber for NickelIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ni48 => 48u16,
            Self::Ni49 => 49u16,
            Self::Ni50 => 50u16,
            Self::Ni51 => 51u16,
            Self::Ni52 => 52u16,
            Self::Ni53 => 53u16,
            Self::Ni54 => 54u16,
            Self::Ni55 => 55u16,
            Self::Ni56 => 56u16,
            Self::Ni57 => 57u16,
            Self::Ni58 => 58u16,
            Self::Ni59 => 59u16,
            Self::Ni60 => 60u16,
            Self::Ni61 => 61u16,
            Self::Ni62 => 62u16,
            Self::Ni63 => 63u16,
            Self::Ni64 => 64u16,
            Self::Ni65 => 65u16,
            Self::Ni66 => 66u16,
            Self::Ni67 => 67u16,
            Self::Ni68 => 68u16,
            Self::Ni69 => 69u16,
            Self::Ni70 => 70u16,
            Self::Ni71 => 71u16,
            Self::Ni72 => 72u16,
            Self::Ni73 => 73u16,
            Self::Ni74 => 74u16,
            Self::Ni75 => 75u16,
            Self::Ni76 => 76u16,
            Self::Ni77 => 77u16,
            Self::Ni78 => 78u16,
            Self::Ni79 => 79u16,
        }
    }
}
impl super::IsotopicComposition for NickelIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ni48 => None,
            Self::Ni49 => None,
            Self::Ni50 => None,
            Self::Ni51 => None,
            Self::Ni52 => None,
            Self::Ni53 => None,
            Self::Ni54 => None,
            Self::Ni55 => None,
            Self::Ni56 => None,
            Self::Ni57 => None,
            Self::Ni58 => Some(0.68077f64),
            Self::Ni59 => None,
            Self::Ni60 => Some(0.26223f64),
            Self::Ni61 => Some(0.011399f64),
            Self::Ni62 => Some(0.036346f64),
            Self::Ni63 => None,
            Self::Ni64 => Some(0.009255f64),
            Self::Ni65 => None,
            Self::Ni66 => None,
            Self::Ni67 => None,
            Self::Ni68 => None,
            Self::Ni69 => None,
            Self::Ni70 => None,
            Self::Ni71 => None,
            Self::Ni72 => None,
            Self::Ni73 => None,
            Self::Ni74 => None,
            Self::Ni75 => None,
            Self::Ni76 => None,
            Self::Ni77 => None,
            Self::Ni78 => None,
            Self::Ni79 => None,
        }
    }
}
impl super::MostAbundantIsotope for NickelIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ni58
    }
}
impl std::fmt::Display for NickelIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ni48 => write!(f, "Ni48"),
            Self::Ni49 => write!(f, "Ni49"),
            Self::Ni50 => write!(f, "Ni50"),
            Self::Ni51 => write!(f, "Ni51"),
            Self::Ni52 => write!(f, "Ni52"),
            Self::Ni53 => write!(f, "Ni53"),
            Self::Ni54 => write!(f, "Ni54"),
            Self::Ni55 => write!(f, "Ni55"),
            Self::Ni56 => write!(f, "Ni56"),
            Self::Ni57 => write!(f, "Ni57"),
            Self::Ni58 => write!(f, "Ni58"),
            Self::Ni59 => write!(f, "Ni59"),
            Self::Ni60 => write!(f, "Ni60"),
            Self::Ni61 => write!(f, "Ni61"),
            Self::Ni62 => write!(f, "Ni62"),
            Self::Ni63 => write!(f, "Ni63"),
            Self::Ni64 => write!(f, "Ni64"),
            Self::Ni65 => write!(f, "Ni65"),
            Self::Ni66 => write!(f, "Ni66"),
            Self::Ni67 => write!(f, "Ni67"),
            Self::Ni68 => write!(f, "Ni68"),
            Self::Ni69 => write!(f, "Ni69"),
            Self::Ni70 => write!(f, "Ni70"),
            Self::Ni71 => write!(f, "Ni71"),
            Self::Ni72 => write!(f, "Ni72"),
            Self::Ni73 => write!(f, "Ni73"),
            Self::Ni74 => write!(f, "Ni74"),
            Self::Ni75 => write!(f, "Ni75"),
            Self::Ni76 => write!(f, "Ni76"),
            Self::Ni77 => write!(f, "Ni77"),
            Self::Ni78 => write!(f, "Ni78"),
            Self::Ni79 => write!(f, "Ni79"),
        }
    }
}
