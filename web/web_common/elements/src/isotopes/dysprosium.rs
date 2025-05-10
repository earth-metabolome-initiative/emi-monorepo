//! Isotopes of the element Dysprosium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Dysprosium
pub enum DysprosiumIsotope {
    /// Isotope Dy138 of Dysprosium
    Dy138,
    /// Isotope Dy139 of Dysprosium
    Dy139,
    /// Isotope Dy140 of Dysprosium
    Dy140,
    /// Isotope Dy141 of Dysprosium
    Dy141,
    /// Isotope Dy142 of Dysprosium
    Dy142,
    /// Isotope Dy143 of Dysprosium
    Dy143,
    /// Isotope Dy144 of Dysprosium
    Dy144,
    /// Isotope Dy145 of Dysprosium
    Dy145,
    /// Isotope Dy146 of Dysprosium
    Dy146,
    /// Isotope Dy147 of Dysprosium
    Dy147,
    /// Isotope Dy148 of Dysprosium
    Dy148,
    /// Isotope Dy149 of Dysprosium
    Dy149,
    /// Isotope Dy150 of Dysprosium
    Dy150,
    /// Isotope Dy151 of Dysprosium
    Dy151,
    /// Isotope Dy152 of Dysprosium
    Dy152,
    /// Isotope Dy153 of Dysprosium
    Dy153,
    /// Isotope Dy154 of Dysprosium
    Dy154,
    /// Isotope Dy155 of Dysprosium
    Dy155,
    /// Isotope Dy156 of Dysprosium
    Dy156,
    /// Isotope Dy157 of Dysprosium
    Dy157,
    /// Isotope Dy158 of Dysprosium
    Dy158,
    /// Isotope Dy159 of Dysprosium
    Dy159,
    /// Isotope Dy160 of Dysprosium
    Dy160,
    /// Isotope Dy161 of Dysprosium
    Dy161,
    /// Isotope Dy162 of Dysprosium
    Dy162,
    /// Isotope Dy163 of Dysprosium
    Dy163,
    /// Isotope Dy164 of Dysprosium
    Dy164,
    /// Isotope Dy165 of Dysprosium
    Dy165,
    /// Isotope Dy166 of Dysprosium
    Dy166,
    /// Isotope Dy167 of Dysprosium
    Dy167,
    /// Isotope Dy168 of Dysprosium
    Dy168,
    /// Isotope Dy169 of Dysprosium
    Dy169,
    /// Isotope Dy170 of Dysprosium
    Dy170,
    /// Isotope Dy171 of Dysprosium
    Dy171,
    /// Isotope Dy172 of Dysprosium
    Dy172,
    /// Isotope Dy173 of Dysprosium
    Dy173,
}
impl super::RelativeAtomicMass for DysprosiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Dy138 => 137.9625f64,
            Self::Dy139 => 138.95959f64,
            Self::Dy140 => 139.95402f64,
            Self::Dy141 => 140.95128f64,
            Self::Dy142 => 141.94619f64,
            Self::Dy143 => 142.943994f64,
            Self::Dy144 => 143.9392695f64,
            Self::Dy145 => 144.937474f64,
            Self::Dy146 => 145.9328445f64,
            Self::Dy147 => 146.9310827f64,
            Self::Dy148 => 147.927157f64,
            Self::Dy149 => 148.927322f64,
            Self::Dy150 => 149.9255933f64,
            Self::Dy151 => 150.9261916f64,
            Self::Dy152 => 151.9247253f64,
            Self::Dy153 => 152.9257724f64,
            Self::Dy154 => 153.9244293f64,
            Self::Dy155 => 154.925759f64,
            Self::Dy156 => 155.9242847f64,
            Self::Dy157 => 156.9254707f64,
            Self::Dy158 => 157.9244159f64,
            Self::Dy159 => 158.925747f64,
            Self::Dy160 => 159.9252046f64,
            Self::Dy161 => 160.9269405f64,
            Self::Dy162 => 161.9268056f64,
            Self::Dy163 => 162.9287383f64,
            Self::Dy164 => 163.9291819f64,
            Self::Dy165 => 164.9317105f64,
            Self::Dy166 => 165.9328139f64,
            Self::Dy167 => 166.935661f64,
            Self::Dy168 => 167.93713f64,
            Self::Dy169 => 168.94031f64,
            Self::Dy170 => 169.94239f64,
            Self::Dy171 => 170.94612f64,
            Self::Dy172 => 171.94846f64,
            Self::Dy173 => 172.95283f64,
        }
    }
}
impl super::ElementVariant for DysprosiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Dy
    }
}
impl super::MassNumber for DysprosiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Dy138 => 138u16,
            Self::Dy139 => 139u16,
            Self::Dy140 => 140u16,
            Self::Dy141 => 141u16,
            Self::Dy142 => 142u16,
            Self::Dy143 => 143u16,
            Self::Dy144 => 144u16,
            Self::Dy145 => 145u16,
            Self::Dy146 => 146u16,
            Self::Dy147 => 147u16,
            Self::Dy148 => 148u16,
            Self::Dy149 => 149u16,
            Self::Dy150 => 150u16,
            Self::Dy151 => 151u16,
            Self::Dy152 => 152u16,
            Self::Dy153 => 153u16,
            Self::Dy154 => 154u16,
            Self::Dy155 => 155u16,
            Self::Dy156 => 156u16,
            Self::Dy157 => 157u16,
            Self::Dy158 => 158u16,
            Self::Dy159 => 159u16,
            Self::Dy160 => 160u16,
            Self::Dy161 => 161u16,
            Self::Dy162 => 162u16,
            Self::Dy163 => 163u16,
            Self::Dy164 => 164u16,
            Self::Dy165 => 165u16,
            Self::Dy166 => 166u16,
            Self::Dy167 => 167u16,
            Self::Dy168 => 168u16,
            Self::Dy169 => 169u16,
            Self::Dy170 => 170u16,
            Self::Dy171 => 171u16,
            Self::Dy172 => 172u16,
            Self::Dy173 => 173u16,
        }
    }
}
impl super::IsotopicComposition for DysprosiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Dy156 => Some(0.00056f64),
            Self::Dy158 => Some(0.00095f64),
            Self::Dy160 => Some(0.02329f64),
            Self::Dy161 => Some(0.18889f64),
            Self::Dy162 => Some(0.25475f64),
            Self::Dy163 => Some(0.24896f64),
            Self::Dy164 => Some(0.2826f64),
            Self::Dy138
            | Self::Dy139
            | Self::Dy140
            | Self::Dy141
            | Self::Dy142
            | Self::Dy143
            | Self::Dy144
            | Self::Dy145
            | Self::Dy146
            | Self::Dy147
            | Self::Dy148
            | Self::Dy149
            | Self::Dy150
            | Self::Dy151
            | Self::Dy152
            | Self::Dy153
            | Self::Dy154
            | Self::Dy155
            | Self::Dy157
            | Self::Dy159
            | Self::Dy165
            | Self::Dy166
            | Self::Dy167
            | Self::Dy168
            | Self::Dy169
            | Self::Dy170
            | Self::Dy171
            | Self::Dy172
            | Self::Dy173 => None,
        }
    }
}
impl super::MostAbundantIsotope for DysprosiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Dy164
    }
}
impl TryFrom<u16> for DysprosiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            138u16 => Ok(Self::Dy138),
            139u16 => Ok(Self::Dy139),
            140u16 => Ok(Self::Dy140),
            141u16 => Ok(Self::Dy141),
            142u16 => Ok(Self::Dy142),
            143u16 => Ok(Self::Dy143),
            144u16 => Ok(Self::Dy144),
            145u16 => Ok(Self::Dy145),
            146u16 => Ok(Self::Dy146),
            147u16 => Ok(Self::Dy147),
            148u16 => Ok(Self::Dy148),
            149u16 => Ok(Self::Dy149),
            150u16 => Ok(Self::Dy150),
            151u16 => Ok(Self::Dy151),
            152u16 => Ok(Self::Dy152),
            153u16 => Ok(Self::Dy153),
            154u16 => Ok(Self::Dy154),
            155u16 => Ok(Self::Dy155),
            156u16 => Ok(Self::Dy156),
            157u16 => Ok(Self::Dy157),
            158u16 => Ok(Self::Dy158),
            159u16 => Ok(Self::Dy159),
            160u16 => Ok(Self::Dy160),
            161u16 => Ok(Self::Dy161),
            162u16 => Ok(Self::Dy162),
            163u16 => Ok(Self::Dy163),
            164u16 => Ok(Self::Dy164),
            165u16 => Ok(Self::Dy165),
            166u16 => Ok(Self::Dy166),
            167u16 => Ok(Self::Dy167),
            168u16 => Ok(Self::Dy168),
            169u16 => Ok(Self::Dy169),
            170u16 => Ok(Self::Dy170),
            171u16 => Ok(Self::Dy171),
            172u16 => Ok(Self::Dy172),
            173u16 => Ok(Self::Dy173),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Dy, value)),
        }
    }
}
impl std::fmt::Display for DysprosiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dy138 => write!(f, "Dy138"),
            Self::Dy139 => write!(f, "Dy139"),
            Self::Dy140 => write!(f, "Dy140"),
            Self::Dy141 => write!(f, "Dy141"),
            Self::Dy142 => write!(f, "Dy142"),
            Self::Dy143 => write!(f, "Dy143"),
            Self::Dy144 => write!(f, "Dy144"),
            Self::Dy145 => write!(f, "Dy145"),
            Self::Dy146 => write!(f, "Dy146"),
            Self::Dy147 => write!(f, "Dy147"),
            Self::Dy148 => write!(f, "Dy148"),
            Self::Dy149 => write!(f, "Dy149"),
            Self::Dy150 => write!(f, "Dy150"),
            Self::Dy151 => write!(f, "Dy151"),
            Self::Dy152 => write!(f, "Dy152"),
            Self::Dy153 => write!(f, "Dy153"),
            Self::Dy154 => write!(f, "Dy154"),
            Self::Dy155 => write!(f, "Dy155"),
            Self::Dy156 => write!(f, "Dy156"),
            Self::Dy157 => write!(f, "Dy157"),
            Self::Dy158 => write!(f, "Dy158"),
            Self::Dy159 => write!(f, "Dy159"),
            Self::Dy160 => write!(f, "Dy160"),
            Self::Dy161 => write!(f, "Dy161"),
            Self::Dy162 => write!(f, "Dy162"),
            Self::Dy163 => write!(f, "Dy163"),
            Self::Dy164 => write!(f, "Dy164"),
            Self::Dy165 => write!(f, "Dy165"),
            Self::Dy166 => write!(f, "Dy166"),
            Self::Dy167 => write!(f, "Dy167"),
            Self::Dy168 => write!(f, "Dy168"),
            Self::Dy169 => write!(f, "Dy169"),
            Self::Dy170 => write!(f, "Dy170"),
            Self::Dy171 => write!(f, "Dy171"),
            Self::Dy172 => write!(f, "Dy172"),
            Self::Dy173 => write!(f, "Dy173"),
        }
    }
}
