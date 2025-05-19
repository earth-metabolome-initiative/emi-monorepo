//! Isotopes of the element Iodine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Iodine
pub enum IodineIsotope {
    /// Isotope I107 of Iodine
    I107,
    /// Isotope I108 of Iodine
    I108,
    /// Isotope I109 of Iodine
    I109,
    /// Isotope I110 of Iodine
    I110,
    /// Isotope I111 of Iodine
    I111,
    /// Isotope I112 of Iodine
    I112,
    /// Isotope I113 of Iodine
    I113,
    /// Isotope I114 of Iodine
    I114,
    /// Isotope I115 of Iodine
    I115,
    /// Isotope I116 of Iodine
    I116,
    /// Isotope I117 of Iodine
    I117,
    /// Isotope I118 of Iodine
    I118,
    /// Isotope I119 of Iodine
    I119,
    /// Isotope I120 of Iodine
    I120,
    /// Isotope I121 of Iodine
    I121,
    /// Isotope I122 of Iodine
    I122,
    /// Isotope I123 of Iodine
    I123,
    /// Isotope I124 of Iodine
    I124,
    /// Isotope I125 of Iodine
    I125,
    /// Isotope I126 of Iodine
    I126,
    /// Isotope I127 of Iodine
    I127,
    /// Isotope I128 of Iodine
    I128,
    /// Isotope I129 of Iodine
    I129,
    /// Isotope I130 of Iodine
    I130,
    /// Isotope I131 of Iodine
    I131,
    /// Isotope I132 of Iodine
    I132,
    /// Isotope I133 of Iodine
    I133,
    /// Isotope I134 of Iodine
    I134,
    /// Isotope I135 of Iodine
    I135,
    /// Isotope I136 of Iodine
    I136,
    /// Isotope I137 of Iodine
    I137,
    /// Isotope I138 of Iodine
    I138,
    /// Isotope I139 of Iodine
    I139,
    /// Isotope I140 of Iodine
    I140,
    /// Isotope I141 of Iodine
    I141,
    /// Isotope I142 of Iodine
    I142,
    /// Isotope I143 of Iodine
    I143,
    /// Isotope I144 of Iodine
    I144,
    /// Isotope I145 of Iodine
    I145,
}
impl super::RelativeAtomicMass for IodineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::I107 => 106.94678f64,
            Self::I108 => 107.94348f64,
            Self::I109 => 108.9380853f64,
            Self::I110 => 109.935089f64,
            Self::I111 => 110.9302692f64,
            Self::I112 => 111.928005f64,
            Self::I113 => 112.9236501f64,
            Self::I114 => 113.92185f64,
            Self::I115 => 114.918048f64,
            Self::I116 => 115.91681f64,
            Self::I117 => 116.913648f64,
            Self::I118 => 117.913074f64,
            Self::I119 => 118.910074f64,
            Self::I120 => 119.910087f64,
            Self::I121 => 120.9074051f64,
            Self::I122 => 121.9075888f64,
            Self::I123 => 122.9055885f64,
            Self::I124 => 123.906209f64,
            Self::I125 => 124.9046294f64,
            Self::I126 => 125.9056233f64,
            Self::I127 => 126.9044719f64,
            Self::I128 => 127.9058086f64,
            Self::I129 => 128.9049837f64,
            Self::I130 => 129.9066702f64,
            Self::I131 => 130.9061263f64,
            Self::I132 => 131.9079935f64,
            Self::I133 => 132.907797f64,
            Self::I134 => 133.9097588f64,
            Self::I135 => 134.9100488f64,
            Self::I136 => 135.914604f64,
            Self::I137 => 136.9180282f64,
            Self::I138 => 137.9227264f64,
            Self::I139 => 138.926506f64,
            Self::I140 => 139.93173f64,
            Self::I141 => 140.93569f64,
            Self::I142 => 141.9412f64,
            Self::I143 => 142.94565f64,
            Self::I144 => 143.95139f64,
            Self::I145 => 144.95605f64,
        }
    }
}
impl super::ElementVariant for IodineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::I
    }
}
impl super::MassNumber for IodineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::I107 => 107u16,
            Self::I108 => 108u16,
            Self::I109 => 109u16,
            Self::I110 => 110u16,
            Self::I111 => 111u16,
            Self::I112 => 112u16,
            Self::I113 => 113u16,
            Self::I114 => 114u16,
            Self::I115 => 115u16,
            Self::I116 => 116u16,
            Self::I117 => 117u16,
            Self::I118 => 118u16,
            Self::I119 => 119u16,
            Self::I120 => 120u16,
            Self::I121 => 121u16,
            Self::I122 => 122u16,
            Self::I123 => 123u16,
            Self::I124 => 124u16,
            Self::I125 => 125u16,
            Self::I126 => 126u16,
            Self::I127 => 127u16,
            Self::I128 => 128u16,
            Self::I129 => 129u16,
            Self::I130 => 130u16,
            Self::I131 => 131u16,
            Self::I132 => 132u16,
            Self::I133 => 133u16,
            Self::I134 => 134u16,
            Self::I135 => 135u16,
            Self::I136 => 136u16,
            Self::I137 => 137u16,
            Self::I138 => 138u16,
            Self::I139 => 139u16,
            Self::I140 => 140u16,
            Self::I141 => 141u16,
            Self::I142 => 142u16,
            Self::I143 => 143u16,
            Self::I144 => 144u16,
            Self::I145 => 145u16,
        }
    }
}
impl super::IsotopicComposition for IodineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::I127 => Some(1f64),
            Self::I107
            | Self::I108
            | Self::I109
            | Self::I110
            | Self::I111
            | Self::I112
            | Self::I113
            | Self::I114
            | Self::I115
            | Self::I116
            | Self::I117
            | Self::I118
            | Self::I119
            | Self::I120
            | Self::I121
            | Self::I122
            | Self::I123
            | Self::I124
            | Self::I125
            | Self::I126
            | Self::I128
            | Self::I129
            | Self::I130
            | Self::I131
            | Self::I132
            | Self::I133
            | Self::I134
            | Self::I135
            | Self::I136
            | Self::I137
            | Self::I138
            | Self::I139
            | Self::I140
            | Self::I141
            | Self::I142
            | Self::I143
            | Self::I144
            | Self::I145 => None,
        }
    }
}
impl super::MostAbundantIsotope for IodineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::I127
    }
}
impl TryFrom<u16> for IodineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            107u16 => Ok(Self::I107),
            108u16 => Ok(Self::I108),
            109u16 => Ok(Self::I109),
            110u16 => Ok(Self::I110),
            111u16 => Ok(Self::I111),
            112u16 => Ok(Self::I112),
            113u16 => Ok(Self::I113),
            114u16 => Ok(Self::I114),
            115u16 => Ok(Self::I115),
            116u16 => Ok(Self::I116),
            117u16 => Ok(Self::I117),
            118u16 => Ok(Self::I118),
            119u16 => Ok(Self::I119),
            120u16 => Ok(Self::I120),
            121u16 => Ok(Self::I121),
            122u16 => Ok(Self::I122),
            123u16 => Ok(Self::I123),
            124u16 => Ok(Self::I124),
            125u16 => Ok(Self::I125),
            126u16 => Ok(Self::I126),
            127u16 => Ok(Self::I127),
            128u16 => Ok(Self::I128),
            129u16 => Ok(Self::I129),
            130u16 => Ok(Self::I130),
            131u16 => Ok(Self::I131),
            132u16 => Ok(Self::I132),
            133u16 => Ok(Self::I133),
            134u16 => Ok(Self::I134),
            135u16 => Ok(Self::I135),
            136u16 => Ok(Self::I136),
            137u16 => Ok(Self::I137),
            138u16 => Ok(Self::I138),
            139u16 => Ok(Self::I139),
            140u16 => Ok(Self::I140),
            141u16 => Ok(Self::I141),
            142u16 => Ok(Self::I142),
            143u16 => Ok(Self::I143),
            144u16 => Ok(Self::I144),
            145u16 => Ok(Self::I145),
            _ => Err(crate::errors::Error::Isotope(crate::Element::I, value)),
        }
    }
}
impl std::fmt::Display for IodineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I107 => write!(f, "I107"),
            Self::I108 => write!(f, "I108"),
            Self::I109 => write!(f, "I109"),
            Self::I110 => write!(f, "I110"),
            Self::I111 => write!(f, "I111"),
            Self::I112 => write!(f, "I112"),
            Self::I113 => write!(f, "I113"),
            Self::I114 => write!(f, "I114"),
            Self::I115 => write!(f, "I115"),
            Self::I116 => write!(f, "I116"),
            Self::I117 => write!(f, "I117"),
            Self::I118 => write!(f, "I118"),
            Self::I119 => write!(f, "I119"),
            Self::I120 => write!(f, "I120"),
            Self::I121 => write!(f, "I121"),
            Self::I122 => write!(f, "I122"),
            Self::I123 => write!(f, "I123"),
            Self::I124 => write!(f, "I124"),
            Self::I125 => write!(f, "I125"),
            Self::I126 => write!(f, "I126"),
            Self::I127 => write!(f, "I127"),
            Self::I128 => write!(f, "I128"),
            Self::I129 => write!(f, "I129"),
            Self::I130 => write!(f, "I130"),
            Self::I131 => write!(f, "I131"),
            Self::I132 => write!(f, "I132"),
            Self::I133 => write!(f, "I133"),
            Self::I134 => write!(f, "I134"),
            Self::I135 => write!(f, "I135"),
            Self::I136 => write!(f, "I136"),
            Self::I137 => write!(f, "I137"),
            Self::I138 => write!(f, "I138"),
            Self::I139 => write!(f, "I139"),
            Self::I140 => write!(f, "I140"),
            Self::I141 => write!(f, "I141"),
            Self::I142 => write!(f, "I142"),
            Self::I143 => write!(f, "I143"),
            Self::I144 => write!(f, "I144"),
            Self::I145 => write!(f, "I145"),
        }
    }
}
