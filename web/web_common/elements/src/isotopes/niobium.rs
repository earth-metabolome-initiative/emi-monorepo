//! Isotopes of the element Niobium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Niobium
pub enum NiobiumIsotope {
    /// Isotope Nb81 of Niobium
    Nb81,
    /// Isotope Nb82 of Niobium
    Nb82,
    /// Isotope Nb83 of Niobium
    Nb83,
    /// Isotope Nb84 of Niobium
    Nb84,
    /// Isotope Nb85 of Niobium
    Nb85,
    /// Isotope Nb86 of Niobium
    Nb86,
    /// Isotope Nb87 of Niobium
    Nb87,
    /// Isotope Nb88 of Niobium
    Nb88,
    /// Isotope Nb89 of Niobium
    Nb89,
    /// Isotope Nb90 of Niobium
    Nb90,
    /// Isotope Nb91 of Niobium
    Nb91,
    /// Isotope Nb92 of Niobium
    Nb92,
    /// Isotope Nb93 of Niobium
    Nb93,
    /// Isotope Nb94 of Niobium
    Nb94,
    /// Isotope Nb95 of Niobium
    Nb95,
    /// Isotope Nb96 of Niobium
    Nb96,
    /// Isotope Nb97 of Niobium
    Nb97,
    /// Isotope Nb98 of Niobium
    Nb98,
    /// Isotope Nb99 of Niobium
    Nb99,
    /// Isotope Nb100 of Niobium
    Nb100,
    /// Isotope Nb101 of Niobium
    Nb101,
    /// Isotope Nb102 of Niobium
    Nb102,
    /// Isotope Nb103 of Niobium
    Nb103,
    /// Isotope Nb104 of Niobium
    Nb104,
    /// Isotope Nb105 of Niobium
    Nb105,
    /// Isotope Nb106 of Niobium
    Nb106,
    /// Isotope Nb107 of Niobium
    Nb107,
    /// Isotope Nb108 of Niobium
    Nb108,
    /// Isotope Nb109 of Niobium
    Nb109,
    /// Isotope Nb110 of Niobium
    Nb110,
    /// Isotope Nb111 of Niobium
    Nb111,
    /// Isotope Nb112 of Niobium
    Nb112,
    /// Isotope Nb113 of Niobium
    Nb113,
    /// Isotope Nb114 of Niobium
    Nb114,
    /// Isotope Nb115 of Niobium
    Nb115,
}
impl super::RelativeAtomicMass for NiobiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Nb81 => 80.9496f64,
            Self::Nb82 => 81.94396f64,
            Self::Nb83 => 82.93729f64,
            Self::Nb84 => 83.93449f64,
            Self::Nb85 => 84.9288458f64,
            Self::Nb86 => 85.9257828f64,
            Self::Nb87 => 86.9206937f64,
            Self::Nb88 => 87.918222f64,
            Self::Nb89 => 88.913445f64,
            Self::Nb90 => 89.9112584f64,
            Self::Nb91 => 90.9069897f64,
            Self::Nb92 => 91.9071881f64,
            Self::Nb93 => 92.906373f64,
            Self::Nb94 => 93.9072788f64,
            Self::Nb95 => 94.9068324f64,
            Self::Nb96 => 95.9080973f64,
            Self::Nb97 => 96.9080959f64,
            Self::Nb98 => 97.9103265f64,
            Self::Nb99 => 98.911613f64,
            Self::Nb100 => 99.9143276f64,
            Self::Nb101 => 100.9153103f64,
            Self::Nb102 => 101.9180772f64,
            Self::Nb103 => 102.9194572f64,
            Self::Nb104 => 103.9228925f64,
            Self::Nb105 => 104.9249465f64,
            Self::Nb106 => 105.9289317f64,
            Self::Nb107 => 106.9315937f64,
            Self::Nb108 => 107.9360748f64,
            Self::Nb109 => 108.93922f64,
            Self::Nb110 => 109.94403f64,
            Self::Nb111 => 110.94753f64,
            Self::Nb112 => 111.95247f64,
            Self::Nb113 => 112.95651f64,
            Self::Nb114 => 113.96201f64,
            Self::Nb115 => 114.96634f64,
        }
    }
}
impl super::ElementVariant for NiobiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Nb
    }
}
impl super::MassNumber for NiobiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Nb81 => 81u16,
            Self::Nb82 => 82u16,
            Self::Nb83 => 83u16,
            Self::Nb84 => 84u16,
            Self::Nb85 => 85u16,
            Self::Nb86 => 86u16,
            Self::Nb87 => 87u16,
            Self::Nb88 => 88u16,
            Self::Nb89 => 89u16,
            Self::Nb90 => 90u16,
            Self::Nb91 => 91u16,
            Self::Nb92 => 92u16,
            Self::Nb93 => 93u16,
            Self::Nb94 => 94u16,
            Self::Nb95 => 95u16,
            Self::Nb96 => 96u16,
            Self::Nb97 => 97u16,
            Self::Nb98 => 98u16,
            Self::Nb99 => 99u16,
            Self::Nb100 => 100u16,
            Self::Nb101 => 101u16,
            Self::Nb102 => 102u16,
            Self::Nb103 => 103u16,
            Self::Nb104 => 104u16,
            Self::Nb105 => 105u16,
            Self::Nb106 => 106u16,
            Self::Nb107 => 107u16,
            Self::Nb108 => 108u16,
            Self::Nb109 => 109u16,
            Self::Nb110 => 110u16,
            Self::Nb111 => 111u16,
            Self::Nb112 => 112u16,
            Self::Nb113 => 113u16,
            Self::Nb114 => 114u16,
            Self::Nb115 => 115u16,
        }
    }
}
impl super::IsotopicComposition for NiobiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Nb93 => Some(1f64),
            Self::Nb81
            | Self::Nb82
            | Self::Nb83
            | Self::Nb84
            | Self::Nb85
            | Self::Nb86
            | Self::Nb87
            | Self::Nb88
            | Self::Nb89
            | Self::Nb90
            | Self::Nb91
            | Self::Nb92
            | Self::Nb94
            | Self::Nb95
            | Self::Nb96
            | Self::Nb97
            | Self::Nb98
            | Self::Nb99
            | Self::Nb100
            | Self::Nb101
            | Self::Nb102
            | Self::Nb103
            | Self::Nb104
            | Self::Nb105
            | Self::Nb106
            | Self::Nb107
            | Self::Nb108
            | Self::Nb109
            | Self::Nb110
            | Self::Nb111
            | Self::Nb112
            | Self::Nb113
            | Self::Nb114
            | Self::Nb115 => None,
        }
    }
}
impl super::MostAbundantIsotope for NiobiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Nb93
    }
}
impl From<NiobiumIsotope> for crate::Isotope {
    fn from(isotope: NiobiumIsotope) -> Self {
        crate::Isotope::Nb(isotope)
    }
}
impl From<NiobiumIsotope> for crate::Element {
    fn from(_isotope: NiobiumIsotope) -> Self {
        crate::Element::Nb
    }
}
impl TryFrom<u16> for NiobiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            81u16 => Ok(Self::Nb81),
            82u16 => Ok(Self::Nb82),
            83u16 => Ok(Self::Nb83),
            84u16 => Ok(Self::Nb84),
            85u16 => Ok(Self::Nb85),
            86u16 => Ok(Self::Nb86),
            87u16 => Ok(Self::Nb87),
            88u16 => Ok(Self::Nb88),
            89u16 => Ok(Self::Nb89),
            90u16 => Ok(Self::Nb90),
            91u16 => Ok(Self::Nb91),
            92u16 => Ok(Self::Nb92),
            93u16 => Ok(Self::Nb93),
            94u16 => Ok(Self::Nb94),
            95u16 => Ok(Self::Nb95),
            96u16 => Ok(Self::Nb96),
            97u16 => Ok(Self::Nb97),
            98u16 => Ok(Self::Nb98),
            99u16 => Ok(Self::Nb99),
            100u16 => Ok(Self::Nb100),
            101u16 => Ok(Self::Nb101),
            102u16 => Ok(Self::Nb102),
            103u16 => Ok(Self::Nb103),
            104u16 => Ok(Self::Nb104),
            105u16 => Ok(Self::Nb105),
            106u16 => Ok(Self::Nb106),
            107u16 => Ok(Self::Nb107),
            108u16 => Ok(Self::Nb108),
            109u16 => Ok(Self::Nb109),
            110u16 => Ok(Self::Nb110),
            111u16 => Ok(Self::Nb111),
            112u16 => Ok(Self::Nb112),
            113u16 => Ok(Self::Nb113),
            114u16 => Ok(Self::Nb114),
            115u16 => Ok(Self::Nb115),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Nb, value)),
        }
    }
}
impl std::fmt::Display for NiobiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nb81 => write!(f, "Nb81"),
            Self::Nb82 => write!(f, "Nb82"),
            Self::Nb83 => write!(f, "Nb83"),
            Self::Nb84 => write!(f, "Nb84"),
            Self::Nb85 => write!(f, "Nb85"),
            Self::Nb86 => write!(f, "Nb86"),
            Self::Nb87 => write!(f, "Nb87"),
            Self::Nb88 => write!(f, "Nb88"),
            Self::Nb89 => write!(f, "Nb89"),
            Self::Nb90 => write!(f, "Nb90"),
            Self::Nb91 => write!(f, "Nb91"),
            Self::Nb92 => write!(f, "Nb92"),
            Self::Nb93 => write!(f, "Nb93"),
            Self::Nb94 => write!(f, "Nb94"),
            Self::Nb95 => write!(f, "Nb95"),
            Self::Nb96 => write!(f, "Nb96"),
            Self::Nb97 => write!(f, "Nb97"),
            Self::Nb98 => write!(f, "Nb98"),
            Self::Nb99 => write!(f, "Nb99"),
            Self::Nb100 => write!(f, "Nb100"),
            Self::Nb101 => write!(f, "Nb101"),
            Self::Nb102 => write!(f, "Nb102"),
            Self::Nb103 => write!(f, "Nb103"),
            Self::Nb104 => write!(f, "Nb104"),
            Self::Nb105 => write!(f, "Nb105"),
            Self::Nb106 => write!(f, "Nb106"),
            Self::Nb107 => write!(f, "Nb107"),
            Self::Nb108 => write!(f, "Nb108"),
            Self::Nb109 => write!(f, "Nb109"),
            Self::Nb110 => write!(f, "Nb110"),
            Self::Nb111 => write!(f, "Nb111"),
            Self::Nb112 => write!(f, "Nb112"),
            Self::Nb113 => write!(f, "Nb113"),
            Self::Nb114 => write!(f, "Nb114"),
            Self::Nb115 => write!(f, "Nb115"),
        }
    }
}
