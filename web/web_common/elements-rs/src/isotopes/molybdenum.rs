//! Isotopes of the element Molybdenum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Molybdenum
pub enum MolybdenumIsotope {
    /// Isotope Mo83 of Molybdenum
    Mo83,
    /// Isotope Mo84 of Molybdenum
    Mo84,
    /// Isotope Mo85 of Molybdenum
    Mo85,
    /// Isotope Mo86 of Molybdenum
    Mo86,
    /// Isotope Mo87 of Molybdenum
    Mo87,
    /// Isotope Mo88 of Molybdenum
    Mo88,
    /// Isotope Mo89 of Molybdenum
    Mo89,
    /// Isotope Mo90 of Molybdenum
    Mo90,
    /// Isotope Mo91 of Molybdenum
    Mo91,
    /// Isotope Mo92 of Molybdenum
    Mo92,
    /// Isotope Mo93 of Molybdenum
    Mo93,
    /// Isotope Mo94 of Molybdenum
    Mo94,
    /// Isotope Mo95 of Molybdenum
    Mo95,
    /// Isotope Mo96 of Molybdenum
    Mo96,
    /// Isotope Mo97 of Molybdenum
    Mo97,
    /// Isotope Mo98 of Molybdenum
    Mo98,
    /// Isotope Mo99 of Molybdenum
    Mo99,
    /// Isotope Mo100 of Molybdenum
    Mo100,
    /// Isotope Mo101 of Molybdenum
    Mo101,
    /// Isotope Mo102 of Molybdenum
    Mo102,
    /// Isotope Mo103 of Molybdenum
    Mo103,
    /// Isotope Mo104 of Molybdenum
    Mo104,
    /// Isotope Mo105 of Molybdenum
    Mo105,
    /// Isotope Mo106 of Molybdenum
    Mo106,
    /// Isotope Mo107 of Molybdenum
    Mo107,
    /// Isotope Mo108 of Molybdenum
    Mo108,
    /// Isotope Mo109 of Molybdenum
    Mo109,
    /// Isotope Mo110 of Molybdenum
    Mo110,
    /// Isotope Mo111 of Molybdenum
    Mo111,
    /// Isotope Mo112 of Molybdenum
    Mo112,
    /// Isotope Mo113 of Molybdenum
    Mo113,
    /// Isotope Mo114 of Molybdenum
    Mo114,
    /// Isotope Mo115 of Molybdenum
    Mo115,
    /// Isotope Mo116 of Molybdenum
    Mo116,
    /// Isotope Mo117 of Molybdenum
    Mo117,
}
impl super::RelativeAtomicMass for MolybdenumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mo83 => 82.94988f64,
            Self::Mo84 => 83.94149f64,
            Self::Mo85 => 84.938261f64,
            Self::Mo86 => 85.9311748f64,
            Self::Mo87 => 86.9281962f64,
            Self::Mo88 => 87.9219678f64,
            Self::Mo89 => 88.9194682f64,
            Self::Mo90 => 89.9139309f64,
            Self::Mo91 => 90.9117453f64,
            Self::Mo92 => 91.90680796f64,
            Self::Mo93 => 92.90680958f64,
            Self::Mo94 => 93.9050849f64,
            Self::Mo95 => 94.90583877f64,
            Self::Mo96 => 95.90467612f64,
            Self::Mo97 => 96.90601812f64,
            Self::Mo98 => 97.90540482f64,
            Self::Mo99 => 98.90770851f64,
            Self::Mo100 => 99.9074718f64,
            Self::Mo101 => 100.9103414f64,
            Self::Mo102 => 101.9102834f64,
            Self::Mo103 => 102.913079f64,
            Self::Mo104 => 103.9137344f64,
            Self::Mo105 => 104.916969f64,
            Self::Mo106 => 105.918259f64,
            Self::Mo107 => 106.922106f64,
            Self::Mo108 => 107.924033f64,
            Self::Mo109 => 108.928424f64,
            Self::Mo110 => 109.930704f64,
            Self::Mo111 => 110.935654f64,
            Self::Mo112 => 111.93831f64,
            Self::Mo113 => 112.94335f64,
            Self::Mo114 => 113.94653f64,
            Self::Mo115 => 114.95196f64,
            Self::Mo116 => 115.95545f64,
            Self::Mo117 => 116.96117f64,
        }
    }
}
impl super::ElementVariant for MolybdenumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mo
    }
}
impl super::MassNumber for MolybdenumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mo83 => 83u16,
            Self::Mo84 => 84u16,
            Self::Mo85 => 85u16,
            Self::Mo86 => 86u16,
            Self::Mo87 => 87u16,
            Self::Mo88 => 88u16,
            Self::Mo89 => 89u16,
            Self::Mo90 => 90u16,
            Self::Mo91 => 91u16,
            Self::Mo92 => 92u16,
            Self::Mo93 => 93u16,
            Self::Mo94 => 94u16,
            Self::Mo95 => 95u16,
            Self::Mo96 => 96u16,
            Self::Mo97 => 97u16,
            Self::Mo98 => 98u16,
            Self::Mo99 => 99u16,
            Self::Mo100 => 100u16,
            Self::Mo101 => 101u16,
            Self::Mo102 => 102u16,
            Self::Mo103 => 103u16,
            Self::Mo104 => 104u16,
            Self::Mo105 => 105u16,
            Self::Mo106 => 106u16,
            Self::Mo107 => 107u16,
            Self::Mo108 => 108u16,
            Self::Mo109 => 109u16,
            Self::Mo110 => 110u16,
            Self::Mo111 => 111u16,
            Self::Mo112 => 112u16,
            Self::Mo113 => 113u16,
            Self::Mo114 => 114u16,
            Self::Mo115 => 115u16,
            Self::Mo116 => 116u16,
            Self::Mo117 => 117u16,
        }
    }
}
impl super::IsotopicComposition for MolybdenumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Mo92 => Some(0.1453f64),
            Self::Mo94 => Some(0.0915f64),
            Self::Mo95 => Some(0.1584f64),
            Self::Mo96 => Some(0.1667f64),
            Self::Mo97 => Some(0.096f64),
            Self::Mo98 => Some(0.2439f64),
            Self::Mo100 => Some(0.0982f64),
            Self::Mo83
            | Self::Mo84
            | Self::Mo85
            | Self::Mo86
            | Self::Mo87
            | Self::Mo88
            | Self::Mo89
            | Self::Mo90
            | Self::Mo91
            | Self::Mo93
            | Self::Mo99
            | Self::Mo101
            | Self::Mo102
            | Self::Mo103
            | Self::Mo104
            | Self::Mo105
            | Self::Mo106
            | Self::Mo107
            | Self::Mo108
            | Self::Mo109
            | Self::Mo110
            | Self::Mo111
            | Self::Mo112
            | Self::Mo113
            | Self::Mo114
            | Self::Mo115
            | Self::Mo116
            | Self::Mo117 => None,
        }
    }
}
impl super::MostAbundantIsotope for MolybdenumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Mo98
    }
}
impl From<MolybdenumIsotope> for crate::Isotope {
    fn from(isotope: MolybdenumIsotope) -> Self {
        crate::Isotope::Mo(isotope)
    }
}
impl From<MolybdenumIsotope> for crate::Element {
    fn from(_isotope: MolybdenumIsotope) -> Self {
        crate::Element::Mo
    }
}
impl TryFrom<u16> for MolybdenumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            83u16 => Ok(Self::Mo83),
            84u16 => Ok(Self::Mo84),
            85u16 => Ok(Self::Mo85),
            86u16 => Ok(Self::Mo86),
            87u16 => Ok(Self::Mo87),
            88u16 => Ok(Self::Mo88),
            89u16 => Ok(Self::Mo89),
            90u16 => Ok(Self::Mo90),
            91u16 => Ok(Self::Mo91),
            92u16 => Ok(Self::Mo92),
            93u16 => Ok(Self::Mo93),
            94u16 => Ok(Self::Mo94),
            95u16 => Ok(Self::Mo95),
            96u16 => Ok(Self::Mo96),
            97u16 => Ok(Self::Mo97),
            98u16 => Ok(Self::Mo98),
            99u16 => Ok(Self::Mo99),
            100u16 => Ok(Self::Mo100),
            101u16 => Ok(Self::Mo101),
            102u16 => Ok(Self::Mo102),
            103u16 => Ok(Self::Mo103),
            104u16 => Ok(Self::Mo104),
            105u16 => Ok(Self::Mo105),
            106u16 => Ok(Self::Mo106),
            107u16 => Ok(Self::Mo107),
            108u16 => Ok(Self::Mo108),
            109u16 => Ok(Self::Mo109),
            110u16 => Ok(Self::Mo110),
            111u16 => Ok(Self::Mo111),
            112u16 => Ok(Self::Mo112),
            113u16 => Ok(Self::Mo113),
            114u16 => Ok(Self::Mo114),
            115u16 => Ok(Self::Mo115),
            116u16 => Ok(Self::Mo116),
            117u16 => Ok(Self::Mo117),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Mo, value)),
        }
    }
}
impl std::fmt::Display for MolybdenumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mo83 => write!(f, "Mo83"),
            Self::Mo84 => write!(f, "Mo84"),
            Self::Mo85 => write!(f, "Mo85"),
            Self::Mo86 => write!(f, "Mo86"),
            Self::Mo87 => write!(f, "Mo87"),
            Self::Mo88 => write!(f, "Mo88"),
            Self::Mo89 => write!(f, "Mo89"),
            Self::Mo90 => write!(f, "Mo90"),
            Self::Mo91 => write!(f, "Mo91"),
            Self::Mo92 => write!(f, "Mo92"),
            Self::Mo93 => write!(f, "Mo93"),
            Self::Mo94 => write!(f, "Mo94"),
            Self::Mo95 => write!(f, "Mo95"),
            Self::Mo96 => write!(f, "Mo96"),
            Self::Mo97 => write!(f, "Mo97"),
            Self::Mo98 => write!(f, "Mo98"),
            Self::Mo99 => write!(f, "Mo99"),
            Self::Mo100 => write!(f, "Mo100"),
            Self::Mo101 => write!(f, "Mo101"),
            Self::Mo102 => write!(f, "Mo102"),
            Self::Mo103 => write!(f, "Mo103"),
            Self::Mo104 => write!(f, "Mo104"),
            Self::Mo105 => write!(f, "Mo105"),
            Self::Mo106 => write!(f, "Mo106"),
            Self::Mo107 => write!(f, "Mo107"),
            Self::Mo108 => write!(f, "Mo108"),
            Self::Mo109 => write!(f, "Mo109"),
            Self::Mo110 => write!(f, "Mo110"),
            Self::Mo111 => write!(f, "Mo111"),
            Self::Mo112 => write!(f, "Mo112"),
            Self::Mo113 => write!(f, "Mo113"),
            Self::Mo114 => write!(f, "Mo114"),
            Self::Mo115 => write!(f, "Mo115"),
            Self::Mo116 => write!(f, "Mo116"),
            Self::Mo117 => write!(f, "Mo117"),
        }
    }
}
