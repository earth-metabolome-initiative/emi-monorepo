//! Isotopes of the element Platinum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Platinum
pub enum PlatinumIsotope {
    /// Isotope Pt166 of Platinum
    Pt166,
    /// Isotope Pt167 of Platinum
    Pt167,
    /// Isotope Pt168 of Platinum
    Pt168,
    /// Isotope Pt169 of Platinum
    Pt169,
    /// Isotope Pt170 of Platinum
    Pt170,
    /// Isotope Pt171 of Platinum
    Pt171,
    /// Isotope Pt172 of Platinum
    Pt172,
    /// Isotope Pt173 of Platinum
    Pt173,
    /// Isotope Pt174 of Platinum
    Pt174,
    /// Isotope Pt175 of Platinum
    Pt175,
    /// Isotope Pt176 of Platinum
    Pt176,
    /// Isotope Pt177 of Platinum
    Pt177,
    /// Isotope Pt178 of Platinum
    Pt178,
    /// Isotope Pt179 of Platinum
    Pt179,
    /// Isotope Pt180 of Platinum
    Pt180,
    /// Isotope Pt181 of Platinum
    Pt181,
    /// Isotope Pt182 of Platinum
    Pt182,
    /// Isotope Pt183 of Platinum
    Pt183,
    /// Isotope Pt184 of Platinum
    Pt184,
    /// Isotope Pt185 of Platinum
    Pt185,
    /// Isotope Pt186 of Platinum
    Pt186,
    /// Isotope Pt187 of Platinum
    Pt187,
    /// Isotope Pt188 of Platinum
    Pt188,
    /// Isotope Pt189 of Platinum
    Pt189,
    /// Isotope Pt190 of Platinum
    Pt190,
    /// Isotope Pt191 of Platinum
    Pt191,
    /// Isotope Pt192 of Platinum
    Pt192,
    /// Isotope Pt193 of Platinum
    Pt193,
    /// Isotope Pt194 of Platinum
    Pt194,
    /// Isotope Pt195 of Platinum
    Pt195,
    /// Isotope Pt196 of Platinum
    Pt196,
    /// Isotope Pt197 of Platinum
    Pt197,
    /// Isotope Pt198 of Platinum
    Pt198,
    /// Isotope Pt199 of Platinum
    Pt199,
    /// Isotope Pt200 of Platinum
    Pt200,
    /// Isotope Pt201 of Platinum
    Pt201,
    /// Isotope Pt202 of Platinum
    Pt202,
    /// Isotope Pt203 of Platinum
    Pt203,
    /// Isotope Pt204 of Platinum
    Pt204,
    /// Isotope Pt205 of Platinum
    Pt205,
    /// Isotope Pt206 of Platinum
    Pt206,
}
impl super::RelativeAtomicMass for PlatinumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Pt166 => 165.99486f64,
            Self::Pt167 => 166.99269f64,
            Self::Pt168 => 167.98813f64,
            Self::Pt169 => 168.98657f64,
            Self::Pt170 => 169.982496f64,
            Self::Pt171 => 170.981245f64,
            Self::Pt172 => 171.977351f64,
            Self::Pt173 => 172.976443f64,
            Self::Pt174 => 173.97282f64,
            Self::Pt175 => 174.97241f64,
            Self::Pt176 => 175.968938f64,
            Self::Pt177 => 176.96847f64,
            Self::Pt178 => 177.96565f64,
            Self::Pt179 => 178.965359f64,
            Self::Pt180 => 179.963032f64,
            Self::Pt181 => 180.963098f64,
            Self::Pt182 => 181.961172f64,
            Self::Pt183 => 182.961597f64,
            Self::Pt184 => 183.959915f64,
            Self::Pt185 => 184.960614f64,
            Self::Pt186 => 185.959351f64,
            Self::Pt187 => 186.960617f64,
            Self::Pt188 => 187.9593889f64,
            Self::Pt189 => 188.960831f64,
            Self::Pt190 => 189.9599297f64,
            Self::Pt191 => 190.9616729f64,
            Self::Pt192 => 191.9610387f64,
            Self::Pt193 => 192.9629824f64,
            Self::Pt194 => 193.9626809f64,
            Self::Pt195 => 194.9647917f64,
            Self::Pt196 => 195.96495209f64,
            Self::Pt197 => 196.96734069f64,
            Self::Pt198 => 197.9678949f64,
            Self::Pt199 => 198.9705952f64,
            Self::Pt200 => 199.971443f64,
            Self::Pt201 => 200.974513f64,
            Self::Pt202 => 201.975639f64,
            Self::Pt203 => 202.97893f64,
            Self::Pt204 => 203.98076f64,
            Self::Pt205 => 204.98608f64,
            Self::Pt206 => 205.98966f64,
        }
    }
}
impl super::ElementVariant for PlatinumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Pt
    }
}
impl super::MassNumber for PlatinumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Pt166 => 166u16,
            Self::Pt167 => 167u16,
            Self::Pt168 => 168u16,
            Self::Pt169 => 169u16,
            Self::Pt170 => 170u16,
            Self::Pt171 => 171u16,
            Self::Pt172 => 172u16,
            Self::Pt173 => 173u16,
            Self::Pt174 => 174u16,
            Self::Pt175 => 175u16,
            Self::Pt176 => 176u16,
            Self::Pt177 => 177u16,
            Self::Pt178 => 178u16,
            Self::Pt179 => 179u16,
            Self::Pt180 => 180u16,
            Self::Pt181 => 181u16,
            Self::Pt182 => 182u16,
            Self::Pt183 => 183u16,
            Self::Pt184 => 184u16,
            Self::Pt185 => 185u16,
            Self::Pt186 => 186u16,
            Self::Pt187 => 187u16,
            Self::Pt188 => 188u16,
            Self::Pt189 => 189u16,
            Self::Pt190 => 190u16,
            Self::Pt191 => 191u16,
            Self::Pt192 => 192u16,
            Self::Pt193 => 193u16,
            Self::Pt194 => 194u16,
            Self::Pt195 => 195u16,
            Self::Pt196 => 196u16,
            Self::Pt197 => 197u16,
            Self::Pt198 => 198u16,
            Self::Pt199 => 199u16,
            Self::Pt200 => 200u16,
            Self::Pt201 => 201u16,
            Self::Pt202 => 202u16,
            Self::Pt203 => 203u16,
            Self::Pt204 => 204u16,
            Self::Pt205 => 205u16,
            Self::Pt206 => 206u16,
        }
    }
}
impl super::IsotopicComposition for PlatinumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Pt190 => Some(0.00012f64),
            Self::Pt192 => Some(0.00782f64),
            Self::Pt194 => Some(0.3286f64),
            Self::Pt195 => Some(0.3378f64),
            Self::Pt196 => Some(0.2521f64),
            Self::Pt198 => Some(0.07356f64),
            Self::Pt166
            | Self::Pt167
            | Self::Pt168
            | Self::Pt169
            | Self::Pt170
            | Self::Pt171
            | Self::Pt172
            | Self::Pt173
            | Self::Pt174
            | Self::Pt175
            | Self::Pt176
            | Self::Pt177
            | Self::Pt178
            | Self::Pt179
            | Self::Pt180
            | Self::Pt181
            | Self::Pt182
            | Self::Pt183
            | Self::Pt184
            | Self::Pt185
            | Self::Pt186
            | Self::Pt187
            | Self::Pt188
            | Self::Pt189
            | Self::Pt191
            | Self::Pt193
            | Self::Pt197
            | Self::Pt199
            | Self::Pt200
            | Self::Pt201
            | Self::Pt202
            | Self::Pt203
            | Self::Pt204
            | Self::Pt205
            | Self::Pt206 => None,
        }
    }
}
impl super::MostAbundantIsotope for PlatinumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Pt195
    }
}
impl TryFrom<u16> for PlatinumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            166u16 => Ok(Self::Pt166),
            167u16 => Ok(Self::Pt167),
            168u16 => Ok(Self::Pt168),
            169u16 => Ok(Self::Pt169),
            170u16 => Ok(Self::Pt170),
            171u16 => Ok(Self::Pt171),
            172u16 => Ok(Self::Pt172),
            173u16 => Ok(Self::Pt173),
            174u16 => Ok(Self::Pt174),
            175u16 => Ok(Self::Pt175),
            176u16 => Ok(Self::Pt176),
            177u16 => Ok(Self::Pt177),
            178u16 => Ok(Self::Pt178),
            179u16 => Ok(Self::Pt179),
            180u16 => Ok(Self::Pt180),
            181u16 => Ok(Self::Pt181),
            182u16 => Ok(Self::Pt182),
            183u16 => Ok(Self::Pt183),
            184u16 => Ok(Self::Pt184),
            185u16 => Ok(Self::Pt185),
            186u16 => Ok(Self::Pt186),
            187u16 => Ok(Self::Pt187),
            188u16 => Ok(Self::Pt188),
            189u16 => Ok(Self::Pt189),
            190u16 => Ok(Self::Pt190),
            191u16 => Ok(Self::Pt191),
            192u16 => Ok(Self::Pt192),
            193u16 => Ok(Self::Pt193),
            194u16 => Ok(Self::Pt194),
            195u16 => Ok(Self::Pt195),
            196u16 => Ok(Self::Pt196),
            197u16 => Ok(Self::Pt197),
            198u16 => Ok(Self::Pt198),
            199u16 => Ok(Self::Pt199),
            200u16 => Ok(Self::Pt200),
            201u16 => Ok(Self::Pt201),
            202u16 => Ok(Self::Pt202),
            203u16 => Ok(Self::Pt203),
            204u16 => Ok(Self::Pt204),
            205u16 => Ok(Self::Pt205),
            206u16 => Ok(Self::Pt206),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Pt, value)),
        }
    }
}
impl std::fmt::Display for PlatinumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pt166 => write!(f, "Pt166"),
            Self::Pt167 => write!(f, "Pt167"),
            Self::Pt168 => write!(f, "Pt168"),
            Self::Pt169 => write!(f, "Pt169"),
            Self::Pt170 => write!(f, "Pt170"),
            Self::Pt171 => write!(f, "Pt171"),
            Self::Pt172 => write!(f, "Pt172"),
            Self::Pt173 => write!(f, "Pt173"),
            Self::Pt174 => write!(f, "Pt174"),
            Self::Pt175 => write!(f, "Pt175"),
            Self::Pt176 => write!(f, "Pt176"),
            Self::Pt177 => write!(f, "Pt177"),
            Self::Pt178 => write!(f, "Pt178"),
            Self::Pt179 => write!(f, "Pt179"),
            Self::Pt180 => write!(f, "Pt180"),
            Self::Pt181 => write!(f, "Pt181"),
            Self::Pt182 => write!(f, "Pt182"),
            Self::Pt183 => write!(f, "Pt183"),
            Self::Pt184 => write!(f, "Pt184"),
            Self::Pt185 => write!(f, "Pt185"),
            Self::Pt186 => write!(f, "Pt186"),
            Self::Pt187 => write!(f, "Pt187"),
            Self::Pt188 => write!(f, "Pt188"),
            Self::Pt189 => write!(f, "Pt189"),
            Self::Pt190 => write!(f, "Pt190"),
            Self::Pt191 => write!(f, "Pt191"),
            Self::Pt192 => write!(f, "Pt192"),
            Self::Pt193 => write!(f, "Pt193"),
            Self::Pt194 => write!(f, "Pt194"),
            Self::Pt195 => write!(f, "Pt195"),
            Self::Pt196 => write!(f, "Pt196"),
            Self::Pt197 => write!(f, "Pt197"),
            Self::Pt198 => write!(f, "Pt198"),
            Self::Pt199 => write!(f, "Pt199"),
            Self::Pt200 => write!(f, "Pt200"),
            Self::Pt201 => write!(f, "Pt201"),
            Self::Pt202 => write!(f, "Pt202"),
            Self::Pt203 => write!(f, "Pt203"),
            Self::Pt204 => write!(f, "Pt204"),
            Self::Pt205 => write!(f, "Pt205"),
            Self::Pt206 => write!(f, "Pt206"),
        }
    }
}
