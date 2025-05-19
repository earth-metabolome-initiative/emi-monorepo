//! Isotopes of the element Terbium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Terbium
pub enum TerbiumIsotope {
    /// Isotope Tb135 of Terbium
    Tb135,
    /// Isotope Tb136 of Terbium
    Tb136,
    /// Isotope Tb137 of Terbium
    Tb137,
    /// Isotope Tb138 of Terbium
    Tb138,
    /// Isotope Tb139 of Terbium
    Tb139,
    /// Isotope Tb140 of Terbium
    Tb140,
    /// Isotope Tb141 of Terbium
    Tb141,
    /// Isotope Tb142 of Terbium
    Tb142,
    /// Isotope Tb143 of Terbium
    Tb143,
    /// Isotope Tb144 of Terbium
    Tb144,
    /// Isotope Tb145 of Terbium
    Tb145,
    /// Isotope Tb146 of Terbium
    Tb146,
    /// Isotope Tb147 of Terbium
    Tb147,
    /// Isotope Tb148 of Terbium
    Tb148,
    /// Isotope Tb149 of Terbium
    Tb149,
    /// Isotope Tb150 of Terbium
    Tb150,
    /// Isotope Tb151 of Terbium
    Tb151,
    /// Isotope Tb152 of Terbium
    Tb152,
    /// Isotope Tb153 of Terbium
    Tb153,
    /// Isotope Tb154 of Terbium
    Tb154,
    /// Isotope Tb155 of Terbium
    Tb155,
    /// Isotope Tb156 of Terbium
    Tb156,
    /// Isotope Tb157 of Terbium
    Tb157,
    /// Isotope Tb158 of Terbium
    Tb158,
    /// Isotope Tb159 of Terbium
    Tb159,
    /// Isotope Tb160 of Terbium
    Tb160,
    /// Isotope Tb161 of Terbium
    Tb161,
    /// Isotope Tb162 of Terbium
    Tb162,
    /// Isotope Tb163 of Terbium
    Tb163,
    /// Isotope Tb164 of Terbium
    Tb164,
    /// Isotope Tb165 of Terbium
    Tb165,
    /// Isotope Tb166 of Terbium
    Tb166,
    /// Isotope Tb167 of Terbium
    Tb167,
    /// Isotope Tb168 of Terbium
    Tb168,
    /// Isotope Tb169 of Terbium
    Tb169,
    /// Isotope Tb170 of Terbium
    Tb170,
    /// Isotope Tb171 of Terbium
    Tb171,
}
impl super::RelativeAtomicMass for TerbiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Tb135 => 134.96476f64,
            Self::Tb136 => 135.96129f64,
            Self::Tb137 => 136.95602f64,
            Self::Tb138 => 137.95312f64,
            Self::Tb139 => 138.94833f64,
            Self::Tb140 => 139.94581f64,
            Self::Tb141 => 140.94145f64,
            Self::Tb142 => 141.93928f64,
            Self::Tb143 => 142.935137f64,
            Self::Tb144 => 143.933045f64,
            Self::Tb145 => 144.92882f64,
            Self::Tb146 => 145.927253f64,
            Self::Tb147 => 146.9240548f64,
            Self::Tb148 => 147.924282f64,
            Self::Tb149 => 148.9232535f64,
            Self::Tb150 => 149.9236649f64,
            Self::Tb151 => 150.9231096f64,
            Self::Tb152 => 151.924083f64,
            Self::Tb153 => 152.9234424f64,
            Self::Tb154 => 153.924685f64,
            Self::Tb155 => 154.923511f64,
            Self::Tb156 => 155.9247552f64,
            Self::Tb157 => 156.924033f64,
            Self::Tb158 => 157.9254209f64,
            Self::Tb159 => 158.9253547f64,
            Self::Tb160 => 159.9271756f64,
            Self::Tb161 => 160.9275778f64,
            Self::Tb162 => 161.929495f64,
            Self::Tb163 => 162.9306547f64,
            Self::Tb164 => 163.93336f64,
            Self::Tb165 => 164.93498f64,
            Self::Tb166 => 165.93786f64,
            Self::Tb167 => 166.93996f64,
            Self::Tb168 => 167.9434f64,
            Self::Tb169 => 168.94597f64,
            Self::Tb170 => 169.94984f64,
            Self::Tb171 => 170.95273f64,
        }
    }
}
impl super::ElementVariant for TerbiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Tb
    }
}
impl super::MassNumber for TerbiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Tb135 => 135u16,
            Self::Tb136 => 136u16,
            Self::Tb137 => 137u16,
            Self::Tb138 => 138u16,
            Self::Tb139 => 139u16,
            Self::Tb140 => 140u16,
            Self::Tb141 => 141u16,
            Self::Tb142 => 142u16,
            Self::Tb143 => 143u16,
            Self::Tb144 => 144u16,
            Self::Tb145 => 145u16,
            Self::Tb146 => 146u16,
            Self::Tb147 => 147u16,
            Self::Tb148 => 148u16,
            Self::Tb149 => 149u16,
            Self::Tb150 => 150u16,
            Self::Tb151 => 151u16,
            Self::Tb152 => 152u16,
            Self::Tb153 => 153u16,
            Self::Tb154 => 154u16,
            Self::Tb155 => 155u16,
            Self::Tb156 => 156u16,
            Self::Tb157 => 157u16,
            Self::Tb158 => 158u16,
            Self::Tb159 => 159u16,
            Self::Tb160 => 160u16,
            Self::Tb161 => 161u16,
            Self::Tb162 => 162u16,
            Self::Tb163 => 163u16,
            Self::Tb164 => 164u16,
            Self::Tb165 => 165u16,
            Self::Tb166 => 166u16,
            Self::Tb167 => 167u16,
            Self::Tb168 => 168u16,
            Self::Tb169 => 169u16,
            Self::Tb170 => 170u16,
            Self::Tb171 => 171u16,
        }
    }
}
impl super::IsotopicComposition for TerbiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Tb159 => Some(1f64),
            Self::Tb135
            | Self::Tb136
            | Self::Tb137
            | Self::Tb138
            | Self::Tb139
            | Self::Tb140
            | Self::Tb141
            | Self::Tb142
            | Self::Tb143
            | Self::Tb144
            | Self::Tb145
            | Self::Tb146
            | Self::Tb147
            | Self::Tb148
            | Self::Tb149
            | Self::Tb150
            | Self::Tb151
            | Self::Tb152
            | Self::Tb153
            | Self::Tb154
            | Self::Tb155
            | Self::Tb156
            | Self::Tb157
            | Self::Tb158
            | Self::Tb160
            | Self::Tb161
            | Self::Tb162
            | Self::Tb163
            | Self::Tb164
            | Self::Tb165
            | Self::Tb166
            | Self::Tb167
            | Self::Tb168
            | Self::Tb169
            | Self::Tb170
            | Self::Tb171 => None,
        }
    }
}
impl super::MostAbundantIsotope for TerbiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Tb159
    }
}
impl TryFrom<u16> for TerbiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            135u16 => Ok(Self::Tb135),
            136u16 => Ok(Self::Tb136),
            137u16 => Ok(Self::Tb137),
            138u16 => Ok(Self::Tb138),
            139u16 => Ok(Self::Tb139),
            140u16 => Ok(Self::Tb140),
            141u16 => Ok(Self::Tb141),
            142u16 => Ok(Self::Tb142),
            143u16 => Ok(Self::Tb143),
            144u16 => Ok(Self::Tb144),
            145u16 => Ok(Self::Tb145),
            146u16 => Ok(Self::Tb146),
            147u16 => Ok(Self::Tb147),
            148u16 => Ok(Self::Tb148),
            149u16 => Ok(Self::Tb149),
            150u16 => Ok(Self::Tb150),
            151u16 => Ok(Self::Tb151),
            152u16 => Ok(Self::Tb152),
            153u16 => Ok(Self::Tb153),
            154u16 => Ok(Self::Tb154),
            155u16 => Ok(Self::Tb155),
            156u16 => Ok(Self::Tb156),
            157u16 => Ok(Self::Tb157),
            158u16 => Ok(Self::Tb158),
            159u16 => Ok(Self::Tb159),
            160u16 => Ok(Self::Tb160),
            161u16 => Ok(Self::Tb161),
            162u16 => Ok(Self::Tb162),
            163u16 => Ok(Self::Tb163),
            164u16 => Ok(Self::Tb164),
            165u16 => Ok(Self::Tb165),
            166u16 => Ok(Self::Tb166),
            167u16 => Ok(Self::Tb167),
            168u16 => Ok(Self::Tb168),
            169u16 => Ok(Self::Tb169),
            170u16 => Ok(Self::Tb170),
            171u16 => Ok(Self::Tb171),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Tb, value)),
        }
    }
}
impl std::fmt::Display for TerbiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tb135 => write!(f, "Tb135"),
            Self::Tb136 => write!(f, "Tb136"),
            Self::Tb137 => write!(f, "Tb137"),
            Self::Tb138 => write!(f, "Tb138"),
            Self::Tb139 => write!(f, "Tb139"),
            Self::Tb140 => write!(f, "Tb140"),
            Self::Tb141 => write!(f, "Tb141"),
            Self::Tb142 => write!(f, "Tb142"),
            Self::Tb143 => write!(f, "Tb143"),
            Self::Tb144 => write!(f, "Tb144"),
            Self::Tb145 => write!(f, "Tb145"),
            Self::Tb146 => write!(f, "Tb146"),
            Self::Tb147 => write!(f, "Tb147"),
            Self::Tb148 => write!(f, "Tb148"),
            Self::Tb149 => write!(f, "Tb149"),
            Self::Tb150 => write!(f, "Tb150"),
            Self::Tb151 => write!(f, "Tb151"),
            Self::Tb152 => write!(f, "Tb152"),
            Self::Tb153 => write!(f, "Tb153"),
            Self::Tb154 => write!(f, "Tb154"),
            Self::Tb155 => write!(f, "Tb155"),
            Self::Tb156 => write!(f, "Tb156"),
            Self::Tb157 => write!(f, "Tb157"),
            Self::Tb158 => write!(f, "Tb158"),
            Self::Tb159 => write!(f, "Tb159"),
            Self::Tb160 => write!(f, "Tb160"),
            Self::Tb161 => write!(f, "Tb161"),
            Self::Tb162 => write!(f, "Tb162"),
            Self::Tb163 => write!(f, "Tb163"),
            Self::Tb164 => write!(f, "Tb164"),
            Self::Tb165 => write!(f, "Tb165"),
            Self::Tb166 => write!(f, "Tb166"),
            Self::Tb167 => write!(f, "Tb167"),
            Self::Tb168 => write!(f, "Tb168"),
            Self::Tb169 => write!(f, "Tb169"),
            Self::Tb170 => write!(f, "Tb170"),
            Self::Tb171 => write!(f, "Tb171"),
        }
    }
}
