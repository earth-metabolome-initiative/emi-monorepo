//! Isotopes of the element Ruthenium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Ruthenium
pub enum RutheniumIsotope {
    /// Isotope Ru87 of Ruthenium
    Ru87,
    /// Isotope Ru88 of Ruthenium
    Ru88,
    /// Isotope Ru89 of Ruthenium
    Ru89,
    /// Isotope Ru90 of Ruthenium
    Ru90,
    /// Isotope Ru91 of Ruthenium
    Ru91,
    /// Isotope Ru92 of Ruthenium
    Ru92,
    /// Isotope Ru93 of Ruthenium
    Ru93,
    /// Isotope Ru94 of Ruthenium
    Ru94,
    /// Isotope Ru95 of Ruthenium
    Ru95,
    /// Isotope Ru96 of Ruthenium
    Ru96,
    /// Isotope Ru97 of Ruthenium
    Ru97,
    /// Isotope Ru98 of Ruthenium
    Ru98,
    /// Isotope Ru99 of Ruthenium
    Ru99,
    /// Isotope Ru100 of Ruthenium
    Ru100,
    /// Isotope Ru101 of Ruthenium
    Ru101,
    /// Isotope Ru102 of Ruthenium
    Ru102,
    /// Isotope Ru103 of Ruthenium
    Ru103,
    /// Isotope Ru104 of Ruthenium
    Ru104,
    /// Isotope Ru105 of Ruthenium
    Ru105,
    /// Isotope Ru106 of Ruthenium
    Ru106,
    /// Isotope Ru107 of Ruthenium
    Ru107,
    /// Isotope Ru108 of Ruthenium
    Ru108,
    /// Isotope Ru109 of Ruthenium
    Ru109,
    /// Isotope Ru110 of Ruthenium
    Ru110,
    /// Isotope Ru111 of Ruthenium
    Ru111,
    /// Isotope Ru112 of Ruthenium
    Ru112,
    /// Isotope Ru113 of Ruthenium
    Ru113,
    /// Isotope Ru114 of Ruthenium
    Ru114,
    /// Isotope Ru115 of Ruthenium
    Ru115,
    /// Isotope Ru116 of Ruthenium
    Ru116,
    /// Isotope Ru117 of Ruthenium
    Ru117,
    /// Isotope Ru118 of Ruthenium
    Ru118,
    /// Isotope Ru119 of Ruthenium
    Ru119,
    /// Isotope Ru120 of Ruthenium
    Ru120,
    /// Isotope Ru121 of Ruthenium
    Ru121,
    /// Isotope Ru122 of Ruthenium
    Ru122,
    /// Isotope Ru123 of Ruthenium
    Ru123,
    /// Isotope Ru124 of Ruthenium
    Ru124,
}
impl super::RelativeAtomicMass for RutheniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ru87 => 86.95069f64,
            Self::Ru88 => 87.9416f64,
            Self::Ru89 => 88.93762f64,
            Self::Ru90 => 89.9303444f64,
            Self::Ru91 => 90.9267419f64,
            Self::Ru92 => 91.9202344f64,
            Self::Ru93 => 92.9171044f64,
            Self::Ru94 => 93.9113429f64,
            Self::Ru95 => 94.910406f64,
            Self::Ru96 => 95.90759025f64,
            Self::Ru97 => 96.9075471f64,
            Self::Ru98 => 97.9052868f64,
            Self::Ru99 => 98.9059341f64,
            Self::Ru100 => 99.9042143f64,
            Self::Ru101 => 100.9055769f64,
            Self::Ru102 => 101.9043441f64,
            Self::Ru103 => 102.9063186f64,
            Self::Ru104 => 103.9054275f64,
            Self::Ru105 => 104.9077476f64,
            Self::Ru106 => 105.9073291f64,
            Self::Ru107 => 106.909972f64,
            Self::Ru108 => 107.910188f64,
            Self::Ru109 => 108.913326f64,
            Self::Ru110 => 109.9140407f64,
            Self::Ru111 => 110.91757f64,
            Self::Ru112 => 111.918809f64,
            Self::Ru113 => 112.922844f64,
            Self::Ru114 => 113.9246136f64,
            Self::Ru115 => 114.92882f64,
            Self::Ru116 => 115.9312192f64,
            Self::Ru117 => 116.9361f64,
            Self::Ru118 => 117.93853f64,
            Self::Ru119 => 118.94357f64,
            Self::Ru120 => 119.94631f64,
            Self::Ru121 => 120.95164f64,
            Self::Ru122 => 121.95447f64,
            Self::Ru123 => 122.95989f64,
            Self::Ru124 => 123.96305f64,
        }
    }
}
impl super::ElementVariant for RutheniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ru
    }
}
impl super::MassNumber for RutheniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ru87 => 87u16,
            Self::Ru88 => 88u16,
            Self::Ru89 => 89u16,
            Self::Ru90 => 90u16,
            Self::Ru91 => 91u16,
            Self::Ru92 => 92u16,
            Self::Ru93 => 93u16,
            Self::Ru94 => 94u16,
            Self::Ru95 => 95u16,
            Self::Ru96 => 96u16,
            Self::Ru97 => 97u16,
            Self::Ru98 => 98u16,
            Self::Ru99 => 99u16,
            Self::Ru100 => 100u16,
            Self::Ru101 => 101u16,
            Self::Ru102 => 102u16,
            Self::Ru103 => 103u16,
            Self::Ru104 => 104u16,
            Self::Ru105 => 105u16,
            Self::Ru106 => 106u16,
            Self::Ru107 => 107u16,
            Self::Ru108 => 108u16,
            Self::Ru109 => 109u16,
            Self::Ru110 => 110u16,
            Self::Ru111 => 111u16,
            Self::Ru112 => 112u16,
            Self::Ru113 => 113u16,
            Self::Ru114 => 114u16,
            Self::Ru115 => 115u16,
            Self::Ru116 => 116u16,
            Self::Ru117 => 117u16,
            Self::Ru118 => 118u16,
            Self::Ru119 => 119u16,
            Self::Ru120 => 120u16,
            Self::Ru121 => 121u16,
            Self::Ru122 => 122u16,
            Self::Ru123 => 123u16,
            Self::Ru124 => 124u16,
        }
    }
}
impl super::IsotopicComposition for RutheniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ru96 => Some(0.0554f64),
            Self::Ru98 => Some(0.0187f64),
            Self::Ru99 => Some(0.1276f64),
            Self::Ru100 => Some(0.126f64),
            Self::Ru101 => Some(0.1706f64),
            Self::Ru102 => Some(0.3155f64),
            Self::Ru104 => Some(0.1862f64),
            Self::Ru87
            | Self::Ru88
            | Self::Ru89
            | Self::Ru90
            | Self::Ru91
            | Self::Ru92
            | Self::Ru93
            | Self::Ru94
            | Self::Ru95
            | Self::Ru97
            | Self::Ru103
            | Self::Ru105
            | Self::Ru106
            | Self::Ru107
            | Self::Ru108
            | Self::Ru109
            | Self::Ru110
            | Self::Ru111
            | Self::Ru112
            | Self::Ru113
            | Self::Ru114
            | Self::Ru115
            | Self::Ru116
            | Self::Ru117
            | Self::Ru118
            | Self::Ru119
            | Self::Ru120
            | Self::Ru121
            | Self::Ru122
            | Self::Ru123
            | Self::Ru124 => None,
        }
    }
}
impl super::MostAbundantIsotope for RutheniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ru102
    }
}
impl TryFrom<u16> for RutheniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            87u16 => Ok(Self::Ru87),
            88u16 => Ok(Self::Ru88),
            89u16 => Ok(Self::Ru89),
            90u16 => Ok(Self::Ru90),
            91u16 => Ok(Self::Ru91),
            92u16 => Ok(Self::Ru92),
            93u16 => Ok(Self::Ru93),
            94u16 => Ok(Self::Ru94),
            95u16 => Ok(Self::Ru95),
            96u16 => Ok(Self::Ru96),
            97u16 => Ok(Self::Ru97),
            98u16 => Ok(Self::Ru98),
            99u16 => Ok(Self::Ru99),
            100u16 => Ok(Self::Ru100),
            101u16 => Ok(Self::Ru101),
            102u16 => Ok(Self::Ru102),
            103u16 => Ok(Self::Ru103),
            104u16 => Ok(Self::Ru104),
            105u16 => Ok(Self::Ru105),
            106u16 => Ok(Self::Ru106),
            107u16 => Ok(Self::Ru107),
            108u16 => Ok(Self::Ru108),
            109u16 => Ok(Self::Ru109),
            110u16 => Ok(Self::Ru110),
            111u16 => Ok(Self::Ru111),
            112u16 => Ok(Self::Ru112),
            113u16 => Ok(Self::Ru113),
            114u16 => Ok(Self::Ru114),
            115u16 => Ok(Self::Ru115),
            116u16 => Ok(Self::Ru116),
            117u16 => Ok(Self::Ru117),
            118u16 => Ok(Self::Ru118),
            119u16 => Ok(Self::Ru119),
            120u16 => Ok(Self::Ru120),
            121u16 => Ok(Self::Ru121),
            122u16 => Ok(Self::Ru122),
            123u16 => Ok(Self::Ru123),
            124u16 => Ok(Self::Ru124),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ru, value)),
        }
    }
}
impl std::fmt::Display for RutheniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ru87 => write!(f, "Ru87"),
            Self::Ru88 => write!(f, "Ru88"),
            Self::Ru89 => write!(f, "Ru89"),
            Self::Ru90 => write!(f, "Ru90"),
            Self::Ru91 => write!(f, "Ru91"),
            Self::Ru92 => write!(f, "Ru92"),
            Self::Ru93 => write!(f, "Ru93"),
            Self::Ru94 => write!(f, "Ru94"),
            Self::Ru95 => write!(f, "Ru95"),
            Self::Ru96 => write!(f, "Ru96"),
            Self::Ru97 => write!(f, "Ru97"),
            Self::Ru98 => write!(f, "Ru98"),
            Self::Ru99 => write!(f, "Ru99"),
            Self::Ru100 => write!(f, "Ru100"),
            Self::Ru101 => write!(f, "Ru101"),
            Self::Ru102 => write!(f, "Ru102"),
            Self::Ru103 => write!(f, "Ru103"),
            Self::Ru104 => write!(f, "Ru104"),
            Self::Ru105 => write!(f, "Ru105"),
            Self::Ru106 => write!(f, "Ru106"),
            Self::Ru107 => write!(f, "Ru107"),
            Self::Ru108 => write!(f, "Ru108"),
            Self::Ru109 => write!(f, "Ru109"),
            Self::Ru110 => write!(f, "Ru110"),
            Self::Ru111 => write!(f, "Ru111"),
            Self::Ru112 => write!(f, "Ru112"),
            Self::Ru113 => write!(f, "Ru113"),
            Self::Ru114 => write!(f, "Ru114"),
            Self::Ru115 => write!(f, "Ru115"),
            Self::Ru116 => write!(f, "Ru116"),
            Self::Ru117 => write!(f, "Ru117"),
            Self::Ru118 => write!(f, "Ru118"),
            Self::Ru119 => write!(f, "Ru119"),
            Self::Ru120 => write!(f, "Ru120"),
            Self::Ru121 => write!(f, "Ru121"),
            Self::Ru122 => write!(f, "Ru122"),
            Self::Ru123 => write!(f, "Ru123"),
            Self::Ru124 => write!(f, "Ru124"),
        }
    }
}
