//! Isotopes of the element Indium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Indium
pub enum IndiumIsotope {
    /// Isotope In97 of Indium
    In97,
    /// Isotope In98 of Indium
    In98,
    /// Isotope In99 of Indium
    In99,
    /// Isotope In100 of Indium
    In100,
    /// Isotope In101 of Indium
    In101,
    /// Isotope In102 of Indium
    In102,
    /// Isotope In103 of Indium
    In103,
    /// Isotope In104 of Indium
    In104,
    /// Isotope In105 of Indium
    In105,
    /// Isotope In106 of Indium
    In106,
    /// Isotope In107 of Indium
    In107,
    /// Isotope In108 of Indium
    In108,
    /// Isotope In109 of Indium
    In109,
    /// Isotope In110 of Indium
    In110,
    /// Isotope In111 of Indium
    In111,
    /// Isotope In112 of Indium
    In112,
    /// Isotope In113 of Indium
    In113,
    /// Isotope In114 of Indium
    In114,
    /// Isotope In115 of Indium
    In115,
    /// Isotope In116 of Indium
    In116,
    /// Isotope In117 of Indium
    In117,
    /// Isotope In118 of Indium
    In118,
    /// Isotope In119 of Indium
    In119,
    /// Isotope In120 of Indium
    In120,
    /// Isotope In121 of Indium
    In121,
    /// Isotope In122 of Indium
    In122,
    /// Isotope In123 of Indium
    In123,
    /// Isotope In124 of Indium
    In124,
    /// Isotope In125 of Indium
    In125,
    /// Isotope In126 of Indium
    In126,
    /// Isotope In127 of Indium
    In127,
    /// Isotope In128 of Indium
    In128,
    /// Isotope In129 of Indium
    In129,
    /// Isotope In130 of Indium
    In130,
    /// Isotope In131 of Indium
    In131,
    /// Isotope In132 of Indium
    In132,
    /// Isotope In133 of Indium
    In133,
    /// Isotope In134 of Indium
    In134,
    /// Isotope In135 of Indium
    In135,
}
impl super::RelativeAtomicMass for IndiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::In97 => 96.94934f64,
            Self::In98 => 97.94214f64,
            Self::In99 => 98.93411f64,
            Self::In100 => 99.93096f64,
            Self::In101 => 100.92634f64,
            Self::In102 => 101.9241071f64,
            Self::In103 => 102.9198819f64,
            Self::In104 => 103.9182145f64,
            Self::In105 => 104.914502f64,
            Self::In106 => 105.913464f64,
            Self::In107 => 106.91029f64,
            Self::In108 => 107.9096935f64,
            Self::In109 => 108.9071514f64,
            Self::In110 => 109.90717f64,
            Self::In111 => 110.9051085f64,
            Self::In112 => 111.9055377f64,
            Self::In113 => 112.90406184f64,
            Self::In114 => 113.90491791f64,
            Self::In115 => 114.903878776f64,
            Self::In116 => 115.90525999f64,
            Self::In117 => 116.9045157f64,
            Self::In118 => 117.9063566f64,
            Self::In119 => 118.9058507f64,
            Self::In120 => 119.907967f64,
            Self::In121 => 120.907851f64,
            Self::In122 => 121.910281f64,
            Self::In123 => 122.910434f64,
            Self::In124 => 123.913182f64,
            Self::In125 => 124.913605f64,
            Self::In126 => 125.916507f64,
            Self::In127 => 126.917446f64,
            Self::In128 => 127.9204f64,
            Self::In129 => 128.9218053f64,
            Self::In130 => 129.924977f64,
            Self::In131 => 130.9269715f64,
            Self::In132 => 131.933001f64,
            Self::In133 => 132.93831f64,
            Self::In134 => 133.94454f64,
            Self::In135 => 134.95005f64,
        }
    }
}
impl super::ElementVariant for IndiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::In
    }
}
impl super::MassNumber for IndiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::In97 => 97u16,
            Self::In98 => 98u16,
            Self::In99 => 99u16,
            Self::In100 => 100u16,
            Self::In101 => 101u16,
            Self::In102 => 102u16,
            Self::In103 => 103u16,
            Self::In104 => 104u16,
            Self::In105 => 105u16,
            Self::In106 => 106u16,
            Self::In107 => 107u16,
            Self::In108 => 108u16,
            Self::In109 => 109u16,
            Self::In110 => 110u16,
            Self::In111 => 111u16,
            Self::In112 => 112u16,
            Self::In113 => 113u16,
            Self::In114 => 114u16,
            Self::In115 => 115u16,
            Self::In116 => 116u16,
            Self::In117 => 117u16,
            Self::In118 => 118u16,
            Self::In119 => 119u16,
            Self::In120 => 120u16,
            Self::In121 => 121u16,
            Self::In122 => 122u16,
            Self::In123 => 123u16,
            Self::In124 => 124u16,
            Self::In125 => 125u16,
            Self::In126 => 126u16,
            Self::In127 => 127u16,
            Self::In128 => 128u16,
            Self::In129 => 129u16,
            Self::In130 => 130u16,
            Self::In131 => 131u16,
            Self::In132 => 132u16,
            Self::In133 => 133u16,
            Self::In134 => 134u16,
            Self::In135 => 135u16,
        }
    }
}
impl super::IsotopicComposition for IndiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::In113 => Some(0.0429f64),
            Self::In115 => Some(0.9571f64),
            Self::In97
            | Self::In98
            | Self::In99
            | Self::In100
            | Self::In101
            | Self::In102
            | Self::In103
            | Self::In104
            | Self::In105
            | Self::In106
            | Self::In107
            | Self::In108
            | Self::In109
            | Self::In110
            | Self::In111
            | Self::In112
            | Self::In114
            | Self::In116
            | Self::In117
            | Self::In118
            | Self::In119
            | Self::In120
            | Self::In121
            | Self::In122
            | Self::In123
            | Self::In124
            | Self::In125
            | Self::In126
            | Self::In127
            | Self::In128
            | Self::In129
            | Self::In130
            | Self::In131
            | Self::In132
            | Self::In133
            | Self::In134
            | Self::In135 => None,
        }
    }
}
impl super::MostAbundantIsotope for IndiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::In115
    }
}
impl From<IndiumIsotope> for crate::Isotope {
    fn from(isotope: IndiumIsotope) -> Self {
        crate::Isotope::In(isotope)
    }
}
impl From<IndiumIsotope> for crate::Element {
    fn from(_isotope: IndiumIsotope) -> Self {
        crate::Element::In
    }
}
impl TryFrom<u16> for IndiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            97u16 => Ok(Self::In97),
            98u16 => Ok(Self::In98),
            99u16 => Ok(Self::In99),
            100u16 => Ok(Self::In100),
            101u16 => Ok(Self::In101),
            102u16 => Ok(Self::In102),
            103u16 => Ok(Self::In103),
            104u16 => Ok(Self::In104),
            105u16 => Ok(Self::In105),
            106u16 => Ok(Self::In106),
            107u16 => Ok(Self::In107),
            108u16 => Ok(Self::In108),
            109u16 => Ok(Self::In109),
            110u16 => Ok(Self::In110),
            111u16 => Ok(Self::In111),
            112u16 => Ok(Self::In112),
            113u16 => Ok(Self::In113),
            114u16 => Ok(Self::In114),
            115u16 => Ok(Self::In115),
            116u16 => Ok(Self::In116),
            117u16 => Ok(Self::In117),
            118u16 => Ok(Self::In118),
            119u16 => Ok(Self::In119),
            120u16 => Ok(Self::In120),
            121u16 => Ok(Self::In121),
            122u16 => Ok(Self::In122),
            123u16 => Ok(Self::In123),
            124u16 => Ok(Self::In124),
            125u16 => Ok(Self::In125),
            126u16 => Ok(Self::In126),
            127u16 => Ok(Self::In127),
            128u16 => Ok(Self::In128),
            129u16 => Ok(Self::In129),
            130u16 => Ok(Self::In130),
            131u16 => Ok(Self::In131),
            132u16 => Ok(Self::In132),
            133u16 => Ok(Self::In133),
            134u16 => Ok(Self::In134),
            135u16 => Ok(Self::In135),
            _ => Err(crate::errors::Error::Isotope(crate::Element::In, value)),
        }
    }
}
impl std::fmt::Display for IndiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::In97 => write!(f, "In97"),
            Self::In98 => write!(f, "In98"),
            Self::In99 => write!(f, "In99"),
            Self::In100 => write!(f, "In100"),
            Self::In101 => write!(f, "In101"),
            Self::In102 => write!(f, "In102"),
            Self::In103 => write!(f, "In103"),
            Self::In104 => write!(f, "In104"),
            Self::In105 => write!(f, "In105"),
            Self::In106 => write!(f, "In106"),
            Self::In107 => write!(f, "In107"),
            Self::In108 => write!(f, "In108"),
            Self::In109 => write!(f, "In109"),
            Self::In110 => write!(f, "In110"),
            Self::In111 => write!(f, "In111"),
            Self::In112 => write!(f, "In112"),
            Self::In113 => write!(f, "In113"),
            Self::In114 => write!(f, "In114"),
            Self::In115 => write!(f, "In115"),
            Self::In116 => write!(f, "In116"),
            Self::In117 => write!(f, "In117"),
            Self::In118 => write!(f, "In118"),
            Self::In119 => write!(f, "In119"),
            Self::In120 => write!(f, "In120"),
            Self::In121 => write!(f, "In121"),
            Self::In122 => write!(f, "In122"),
            Self::In123 => write!(f, "In123"),
            Self::In124 => write!(f, "In124"),
            Self::In125 => write!(f, "In125"),
            Self::In126 => write!(f, "In126"),
            Self::In127 => write!(f, "In127"),
            Self::In128 => write!(f, "In128"),
            Self::In129 => write!(f, "In129"),
            Self::In130 => write!(f, "In130"),
            Self::In131 => write!(f, "In131"),
            Self::In132 => write!(f, "In132"),
            Self::In133 => write!(f, "In133"),
            Self::In134 => write!(f, "In134"),
            Self::In135 => write!(f, "In135"),
        }
    }
}
