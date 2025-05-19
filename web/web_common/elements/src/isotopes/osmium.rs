//! Isotopes of the element Osmium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Osmium
pub enum OsmiumIsotope {
    /// Isotope Os161 of Osmium
    Os161,
    /// Isotope Os162 of Osmium
    Os162,
    /// Isotope Os163 of Osmium
    Os163,
    /// Isotope Os164 of Osmium
    Os164,
    /// Isotope Os165 of Osmium
    Os165,
    /// Isotope Os166 of Osmium
    Os166,
    /// Isotope Os167 of Osmium
    Os167,
    /// Isotope Os168 of Osmium
    Os168,
    /// Isotope Os169 of Osmium
    Os169,
    /// Isotope Os170 of Osmium
    Os170,
    /// Isotope Os171 of Osmium
    Os171,
    /// Isotope Os172 of Osmium
    Os172,
    /// Isotope Os173 of Osmium
    Os173,
    /// Isotope Os174 of Osmium
    Os174,
    /// Isotope Os175 of Osmium
    Os175,
    /// Isotope Os176 of Osmium
    Os176,
    /// Isotope Os177 of Osmium
    Os177,
    /// Isotope Os178 of Osmium
    Os178,
    /// Isotope Os179 of Osmium
    Os179,
    /// Isotope Os180 of Osmium
    Os180,
    /// Isotope Os181 of Osmium
    Os181,
    /// Isotope Os182 of Osmium
    Os182,
    /// Isotope Os183 of Osmium
    Os183,
    /// Isotope Os184 of Osmium
    Os184,
    /// Isotope Os185 of Osmium
    Os185,
    /// Isotope Os186 of Osmium
    Os186,
    /// Isotope Os187 of Osmium
    Os187,
    /// Isotope Os188 of Osmium
    Os188,
    /// Isotope Os189 of Osmium
    Os189,
    /// Isotope Os190 of Osmium
    Os190,
    /// Isotope Os191 of Osmium
    Os191,
    /// Isotope Os192 of Osmium
    Os192,
    /// Isotope Os193 of Osmium
    Os193,
    /// Isotope Os194 of Osmium
    Os194,
    /// Isotope Os195 of Osmium
    Os195,
    /// Isotope Os196 of Osmium
    Os196,
    /// Isotope Os197 of Osmium
    Os197,
    /// Isotope Os198 of Osmium
    Os198,
    /// Isotope Os199 of Osmium
    Os199,
    /// Isotope Os200 of Osmium
    Os200,
    /// Isotope Os201 of Osmium
    Os201,
    /// Isotope Os202 of Osmium
    Os202,
}
impl super::RelativeAtomicMass for OsmiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Os161 => 160.98903f64,
            Self::Os162 => 161.98443f64,
            Self::Os163 => 162.98241f64,
            Self::Os164 => 163.97802f64,
            Self::Os165 => 164.9766f64,
            Self::Os166 => 165.972692f64,
            Self::Os167 => 166.971549f64,
            Self::Os168 => 167.967808f64,
            Self::Os169 => 168.967018f64,
            Self::Os170 => 169.963578f64,
            Self::Os171 => 170.963174f64,
            Self::Os172 => 171.960017f64,
            Self::Os173 => 172.959808f64,
            Self::Os174 => 173.957064f64,
            Self::Os175 => 174.956945f64,
            Self::Os176 => 175.954806f64,
            Self::Os177 => 176.954966f64,
            Self::Os178 => 177.953254f64,
            Self::Os179 => 178.953817f64,
            Self::Os180 => 179.952375f64,
            Self::Os181 => 180.953247f64,
            Self::Os182 => 181.95211f64,
            Self::Os183 => 182.953125f64,
            Self::Os184 => 183.9524885f64,
            Self::Os185 => 184.9540417f64,
            Self::Os186 => 185.953835f64,
            Self::Os187 => 186.9557474f64,
            Self::Os188 => 187.9558352f64,
            Self::Os189 => 188.9581442f64,
            Self::Os190 => 189.9584437f64,
            Self::Os191 => 190.9609264f64,
            Self::Os192 => 191.961477f64,
            Self::Os193 => 192.9641479f64,
            Self::Os194 => 193.9651772f64,
            Self::Os195 => 194.968318f64,
            Self::Os196 => 195.969641f64,
            Self::Os197 => 196.97283f64,
            Self::Os198 => 197.97441f64,
            Self::Os199 => 198.97801f64,
            Self::Os200 => 199.97984f64,
            Self::Os201 => 200.98364f64,
            Self::Os202 => 201.98595f64,
        }
    }
}
impl super::ElementVariant for OsmiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Os
    }
}
impl super::MassNumber for OsmiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Os161 => 161u16,
            Self::Os162 => 162u16,
            Self::Os163 => 163u16,
            Self::Os164 => 164u16,
            Self::Os165 => 165u16,
            Self::Os166 => 166u16,
            Self::Os167 => 167u16,
            Self::Os168 => 168u16,
            Self::Os169 => 169u16,
            Self::Os170 => 170u16,
            Self::Os171 => 171u16,
            Self::Os172 => 172u16,
            Self::Os173 => 173u16,
            Self::Os174 => 174u16,
            Self::Os175 => 175u16,
            Self::Os176 => 176u16,
            Self::Os177 => 177u16,
            Self::Os178 => 178u16,
            Self::Os179 => 179u16,
            Self::Os180 => 180u16,
            Self::Os181 => 181u16,
            Self::Os182 => 182u16,
            Self::Os183 => 183u16,
            Self::Os184 => 184u16,
            Self::Os185 => 185u16,
            Self::Os186 => 186u16,
            Self::Os187 => 187u16,
            Self::Os188 => 188u16,
            Self::Os189 => 189u16,
            Self::Os190 => 190u16,
            Self::Os191 => 191u16,
            Self::Os192 => 192u16,
            Self::Os193 => 193u16,
            Self::Os194 => 194u16,
            Self::Os195 => 195u16,
            Self::Os196 => 196u16,
            Self::Os197 => 197u16,
            Self::Os198 => 198u16,
            Self::Os199 => 199u16,
            Self::Os200 => 200u16,
            Self::Os201 => 201u16,
            Self::Os202 => 202u16,
        }
    }
}
impl super::IsotopicComposition for OsmiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Os184 => Some(0.0002f64),
            Self::Os186 => Some(0.0159f64),
            Self::Os187 => Some(0.0196f64),
            Self::Os188 => Some(0.1324f64),
            Self::Os189 => Some(0.1615f64),
            Self::Os190 => Some(0.2626f64),
            Self::Os192 => Some(0.4078f64),
            Self::Os161
            | Self::Os162
            | Self::Os163
            | Self::Os164
            | Self::Os165
            | Self::Os166
            | Self::Os167
            | Self::Os168
            | Self::Os169
            | Self::Os170
            | Self::Os171
            | Self::Os172
            | Self::Os173
            | Self::Os174
            | Self::Os175
            | Self::Os176
            | Self::Os177
            | Self::Os178
            | Self::Os179
            | Self::Os180
            | Self::Os181
            | Self::Os182
            | Self::Os183
            | Self::Os185
            | Self::Os191
            | Self::Os193
            | Self::Os194
            | Self::Os195
            | Self::Os196
            | Self::Os197
            | Self::Os198
            | Self::Os199
            | Self::Os200
            | Self::Os201
            | Self::Os202 => None,
        }
    }
}
impl super::MostAbundantIsotope for OsmiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Os192
    }
}
impl TryFrom<u16> for OsmiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            161u16 => Ok(Self::Os161),
            162u16 => Ok(Self::Os162),
            163u16 => Ok(Self::Os163),
            164u16 => Ok(Self::Os164),
            165u16 => Ok(Self::Os165),
            166u16 => Ok(Self::Os166),
            167u16 => Ok(Self::Os167),
            168u16 => Ok(Self::Os168),
            169u16 => Ok(Self::Os169),
            170u16 => Ok(Self::Os170),
            171u16 => Ok(Self::Os171),
            172u16 => Ok(Self::Os172),
            173u16 => Ok(Self::Os173),
            174u16 => Ok(Self::Os174),
            175u16 => Ok(Self::Os175),
            176u16 => Ok(Self::Os176),
            177u16 => Ok(Self::Os177),
            178u16 => Ok(Self::Os178),
            179u16 => Ok(Self::Os179),
            180u16 => Ok(Self::Os180),
            181u16 => Ok(Self::Os181),
            182u16 => Ok(Self::Os182),
            183u16 => Ok(Self::Os183),
            184u16 => Ok(Self::Os184),
            185u16 => Ok(Self::Os185),
            186u16 => Ok(Self::Os186),
            187u16 => Ok(Self::Os187),
            188u16 => Ok(Self::Os188),
            189u16 => Ok(Self::Os189),
            190u16 => Ok(Self::Os190),
            191u16 => Ok(Self::Os191),
            192u16 => Ok(Self::Os192),
            193u16 => Ok(Self::Os193),
            194u16 => Ok(Self::Os194),
            195u16 => Ok(Self::Os195),
            196u16 => Ok(Self::Os196),
            197u16 => Ok(Self::Os197),
            198u16 => Ok(Self::Os198),
            199u16 => Ok(Self::Os199),
            200u16 => Ok(Self::Os200),
            201u16 => Ok(Self::Os201),
            202u16 => Ok(Self::Os202),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Os, value)),
        }
    }
}
impl std::fmt::Display for OsmiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Os161 => write!(f, "Os161"),
            Self::Os162 => write!(f, "Os162"),
            Self::Os163 => write!(f, "Os163"),
            Self::Os164 => write!(f, "Os164"),
            Self::Os165 => write!(f, "Os165"),
            Self::Os166 => write!(f, "Os166"),
            Self::Os167 => write!(f, "Os167"),
            Self::Os168 => write!(f, "Os168"),
            Self::Os169 => write!(f, "Os169"),
            Self::Os170 => write!(f, "Os170"),
            Self::Os171 => write!(f, "Os171"),
            Self::Os172 => write!(f, "Os172"),
            Self::Os173 => write!(f, "Os173"),
            Self::Os174 => write!(f, "Os174"),
            Self::Os175 => write!(f, "Os175"),
            Self::Os176 => write!(f, "Os176"),
            Self::Os177 => write!(f, "Os177"),
            Self::Os178 => write!(f, "Os178"),
            Self::Os179 => write!(f, "Os179"),
            Self::Os180 => write!(f, "Os180"),
            Self::Os181 => write!(f, "Os181"),
            Self::Os182 => write!(f, "Os182"),
            Self::Os183 => write!(f, "Os183"),
            Self::Os184 => write!(f, "Os184"),
            Self::Os185 => write!(f, "Os185"),
            Self::Os186 => write!(f, "Os186"),
            Self::Os187 => write!(f, "Os187"),
            Self::Os188 => write!(f, "Os188"),
            Self::Os189 => write!(f, "Os189"),
            Self::Os190 => write!(f, "Os190"),
            Self::Os191 => write!(f, "Os191"),
            Self::Os192 => write!(f, "Os192"),
            Self::Os193 => write!(f, "Os193"),
            Self::Os194 => write!(f, "Os194"),
            Self::Os195 => write!(f, "Os195"),
            Self::Os196 => write!(f, "Os196"),
            Self::Os197 => write!(f, "Os197"),
            Self::Os198 => write!(f, "Os198"),
            Self::Os199 => write!(f, "Os199"),
            Self::Os200 => write!(f, "Os200"),
            Self::Os201 => write!(f, "Os201"),
            Self::Os202 => write!(f, "Os202"),
        }
    }
}
