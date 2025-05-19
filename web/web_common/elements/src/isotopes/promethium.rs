//! Isotopes of the element Promethium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Promethium
pub enum PromethiumIsotope {
    /// Isotope Pm126 of Promethium
    Pm126,
    /// Isotope Pm127 of Promethium
    Pm127,
    /// Isotope Pm128 of Promethium
    Pm128,
    /// Isotope Pm129 of Promethium
    Pm129,
    /// Isotope Pm130 of Promethium
    Pm130,
    /// Isotope Pm131 of Promethium
    Pm131,
    /// Isotope Pm132 of Promethium
    Pm132,
    /// Isotope Pm133 of Promethium
    Pm133,
    /// Isotope Pm134 of Promethium
    Pm134,
    /// Isotope Pm135 of Promethium
    Pm135,
    /// Isotope Pm136 of Promethium
    Pm136,
    /// Isotope Pm137 of Promethium
    Pm137,
    /// Isotope Pm138 of Promethium
    Pm138,
    /// Isotope Pm139 of Promethium
    Pm139,
    /// Isotope Pm140 of Promethium
    Pm140,
    /// Isotope Pm141 of Promethium
    Pm141,
    /// Isotope Pm142 of Promethium
    Pm142,
    /// Isotope Pm143 of Promethium
    Pm143,
    /// Isotope Pm144 of Promethium
    Pm144,
    /// Isotope Pm145 of Promethium
    Pm145,
    /// Isotope Pm146 of Promethium
    Pm146,
    /// Isotope Pm147 of Promethium
    Pm147,
    /// Isotope Pm148 of Promethium
    Pm148,
    /// Isotope Pm149 of Promethium
    Pm149,
    /// Isotope Pm150 of Promethium
    Pm150,
    /// Isotope Pm151 of Promethium
    Pm151,
    /// Isotope Pm152 of Promethium
    Pm152,
    /// Isotope Pm153 of Promethium
    Pm153,
    /// Isotope Pm154 of Promethium
    Pm154,
    /// Isotope Pm155 of Promethium
    Pm155,
    /// Isotope Pm156 of Promethium
    Pm156,
    /// Isotope Pm157 of Promethium
    Pm157,
    /// Isotope Pm158 of Promethium
    Pm158,
    /// Isotope Pm159 of Promethium
    Pm159,
    /// Isotope Pm160 of Promethium
    Pm160,
    /// Isotope Pm161 of Promethium
    Pm161,
    /// Isotope Pm162 of Promethium
    Pm162,
    /// Isotope Pm163 of Promethium
    Pm163,
}
impl super::RelativeAtomicMass for PromethiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Pm126 => 125.95792f64,
            Self::Pm127 => 126.95192f64,
            Self::Pm128 => 127.9487f64,
            Self::Pm129 => 128.94323f64,
            Self::Pm130 => 129.94053f64,
            Self::Pm131 => 130.93567f64,
            Self::Pm132 => 131.93384f64,
            Self::Pm133 => 132.929782f64,
            Self::Pm134 => 133.928353f64,
            Self::Pm135 => 134.924823f64,
            Self::Pm136 => 135.923585f64,
            Self::Pm137 => 136.92048f64,
            Self::Pm138 => 137.919548f64,
            Self::Pm139 => 138.9168f64,
            Self::Pm140 => 139.91604f64,
            Self::Pm141 => 140.913555f64,
            Self::Pm142 => 141.91289f64,
            Self::Pm143 => 142.9109383f64,
            Self::Pm144 => 143.9125964f64,
            Self::Pm145 => 144.9127559f64,
            Self::Pm146 => 145.9147024f64,
            Self::Pm147 => 146.915145f64,
            Self::Pm148 => 147.9174819f64,
            Self::Pm149 => 148.9183423f64,
            Self::Pm150 => 149.920991f64,
            Self::Pm151 => 150.9212175f64,
            Self::Pm152 => 151.923506f64,
            Self::Pm153 => 152.9241567f64,
            Self::Pm154 => 153.926472f64,
            Self::Pm155 => 154.928137f64,
            Self::Pm156 => 155.9311175f64,
            Self::Pm157 => 156.9331214f64,
            Self::Pm158 => 157.936565f64,
            Self::Pm159 => 158.939287f64,
            Self::Pm160 => 159.9431f64,
            Self::Pm161 => 160.94607f64,
            Self::Pm162 => 161.95022f64,
            Self::Pm163 => 162.95357f64,
        }
    }
}
impl super::ElementVariant for PromethiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Pm
    }
}
impl super::MassNumber for PromethiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Pm126 => 126u16,
            Self::Pm127 => 127u16,
            Self::Pm128 => 128u16,
            Self::Pm129 => 129u16,
            Self::Pm130 => 130u16,
            Self::Pm131 => 131u16,
            Self::Pm132 => 132u16,
            Self::Pm133 => 133u16,
            Self::Pm134 => 134u16,
            Self::Pm135 => 135u16,
            Self::Pm136 => 136u16,
            Self::Pm137 => 137u16,
            Self::Pm138 => 138u16,
            Self::Pm139 => 139u16,
            Self::Pm140 => 140u16,
            Self::Pm141 => 141u16,
            Self::Pm142 => 142u16,
            Self::Pm143 => 143u16,
            Self::Pm144 => 144u16,
            Self::Pm145 => 145u16,
            Self::Pm146 => 146u16,
            Self::Pm147 => 147u16,
            Self::Pm148 => 148u16,
            Self::Pm149 => 149u16,
            Self::Pm150 => 150u16,
            Self::Pm151 => 151u16,
            Self::Pm152 => 152u16,
            Self::Pm153 => 153u16,
            Self::Pm154 => 154u16,
            Self::Pm155 => 155u16,
            Self::Pm156 => 156u16,
            Self::Pm157 => 157u16,
            Self::Pm158 => 158u16,
            Self::Pm159 => 159u16,
            Self::Pm160 => 160u16,
            Self::Pm161 => 161u16,
            Self::Pm162 => 162u16,
            Self::Pm163 => 163u16,
        }
    }
}
impl super::IsotopicComposition for PromethiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for PromethiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Pm163
    }
}
impl TryFrom<u16> for PromethiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            126u16 => Ok(Self::Pm126),
            127u16 => Ok(Self::Pm127),
            128u16 => Ok(Self::Pm128),
            129u16 => Ok(Self::Pm129),
            130u16 => Ok(Self::Pm130),
            131u16 => Ok(Self::Pm131),
            132u16 => Ok(Self::Pm132),
            133u16 => Ok(Self::Pm133),
            134u16 => Ok(Self::Pm134),
            135u16 => Ok(Self::Pm135),
            136u16 => Ok(Self::Pm136),
            137u16 => Ok(Self::Pm137),
            138u16 => Ok(Self::Pm138),
            139u16 => Ok(Self::Pm139),
            140u16 => Ok(Self::Pm140),
            141u16 => Ok(Self::Pm141),
            142u16 => Ok(Self::Pm142),
            143u16 => Ok(Self::Pm143),
            144u16 => Ok(Self::Pm144),
            145u16 => Ok(Self::Pm145),
            146u16 => Ok(Self::Pm146),
            147u16 => Ok(Self::Pm147),
            148u16 => Ok(Self::Pm148),
            149u16 => Ok(Self::Pm149),
            150u16 => Ok(Self::Pm150),
            151u16 => Ok(Self::Pm151),
            152u16 => Ok(Self::Pm152),
            153u16 => Ok(Self::Pm153),
            154u16 => Ok(Self::Pm154),
            155u16 => Ok(Self::Pm155),
            156u16 => Ok(Self::Pm156),
            157u16 => Ok(Self::Pm157),
            158u16 => Ok(Self::Pm158),
            159u16 => Ok(Self::Pm159),
            160u16 => Ok(Self::Pm160),
            161u16 => Ok(Self::Pm161),
            162u16 => Ok(Self::Pm162),
            163u16 => Ok(Self::Pm163),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Pm, value)),
        }
    }
}
impl std::fmt::Display for PromethiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pm126 => write!(f, "Pm126"),
            Self::Pm127 => write!(f, "Pm127"),
            Self::Pm128 => write!(f, "Pm128"),
            Self::Pm129 => write!(f, "Pm129"),
            Self::Pm130 => write!(f, "Pm130"),
            Self::Pm131 => write!(f, "Pm131"),
            Self::Pm132 => write!(f, "Pm132"),
            Self::Pm133 => write!(f, "Pm133"),
            Self::Pm134 => write!(f, "Pm134"),
            Self::Pm135 => write!(f, "Pm135"),
            Self::Pm136 => write!(f, "Pm136"),
            Self::Pm137 => write!(f, "Pm137"),
            Self::Pm138 => write!(f, "Pm138"),
            Self::Pm139 => write!(f, "Pm139"),
            Self::Pm140 => write!(f, "Pm140"),
            Self::Pm141 => write!(f, "Pm141"),
            Self::Pm142 => write!(f, "Pm142"),
            Self::Pm143 => write!(f, "Pm143"),
            Self::Pm144 => write!(f, "Pm144"),
            Self::Pm145 => write!(f, "Pm145"),
            Self::Pm146 => write!(f, "Pm146"),
            Self::Pm147 => write!(f, "Pm147"),
            Self::Pm148 => write!(f, "Pm148"),
            Self::Pm149 => write!(f, "Pm149"),
            Self::Pm150 => write!(f, "Pm150"),
            Self::Pm151 => write!(f, "Pm151"),
            Self::Pm152 => write!(f, "Pm152"),
            Self::Pm153 => write!(f, "Pm153"),
            Self::Pm154 => write!(f, "Pm154"),
            Self::Pm155 => write!(f, "Pm155"),
            Self::Pm156 => write!(f, "Pm156"),
            Self::Pm157 => write!(f, "Pm157"),
            Self::Pm158 => write!(f, "Pm158"),
            Self::Pm159 => write!(f, "Pm159"),
            Self::Pm160 => write!(f, "Pm160"),
            Self::Pm161 => write!(f, "Pm161"),
            Self::Pm162 => write!(f, "Pm162"),
            Self::Pm163 => write!(f, "Pm163"),
        }
    }
}
