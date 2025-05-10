//! Isotopes of the element Cerium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Cerium
pub enum CeriumIsotope {
    /// Isotope Ce119 of Cerium
    Ce119,
    /// Isotope Ce120 of Cerium
    Ce120,
    /// Isotope Ce121 of Cerium
    Ce121,
    /// Isotope Ce122 of Cerium
    Ce122,
    /// Isotope Ce123 of Cerium
    Ce123,
    /// Isotope Ce124 of Cerium
    Ce124,
    /// Isotope Ce125 of Cerium
    Ce125,
    /// Isotope Ce126 of Cerium
    Ce126,
    /// Isotope Ce127 of Cerium
    Ce127,
    /// Isotope Ce128 of Cerium
    Ce128,
    /// Isotope Ce129 of Cerium
    Ce129,
    /// Isotope Ce130 of Cerium
    Ce130,
    /// Isotope Ce131 of Cerium
    Ce131,
    /// Isotope Ce132 of Cerium
    Ce132,
    /// Isotope Ce133 of Cerium
    Ce133,
    /// Isotope Ce134 of Cerium
    Ce134,
    /// Isotope Ce135 of Cerium
    Ce135,
    /// Isotope Ce136 of Cerium
    Ce136,
    /// Isotope Ce137 of Cerium
    Ce137,
    /// Isotope Ce138 of Cerium
    Ce138,
    /// Isotope Ce139 of Cerium
    Ce139,
    /// Isotope Ce140 of Cerium
    Ce140,
    /// Isotope Ce141 of Cerium
    Ce141,
    /// Isotope Ce142 of Cerium
    Ce142,
    /// Isotope Ce143 of Cerium
    Ce143,
    /// Isotope Ce144 of Cerium
    Ce144,
    /// Isotope Ce145 of Cerium
    Ce145,
    /// Isotope Ce146 of Cerium
    Ce146,
    /// Isotope Ce147 of Cerium
    Ce147,
    /// Isotope Ce148 of Cerium
    Ce148,
    /// Isotope Ce149 of Cerium
    Ce149,
    /// Isotope Ce150 of Cerium
    Ce150,
    /// Isotope Ce151 of Cerium
    Ce151,
    /// Isotope Ce152 of Cerium
    Ce152,
    /// Isotope Ce153 of Cerium
    Ce153,
    /// Isotope Ce154 of Cerium
    Ce154,
    /// Isotope Ce155 of Cerium
    Ce155,
    /// Isotope Ce156 of Cerium
    Ce156,
    /// Isotope Ce157 of Cerium
    Ce157,
}
impl super::RelativeAtomicMass for CeriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ce119 => 118.95271f64,
            Self::Ce120 => 119.94654f64,
            Self::Ce121 => 120.94335f64,
            Self::Ce122 => 121.93787f64,
            Self::Ce123 => 122.93528f64,
            Self::Ce124 => 123.93031f64,
            Self::Ce125 => 124.92844f64,
            Self::Ce126 => 125.923971f64,
            Self::Ce127 => 126.922727f64,
            Self::Ce128 => 127.918911f64,
            Self::Ce129 => 128.918102f64,
            Self::Ce130 => 129.914736f64,
            Self::Ce131 => 130.914429f64,
            Self::Ce132 => 131.911464f64,
            Self::Ce133 => 132.91152f64,
            Self::Ce134 => 133.908928f64,
            Self::Ce135 => 134.909161f64,
            Self::Ce136 => 135.90712921f64,
            Self::Ce137 => 136.90776236f64,
            Self::Ce138 => 137.905991f64,
            Self::Ce139 => 138.9066551f64,
            Self::Ce140 => 139.9054431f64,
            Self::Ce141 => 140.9082807f64,
            Self::Ce142 => 141.9092504f64,
            Self::Ce143 => 142.9123921f64,
            Self::Ce144 => 143.9136529f64,
            Self::Ce145 => 144.917265f64,
            Self::Ce146 => 145.918802f64,
            Self::Ce147 => 146.9226899f64,
            Self::Ce148 => 147.924424f64,
            Self::Ce149 => 148.928427f64,
            Self::Ce150 => 149.930384f64,
            Self::Ce151 => 150.934272f64,
            Self::Ce152 => 151.9366f64,
            Self::Ce153 => 152.94093f64,
            Self::Ce154 => 153.9438f64,
            Self::Ce155 => 154.94855f64,
            Self::Ce156 => 155.95183f64,
            Self::Ce157 => 156.95705f64,
        }
    }
}
impl super::ElementVariant for CeriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ce
    }
}
impl super::MassNumber for CeriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ce119 => 119u16,
            Self::Ce120 => 120u16,
            Self::Ce121 => 121u16,
            Self::Ce122 => 122u16,
            Self::Ce123 => 123u16,
            Self::Ce124 => 124u16,
            Self::Ce125 => 125u16,
            Self::Ce126 => 126u16,
            Self::Ce127 => 127u16,
            Self::Ce128 => 128u16,
            Self::Ce129 => 129u16,
            Self::Ce130 => 130u16,
            Self::Ce131 => 131u16,
            Self::Ce132 => 132u16,
            Self::Ce133 => 133u16,
            Self::Ce134 => 134u16,
            Self::Ce135 => 135u16,
            Self::Ce136 => 136u16,
            Self::Ce137 => 137u16,
            Self::Ce138 => 138u16,
            Self::Ce139 => 139u16,
            Self::Ce140 => 140u16,
            Self::Ce141 => 141u16,
            Self::Ce142 => 142u16,
            Self::Ce143 => 143u16,
            Self::Ce144 => 144u16,
            Self::Ce145 => 145u16,
            Self::Ce146 => 146u16,
            Self::Ce147 => 147u16,
            Self::Ce148 => 148u16,
            Self::Ce149 => 149u16,
            Self::Ce150 => 150u16,
            Self::Ce151 => 151u16,
            Self::Ce152 => 152u16,
            Self::Ce153 => 153u16,
            Self::Ce154 => 154u16,
            Self::Ce155 => 155u16,
            Self::Ce156 => 156u16,
            Self::Ce157 => 157u16,
        }
    }
}
impl super::IsotopicComposition for CeriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ce136 => Some(0.00185f64),
            Self::Ce138 => Some(0.00251f64),
            Self::Ce140 => Some(0.8845f64),
            Self::Ce142 => Some(0.11114f64),
            Self::Ce119
            | Self::Ce120
            | Self::Ce121
            | Self::Ce122
            | Self::Ce123
            | Self::Ce124
            | Self::Ce125
            | Self::Ce126
            | Self::Ce127
            | Self::Ce128
            | Self::Ce129
            | Self::Ce130
            | Self::Ce131
            | Self::Ce132
            | Self::Ce133
            | Self::Ce134
            | Self::Ce135
            | Self::Ce137
            | Self::Ce139
            | Self::Ce141
            | Self::Ce143
            | Self::Ce144
            | Self::Ce145
            | Self::Ce146
            | Self::Ce147
            | Self::Ce148
            | Self::Ce149
            | Self::Ce150
            | Self::Ce151
            | Self::Ce152
            | Self::Ce153
            | Self::Ce154
            | Self::Ce155
            | Self::Ce156
            | Self::Ce157 => None,
        }
    }
}
impl super::MostAbundantIsotope for CeriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ce140
    }
}
impl TryFrom<u16> for CeriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            119u16 => Ok(Self::Ce119),
            120u16 => Ok(Self::Ce120),
            121u16 => Ok(Self::Ce121),
            122u16 => Ok(Self::Ce122),
            123u16 => Ok(Self::Ce123),
            124u16 => Ok(Self::Ce124),
            125u16 => Ok(Self::Ce125),
            126u16 => Ok(Self::Ce126),
            127u16 => Ok(Self::Ce127),
            128u16 => Ok(Self::Ce128),
            129u16 => Ok(Self::Ce129),
            130u16 => Ok(Self::Ce130),
            131u16 => Ok(Self::Ce131),
            132u16 => Ok(Self::Ce132),
            133u16 => Ok(Self::Ce133),
            134u16 => Ok(Self::Ce134),
            135u16 => Ok(Self::Ce135),
            136u16 => Ok(Self::Ce136),
            137u16 => Ok(Self::Ce137),
            138u16 => Ok(Self::Ce138),
            139u16 => Ok(Self::Ce139),
            140u16 => Ok(Self::Ce140),
            141u16 => Ok(Self::Ce141),
            142u16 => Ok(Self::Ce142),
            143u16 => Ok(Self::Ce143),
            144u16 => Ok(Self::Ce144),
            145u16 => Ok(Self::Ce145),
            146u16 => Ok(Self::Ce146),
            147u16 => Ok(Self::Ce147),
            148u16 => Ok(Self::Ce148),
            149u16 => Ok(Self::Ce149),
            150u16 => Ok(Self::Ce150),
            151u16 => Ok(Self::Ce151),
            152u16 => Ok(Self::Ce152),
            153u16 => Ok(Self::Ce153),
            154u16 => Ok(Self::Ce154),
            155u16 => Ok(Self::Ce155),
            156u16 => Ok(Self::Ce156),
            157u16 => Ok(Self::Ce157),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ce, value)),
        }
    }
}
impl std::fmt::Display for CeriumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ce119 => write!(f, "Ce119"),
            Self::Ce120 => write!(f, "Ce120"),
            Self::Ce121 => write!(f, "Ce121"),
            Self::Ce122 => write!(f, "Ce122"),
            Self::Ce123 => write!(f, "Ce123"),
            Self::Ce124 => write!(f, "Ce124"),
            Self::Ce125 => write!(f, "Ce125"),
            Self::Ce126 => write!(f, "Ce126"),
            Self::Ce127 => write!(f, "Ce127"),
            Self::Ce128 => write!(f, "Ce128"),
            Self::Ce129 => write!(f, "Ce129"),
            Self::Ce130 => write!(f, "Ce130"),
            Self::Ce131 => write!(f, "Ce131"),
            Self::Ce132 => write!(f, "Ce132"),
            Self::Ce133 => write!(f, "Ce133"),
            Self::Ce134 => write!(f, "Ce134"),
            Self::Ce135 => write!(f, "Ce135"),
            Self::Ce136 => write!(f, "Ce136"),
            Self::Ce137 => write!(f, "Ce137"),
            Self::Ce138 => write!(f, "Ce138"),
            Self::Ce139 => write!(f, "Ce139"),
            Self::Ce140 => write!(f, "Ce140"),
            Self::Ce141 => write!(f, "Ce141"),
            Self::Ce142 => write!(f, "Ce142"),
            Self::Ce143 => write!(f, "Ce143"),
            Self::Ce144 => write!(f, "Ce144"),
            Self::Ce145 => write!(f, "Ce145"),
            Self::Ce146 => write!(f, "Ce146"),
            Self::Ce147 => write!(f, "Ce147"),
            Self::Ce148 => write!(f, "Ce148"),
            Self::Ce149 => write!(f, "Ce149"),
            Self::Ce150 => write!(f, "Ce150"),
            Self::Ce151 => write!(f, "Ce151"),
            Self::Ce152 => write!(f, "Ce152"),
            Self::Ce153 => write!(f, "Ce153"),
            Self::Ce154 => write!(f, "Ce154"),
            Self::Ce155 => write!(f, "Ce155"),
            Self::Ce156 => write!(f, "Ce156"),
            Self::Ce157 => write!(f, "Ce157"),
        }
    }
}
