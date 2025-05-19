//! Isotopes of the element Tin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Tin
pub enum TinIsotope {
    /// Isotope Sn99 of Tin
    Sn99,
    /// Isotope Sn100 of Tin
    Sn100,
    /// Isotope Sn101 of Tin
    Sn101,
    /// Isotope Sn102 of Tin
    Sn102,
    /// Isotope Sn103 of Tin
    Sn103,
    /// Isotope Sn104 of Tin
    Sn104,
    /// Isotope Sn105 of Tin
    Sn105,
    /// Isotope Sn106 of Tin
    Sn106,
    /// Isotope Sn107 of Tin
    Sn107,
    /// Isotope Sn108 of Tin
    Sn108,
    /// Isotope Sn109 of Tin
    Sn109,
    /// Isotope Sn110 of Tin
    Sn110,
    /// Isotope Sn111 of Tin
    Sn111,
    /// Isotope Sn112 of Tin
    Sn112,
    /// Isotope Sn113 of Tin
    Sn113,
    /// Isotope Sn114 of Tin
    Sn114,
    /// Isotope Sn115 of Tin
    Sn115,
    /// Isotope Sn116 of Tin
    Sn116,
    /// Isotope Sn117 of Tin
    Sn117,
    /// Isotope Sn118 of Tin
    Sn118,
    /// Isotope Sn119 of Tin
    Sn119,
    /// Isotope Sn120 of Tin
    Sn120,
    /// Isotope Sn121 of Tin
    Sn121,
    /// Isotope Sn122 of Tin
    Sn122,
    /// Isotope Sn123 of Tin
    Sn123,
    /// Isotope Sn124 of Tin
    Sn124,
    /// Isotope Sn125 of Tin
    Sn125,
    /// Isotope Sn126 of Tin
    Sn126,
    /// Isotope Sn127 of Tin
    Sn127,
    /// Isotope Sn128 of Tin
    Sn128,
    /// Isotope Sn129 of Tin
    Sn129,
    /// Isotope Sn130 of Tin
    Sn130,
    /// Isotope Sn131 of Tin
    Sn131,
    /// Isotope Sn132 of Tin
    Sn132,
    /// Isotope Sn133 of Tin
    Sn133,
    /// Isotope Sn134 of Tin
    Sn134,
    /// Isotope Sn135 of Tin
    Sn135,
    /// Isotope Sn136 of Tin
    Sn136,
    /// Isotope Sn137 of Tin
    Sn137,
    /// Isotope Sn138 of Tin
    Sn138,
}
impl super::RelativeAtomicMass for TinIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Sn99 => 98.94853f64,
            Self::Sn100 => 99.9385f64,
            Self::Sn101 => 100.93526f64,
            Self::Sn102 => 101.93029f64,
            Self::Sn103 => 102.928105f64,
            Self::Sn104 => 103.9231052f64,
            Self::Sn105 => 104.9212684f64,
            Self::Sn106 => 105.9169574f64,
            Self::Sn107 => 106.9157137f64,
            Self::Sn108 => 107.9118943f64,
            Self::Sn109 => 108.9112921f64,
            Self::Sn110 => 109.907845f64,
            Self::Sn111 => 110.9077401f64,
            Self::Sn112 => 111.90482387f64,
            Self::Sn113 => 112.9051757f64,
            Self::Sn114 => 113.9027827f64,
            Self::Sn115 => 114.903344699f64,
            Self::Sn116 => 115.9017428f64,
            Self::Sn117 => 116.90295398f64,
            Self::Sn118 => 117.90160657f64,
            Self::Sn119 => 118.90331117f64,
            Self::Sn120 => 119.90220163f64,
            Self::Sn121 => 120.9042426f64,
            Self::Sn122 => 121.9034438f64,
            Self::Sn123 => 122.9057252f64,
            Self::Sn124 => 123.9052766f64,
            Self::Sn125 => 124.9077864f64,
            Self::Sn126 => 125.907659f64,
            Self::Sn127 => 126.91039f64,
            Self::Sn128 => 127.910507f64,
            Self::Sn129 => 128.913465f64,
            Self::Sn130 => 129.9139738f64,
            Self::Sn131 => 130.917045f64,
            Self::Sn132 => 131.9178267f64,
            Self::Sn133 => 132.9239134f64,
            Self::Sn134 => 133.9286821f64,
            Self::Sn135 => 134.9349086f64,
            Self::Sn136 => 135.93999f64,
            Self::Sn137 => 136.94655f64,
            Self::Sn138 => 137.95184f64,
        }
    }
}
impl super::ElementVariant for TinIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Sn
    }
}
impl super::MassNumber for TinIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Sn99 => 99u16,
            Self::Sn100 => 100u16,
            Self::Sn101 => 101u16,
            Self::Sn102 => 102u16,
            Self::Sn103 => 103u16,
            Self::Sn104 => 104u16,
            Self::Sn105 => 105u16,
            Self::Sn106 => 106u16,
            Self::Sn107 => 107u16,
            Self::Sn108 => 108u16,
            Self::Sn109 => 109u16,
            Self::Sn110 => 110u16,
            Self::Sn111 => 111u16,
            Self::Sn112 => 112u16,
            Self::Sn113 => 113u16,
            Self::Sn114 => 114u16,
            Self::Sn115 => 115u16,
            Self::Sn116 => 116u16,
            Self::Sn117 => 117u16,
            Self::Sn118 => 118u16,
            Self::Sn119 => 119u16,
            Self::Sn120 => 120u16,
            Self::Sn121 => 121u16,
            Self::Sn122 => 122u16,
            Self::Sn123 => 123u16,
            Self::Sn124 => 124u16,
            Self::Sn125 => 125u16,
            Self::Sn126 => 126u16,
            Self::Sn127 => 127u16,
            Self::Sn128 => 128u16,
            Self::Sn129 => 129u16,
            Self::Sn130 => 130u16,
            Self::Sn131 => 131u16,
            Self::Sn132 => 132u16,
            Self::Sn133 => 133u16,
            Self::Sn134 => 134u16,
            Self::Sn135 => 135u16,
            Self::Sn136 => 136u16,
            Self::Sn137 => 137u16,
            Self::Sn138 => 138u16,
        }
    }
}
impl super::IsotopicComposition for TinIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Sn112 => Some(0.0097f64),
            Self::Sn114 => Some(0.0066f64),
            Self::Sn115 => Some(0.0034f64),
            Self::Sn116 => Some(0.1454f64),
            Self::Sn117 => Some(0.0768f64),
            Self::Sn118 => Some(0.2422f64),
            Self::Sn119 => Some(0.0859f64),
            Self::Sn120 => Some(0.3258f64),
            Self::Sn122 => Some(0.0463f64),
            Self::Sn124 => Some(0.0579f64),
            Self::Sn99
            | Self::Sn100
            | Self::Sn101
            | Self::Sn102
            | Self::Sn103
            | Self::Sn104
            | Self::Sn105
            | Self::Sn106
            | Self::Sn107
            | Self::Sn108
            | Self::Sn109
            | Self::Sn110
            | Self::Sn111
            | Self::Sn113
            | Self::Sn121
            | Self::Sn123
            | Self::Sn125
            | Self::Sn126
            | Self::Sn127
            | Self::Sn128
            | Self::Sn129
            | Self::Sn130
            | Self::Sn131
            | Self::Sn132
            | Self::Sn133
            | Self::Sn134
            | Self::Sn135
            | Self::Sn136
            | Self::Sn137
            | Self::Sn138 => None,
        }
    }
}
impl super::MostAbundantIsotope for TinIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Sn120
    }
}
impl TryFrom<u16> for TinIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            99u16 => Ok(Self::Sn99),
            100u16 => Ok(Self::Sn100),
            101u16 => Ok(Self::Sn101),
            102u16 => Ok(Self::Sn102),
            103u16 => Ok(Self::Sn103),
            104u16 => Ok(Self::Sn104),
            105u16 => Ok(Self::Sn105),
            106u16 => Ok(Self::Sn106),
            107u16 => Ok(Self::Sn107),
            108u16 => Ok(Self::Sn108),
            109u16 => Ok(Self::Sn109),
            110u16 => Ok(Self::Sn110),
            111u16 => Ok(Self::Sn111),
            112u16 => Ok(Self::Sn112),
            113u16 => Ok(Self::Sn113),
            114u16 => Ok(Self::Sn114),
            115u16 => Ok(Self::Sn115),
            116u16 => Ok(Self::Sn116),
            117u16 => Ok(Self::Sn117),
            118u16 => Ok(Self::Sn118),
            119u16 => Ok(Self::Sn119),
            120u16 => Ok(Self::Sn120),
            121u16 => Ok(Self::Sn121),
            122u16 => Ok(Self::Sn122),
            123u16 => Ok(Self::Sn123),
            124u16 => Ok(Self::Sn124),
            125u16 => Ok(Self::Sn125),
            126u16 => Ok(Self::Sn126),
            127u16 => Ok(Self::Sn127),
            128u16 => Ok(Self::Sn128),
            129u16 => Ok(Self::Sn129),
            130u16 => Ok(Self::Sn130),
            131u16 => Ok(Self::Sn131),
            132u16 => Ok(Self::Sn132),
            133u16 => Ok(Self::Sn133),
            134u16 => Ok(Self::Sn134),
            135u16 => Ok(Self::Sn135),
            136u16 => Ok(Self::Sn136),
            137u16 => Ok(Self::Sn137),
            138u16 => Ok(Self::Sn138),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Sn, value)),
        }
    }
}
impl std::fmt::Display for TinIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sn99 => write!(f, "Sn99"),
            Self::Sn100 => write!(f, "Sn100"),
            Self::Sn101 => write!(f, "Sn101"),
            Self::Sn102 => write!(f, "Sn102"),
            Self::Sn103 => write!(f, "Sn103"),
            Self::Sn104 => write!(f, "Sn104"),
            Self::Sn105 => write!(f, "Sn105"),
            Self::Sn106 => write!(f, "Sn106"),
            Self::Sn107 => write!(f, "Sn107"),
            Self::Sn108 => write!(f, "Sn108"),
            Self::Sn109 => write!(f, "Sn109"),
            Self::Sn110 => write!(f, "Sn110"),
            Self::Sn111 => write!(f, "Sn111"),
            Self::Sn112 => write!(f, "Sn112"),
            Self::Sn113 => write!(f, "Sn113"),
            Self::Sn114 => write!(f, "Sn114"),
            Self::Sn115 => write!(f, "Sn115"),
            Self::Sn116 => write!(f, "Sn116"),
            Self::Sn117 => write!(f, "Sn117"),
            Self::Sn118 => write!(f, "Sn118"),
            Self::Sn119 => write!(f, "Sn119"),
            Self::Sn120 => write!(f, "Sn120"),
            Self::Sn121 => write!(f, "Sn121"),
            Self::Sn122 => write!(f, "Sn122"),
            Self::Sn123 => write!(f, "Sn123"),
            Self::Sn124 => write!(f, "Sn124"),
            Self::Sn125 => write!(f, "Sn125"),
            Self::Sn126 => write!(f, "Sn126"),
            Self::Sn127 => write!(f, "Sn127"),
            Self::Sn128 => write!(f, "Sn128"),
            Self::Sn129 => write!(f, "Sn129"),
            Self::Sn130 => write!(f, "Sn130"),
            Self::Sn131 => write!(f, "Sn131"),
            Self::Sn132 => write!(f, "Sn132"),
            Self::Sn133 => write!(f, "Sn133"),
            Self::Sn134 => write!(f, "Sn134"),
            Self::Sn135 => write!(f, "Sn135"),
            Self::Sn136 => write!(f, "Sn136"),
            Self::Sn137 => write!(f, "Sn137"),
            Self::Sn138 => write!(f, "Sn138"),
        }
    }
}
