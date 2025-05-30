//! Isotopes of the element Zirconium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Zirconium
pub enum ZirconiumIsotope {
    /// Isotope Zr78 of Zirconium
    Zr78,
    /// Isotope Zr79 of Zirconium
    Zr79,
    /// Isotope Zr80 of Zirconium
    Zr80,
    /// Isotope Zr81 of Zirconium
    Zr81,
    /// Isotope Zr82 of Zirconium
    Zr82,
    /// Isotope Zr83 of Zirconium
    Zr83,
    /// Isotope Zr84 of Zirconium
    Zr84,
    /// Isotope Zr85 of Zirconium
    Zr85,
    /// Isotope Zr86 of Zirconium
    Zr86,
    /// Isotope Zr87 of Zirconium
    Zr87,
    /// Isotope Zr88 of Zirconium
    Zr88,
    /// Isotope Zr89 of Zirconium
    Zr89,
    /// Isotope Zr90 of Zirconium
    Zr90,
    /// Isotope Zr91 of Zirconium
    Zr91,
    /// Isotope Zr92 of Zirconium
    Zr92,
    /// Isotope Zr93 of Zirconium
    Zr93,
    /// Isotope Zr94 of Zirconium
    Zr94,
    /// Isotope Zr95 of Zirconium
    Zr95,
    /// Isotope Zr96 of Zirconium
    Zr96,
    /// Isotope Zr97 of Zirconium
    Zr97,
    /// Isotope Zr98 of Zirconium
    Zr98,
    /// Isotope Zr99 of Zirconium
    Zr99,
    /// Isotope Zr100 of Zirconium
    Zr100,
    /// Isotope Zr101 of Zirconium
    Zr101,
    /// Isotope Zr102 of Zirconium
    Zr102,
    /// Isotope Zr103 of Zirconium
    Zr103,
    /// Isotope Zr104 of Zirconium
    Zr104,
    /// Isotope Zr105 of Zirconium
    Zr105,
    /// Isotope Zr106 of Zirconium
    Zr106,
    /// Isotope Zr107 of Zirconium
    Zr107,
    /// Isotope Zr108 of Zirconium
    Zr108,
    /// Isotope Zr109 of Zirconium
    Zr109,
    /// Isotope Zr110 of Zirconium
    Zr110,
    /// Isotope Zr111 of Zirconium
    Zr111,
    /// Isotope Zr112 of Zirconium
    Zr112,
}
impl super::RelativeAtomicMass for ZirconiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Zr78 => 77.95566f64,
            Self::Zr79 => 78.94948f64,
            Self::Zr80 => 79.9404f64,
            Self::Zr81 => 80.93731f64,
            Self::Zr82 => 81.93135f64,
            Self::Zr83 => 82.9292421f64,
            Self::Zr84 => 83.9233269f64,
            Self::Zr85 => 84.9214444f64,
            Self::Zr86 => 85.9162972f64,
            Self::Zr87 => 86.914818f64,
            Self::Zr88 => 87.9102213f64,
            Self::Zr89 => 88.9088814f64,
            Self::Zr90 => 89.9046977f64,
            Self::Zr91 => 90.9056396f64,
            Self::Zr92 => 91.9050347f64,
            Self::Zr93 => 92.9064699f64,
            Self::Zr94 => 93.9063108f64,
            Self::Zr95 => 94.9080385f64,
            Self::Zr96 => 95.9082714f64,
            Self::Zr97 => 96.9109512f64,
            Self::Zr98 => 97.9127289f64,
            Self::Zr99 => 98.916667f64,
            Self::Zr100 => 99.9180006f64,
            Self::Zr101 => 100.921448f64,
            Self::Zr102 => 101.9231409f64,
            Self::Zr103 => 102.927191f64,
            Self::Zr104 => 103.929436f64,
            Self::Zr105 => 104.934008f64,
            Self::Zr106 => 105.93676f64,
            Self::Zr107 => 106.94174f64,
            Self::Zr108 => 107.94487f64,
            Self::Zr109 => 108.95041f64,
            Self::Zr110 => 109.95396f64,
            Self::Zr111 => 110.95968f64,
            Self::Zr112 => 111.9637f64,
        }
    }
}
impl super::ElementVariant for ZirconiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Zr
    }
}
impl super::MassNumber for ZirconiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Zr78 => 78u16,
            Self::Zr79 => 79u16,
            Self::Zr80 => 80u16,
            Self::Zr81 => 81u16,
            Self::Zr82 => 82u16,
            Self::Zr83 => 83u16,
            Self::Zr84 => 84u16,
            Self::Zr85 => 85u16,
            Self::Zr86 => 86u16,
            Self::Zr87 => 87u16,
            Self::Zr88 => 88u16,
            Self::Zr89 => 89u16,
            Self::Zr90 => 90u16,
            Self::Zr91 => 91u16,
            Self::Zr92 => 92u16,
            Self::Zr93 => 93u16,
            Self::Zr94 => 94u16,
            Self::Zr95 => 95u16,
            Self::Zr96 => 96u16,
            Self::Zr97 => 97u16,
            Self::Zr98 => 98u16,
            Self::Zr99 => 99u16,
            Self::Zr100 => 100u16,
            Self::Zr101 => 101u16,
            Self::Zr102 => 102u16,
            Self::Zr103 => 103u16,
            Self::Zr104 => 104u16,
            Self::Zr105 => 105u16,
            Self::Zr106 => 106u16,
            Self::Zr107 => 107u16,
            Self::Zr108 => 108u16,
            Self::Zr109 => 109u16,
            Self::Zr110 => 110u16,
            Self::Zr111 => 111u16,
            Self::Zr112 => 112u16,
        }
    }
}
impl super::IsotopicComposition for ZirconiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Zr90 => Some(0.5145f64),
            Self::Zr91 => Some(0.1122f64),
            Self::Zr92 => Some(0.1715f64),
            Self::Zr94 => Some(0.1738f64),
            Self::Zr96 => Some(0.028f64),
            Self::Zr78
            | Self::Zr79
            | Self::Zr80
            | Self::Zr81
            | Self::Zr82
            | Self::Zr83
            | Self::Zr84
            | Self::Zr85
            | Self::Zr86
            | Self::Zr87
            | Self::Zr88
            | Self::Zr89
            | Self::Zr93
            | Self::Zr95
            | Self::Zr97
            | Self::Zr98
            | Self::Zr99
            | Self::Zr100
            | Self::Zr101
            | Self::Zr102
            | Self::Zr103
            | Self::Zr104
            | Self::Zr105
            | Self::Zr106
            | Self::Zr107
            | Self::Zr108
            | Self::Zr109
            | Self::Zr110
            | Self::Zr111
            | Self::Zr112 => None,
        }
    }
}
impl super::MostAbundantIsotope for ZirconiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Zr90
    }
}
impl From<ZirconiumIsotope> for crate::Isotope {
    fn from(isotope: ZirconiumIsotope) -> Self {
        crate::Isotope::Zr(isotope)
    }
}
impl From<ZirconiumIsotope> for crate::Element {
    fn from(_isotope: ZirconiumIsotope) -> Self {
        crate::Element::Zr
    }
}
impl TryFrom<u16> for ZirconiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            78u16 => Ok(Self::Zr78),
            79u16 => Ok(Self::Zr79),
            80u16 => Ok(Self::Zr80),
            81u16 => Ok(Self::Zr81),
            82u16 => Ok(Self::Zr82),
            83u16 => Ok(Self::Zr83),
            84u16 => Ok(Self::Zr84),
            85u16 => Ok(Self::Zr85),
            86u16 => Ok(Self::Zr86),
            87u16 => Ok(Self::Zr87),
            88u16 => Ok(Self::Zr88),
            89u16 => Ok(Self::Zr89),
            90u16 => Ok(Self::Zr90),
            91u16 => Ok(Self::Zr91),
            92u16 => Ok(Self::Zr92),
            93u16 => Ok(Self::Zr93),
            94u16 => Ok(Self::Zr94),
            95u16 => Ok(Self::Zr95),
            96u16 => Ok(Self::Zr96),
            97u16 => Ok(Self::Zr97),
            98u16 => Ok(Self::Zr98),
            99u16 => Ok(Self::Zr99),
            100u16 => Ok(Self::Zr100),
            101u16 => Ok(Self::Zr101),
            102u16 => Ok(Self::Zr102),
            103u16 => Ok(Self::Zr103),
            104u16 => Ok(Self::Zr104),
            105u16 => Ok(Self::Zr105),
            106u16 => Ok(Self::Zr106),
            107u16 => Ok(Self::Zr107),
            108u16 => Ok(Self::Zr108),
            109u16 => Ok(Self::Zr109),
            110u16 => Ok(Self::Zr110),
            111u16 => Ok(Self::Zr111),
            112u16 => Ok(Self::Zr112),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Zr, value)),
        }
    }
}
impl std::fmt::Display for ZirconiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zr78 => write!(f, "Zr78"),
            Self::Zr79 => write!(f, "Zr79"),
            Self::Zr80 => write!(f, "Zr80"),
            Self::Zr81 => write!(f, "Zr81"),
            Self::Zr82 => write!(f, "Zr82"),
            Self::Zr83 => write!(f, "Zr83"),
            Self::Zr84 => write!(f, "Zr84"),
            Self::Zr85 => write!(f, "Zr85"),
            Self::Zr86 => write!(f, "Zr86"),
            Self::Zr87 => write!(f, "Zr87"),
            Self::Zr88 => write!(f, "Zr88"),
            Self::Zr89 => write!(f, "Zr89"),
            Self::Zr90 => write!(f, "Zr90"),
            Self::Zr91 => write!(f, "Zr91"),
            Self::Zr92 => write!(f, "Zr92"),
            Self::Zr93 => write!(f, "Zr93"),
            Self::Zr94 => write!(f, "Zr94"),
            Self::Zr95 => write!(f, "Zr95"),
            Self::Zr96 => write!(f, "Zr96"),
            Self::Zr97 => write!(f, "Zr97"),
            Self::Zr98 => write!(f, "Zr98"),
            Self::Zr99 => write!(f, "Zr99"),
            Self::Zr100 => write!(f, "Zr100"),
            Self::Zr101 => write!(f, "Zr101"),
            Self::Zr102 => write!(f, "Zr102"),
            Self::Zr103 => write!(f, "Zr103"),
            Self::Zr104 => write!(f, "Zr104"),
            Self::Zr105 => write!(f, "Zr105"),
            Self::Zr106 => write!(f, "Zr106"),
            Self::Zr107 => write!(f, "Zr107"),
            Self::Zr108 => write!(f, "Zr108"),
            Self::Zr109 => write!(f, "Zr109"),
            Self::Zr110 => write!(f, "Zr110"),
            Self::Zr111 => write!(f, "Zr111"),
            Self::Zr112 => write!(f, "Zr112"),
        }
    }
}
