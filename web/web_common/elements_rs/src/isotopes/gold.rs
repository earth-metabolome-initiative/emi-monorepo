//! Isotopes of the element Gold
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Gold
pub enum GoldIsotope {
    /// Isotope Au169 of Gold
    Au169,
    /// Isotope Au170 of Gold
    Au170,
    /// Isotope Au171 of Gold
    Au171,
    /// Isotope Au172 of Gold
    Au172,
    /// Isotope Au173 of Gold
    Au173,
    /// Isotope Au174 of Gold
    Au174,
    /// Isotope Au175 of Gold
    Au175,
    /// Isotope Au176 of Gold
    Au176,
    /// Isotope Au177 of Gold
    Au177,
    /// Isotope Au178 of Gold
    Au178,
    /// Isotope Au179 of Gold
    Au179,
    /// Isotope Au180 of Gold
    Au180,
    /// Isotope Au181 of Gold
    Au181,
    /// Isotope Au182 of Gold
    Au182,
    /// Isotope Au183 of Gold
    Au183,
    /// Isotope Au184 of Gold
    Au184,
    /// Isotope Au185 of Gold
    Au185,
    /// Isotope Au186 of Gold
    Au186,
    /// Isotope Au187 of Gold
    Au187,
    /// Isotope Au188 of Gold
    Au188,
    /// Isotope Au189 of Gold
    Au189,
    /// Isotope Au190 of Gold
    Au190,
    /// Isotope Au191 of Gold
    Au191,
    /// Isotope Au192 of Gold
    Au192,
    /// Isotope Au193 of Gold
    Au193,
    /// Isotope Au194 of Gold
    Au194,
    /// Isotope Au195 of Gold
    Au195,
    /// Isotope Au196 of Gold
    Au196,
    /// Isotope Au197 of Gold
    Au197,
    /// Isotope Au198 of Gold
    Au198,
    /// Isotope Au199 of Gold
    Au199,
    /// Isotope Au200 of Gold
    Au200,
    /// Isotope Au201 of Gold
    Au201,
    /// Isotope Au202 of Gold
    Au202,
    /// Isotope Au203 of Gold
    Au203,
    /// Isotope Au204 of Gold
    Au204,
    /// Isotope Au205 of Gold
    Au205,
    /// Isotope Au206 of Gold
    Au206,
    /// Isotope Au207 of Gold
    Au207,
    /// Isotope Au208 of Gold
    Au208,
    /// Isotope Au209 of Gold
    Au209,
    /// Isotope Au210 of Gold
    Au210,
}
impl super::RelativeAtomicMass for GoldIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Au169 => 168.99808f64,
            Self::Au170 => 169.99597f64,
            Self::Au171 => 170.991876f64,
            Self::Au172 => 171.989942f64,
            Self::Au173 => 172.986241f64,
            Self::Au174 => 173.984717f64,
            Self::Au175 => 174.981304f64,
            Self::Au176 => 175.98025f64,
            Self::Au177 => 176.97687f64,
            Self::Au178 => 177.976032f64,
            Self::Au179 => 178.973174f64,
            Self::Au180 => 179.972523f64,
            Self::Au181 => 180.970079f64,
            Self::Au182 => 181.969618f64,
            Self::Au183 => 182.967591f64,
            Self::Au184 => 183.967452f64,
            Self::Au185 => 184.96579f64,
            Self::Au186 => 185.965953f64,
            Self::Au187 => 186.964543f64,
            Self::Au188 => 187.965349f64,
            Self::Au189 => 188.963948f64,
            Self::Au190 => 189.964698f64,
            Self::Au191 => 190.963702f64,
            Self::Au192 => 191.964814f64,
            Self::Au193 => 192.9641373f64,
            Self::Au194 => 193.9654178f64,
            Self::Au195 => 194.9650352f64,
            Self::Au196 => 195.9665699f64,
            Self::Au197 => 196.96656879f64,
            Self::Au198 => 197.96824242f64,
            Self::Au199 => 198.96876528f64,
            Self::Au200 => 199.970756f64,
            Self::Au201 => 200.9716575f64,
            Self::Au202 => 201.973856f64,
            Self::Au203 => 202.9751544f64,
            Self::Au204 => 203.97783f64,
            Self::Au205 => 204.97985f64,
            Self::Au206 => 205.98474f64,
            Self::Au207 => 206.9884f64,
            Self::Au208 => 207.99345f64,
            Self::Au209 => 208.99735f64,
            Self::Au210 => 210.0025f64,
        }
    }
}
impl super::ElementVariant for GoldIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Au
    }
}
impl super::MassNumber for GoldIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Au169 => 169u16,
            Self::Au170 => 170u16,
            Self::Au171 => 171u16,
            Self::Au172 => 172u16,
            Self::Au173 => 173u16,
            Self::Au174 => 174u16,
            Self::Au175 => 175u16,
            Self::Au176 => 176u16,
            Self::Au177 => 177u16,
            Self::Au178 => 178u16,
            Self::Au179 => 179u16,
            Self::Au180 => 180u16,
            Self::Au181 => 181u16,
            Self::Au182 => 182u16,
            Self::Au183 => 183u16,
            Self::Au184 => 184u16,
            Self::Au185 => 185u16,
            Self::Au186 => 186u16,
            Self::Au187 => 187u16,
            Self::Au188 => 188u16,
            Self::Au189 => 189u16,
            Self::Au190 => 190u16,
            Self::Au191 => 191u16,
            Self::Au192 => 192u16,
            Self::Au193 => 193u16,
            Self::Au194 => 194u16,
            Self::Au195 => 195u16,
            Self::Au196 => 196u16,
            Self::Au197 => 197u16,
            Self::Au198 => 198u16,
            Self::Au199 => 199u16,
            Self::Au200 => 200u16,
            Self::Au201 => 201u16,
            Self::Au202 => 202u16,
            Self::Au203 => 203u16,
            Self::Au204 => 204u16,
            Self::Au205 => 205u16,
            Self::Au206 => 206u16,
            Self::Au207 => 207u16,
            Self::Au208 => 208u16,
            Self::Au209 => 209u16,
            Self::Au210 => 210u16,
        }
    }
}
impl super::IsotopicComposition for GoldIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Au197 => Some(1f64),
            Self::Au169
            | Self::Au170
            | Self::Au171
            | Self::Au172
            | Self::Au173
            | Self::Au174
            | Self::Au175
            | Self::Au176
            | Self::Au177
            | Self::Au178
            | Self::Au179
            | Self::Au180
            | Self::Au181
            | Self::Au182
            | Self::Au183
            | Self::Au184
            | Self::Au185
            | Self::Au186
            | Self::Au187
            | Self::Au188
            | Self::Au189
            | Self::Au190
            | Self::Au191
            | Self::Au192
            | Self::Au193
            | Self::Au194
            | Self::Au195
            | Self::Au196
            | Self::Au198
            | Self::Au199
            | Self::Au200
            | Self::Au201
            | Self::Au202
            | Self::Au203
            | Self::Au204
            | Self::Au205
            | Self::Au206
            | Self::Au207
            | Self::Au208
            | Self::Au209
            | Self::Au210 => None,
        }
    }
}
impl super::MostAbundantIsotope for GoldIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Au197
    }
}
impl From<GoldIsotope> for crate::Isotope {
    fn from(isotope: GoldIsotope) -> Self {
        crate::Isotope::Au(isotope)
    }
}
impl From<GoldIsotope> for crate::Element {
    fn from(_isotope: GoldIsotope) -> Self {
        crate::Element::Au
    }
}
impl TryFrom<u16> for GoldIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            169u16 => Ok(Self::Au169),
            170u16 => Ok(Self::Au170),
            171u16 => Ok(Self::Au171),
            172u16 => Ok(Self::Au172),
            173u16 => Ok(Self::Au173),
            174u16 => Ok(Self::Au174),
            175u16 => Ok(Self::Au175),
            176u16 => Ok(Self::Au176),
            177u16 => Ok(Self::Au177),
            178u16 => Ok(Self::Au178),
            179u16 => Ok(Self::Au179),
            180u16 => Ok(Self::Au180),
            181u16 => Ok(Self::Au181),
            182u16 => Ok(Self::Au182),
            183u16 => Ok(Self::Au183),
            184u16 => Ok(Self::Au184),
            185u16 => Ok(Self::Au185),
            186u16 => Ok(Self::Au186),
            187u16 => Ok(Self::Au187),
            188u16 => Ok(Self::Au188),
            189u16 => Ok(Self::Au189),
            190u16 => Ok(Self::Au190),
            191u16 => Ok(Self::Au191),
            192u16 => Ok(Self::Au192),
            193u16 => Ok(Self::Au193),
            194u16 => Ok(Self::Au194),
            195u16 => Ok(Self::Au195),
            196u16 => Ok(Self::Au196),
            197u16 => Ok(Self::Au197),
            198u16 => Ok(Self::Au198),
            199u16 => Ok(Self::Au199),
            200u16 => Ok(Self::Au200),
            201u16 => Ok(Self::Au201),
            202u16 => Ok(Self::Au202),
            203u16 => Ok(Self::Au203),
            204u16 => Ok(Self::Au204),
            205u16 => Ok(Self::Au205),
            206u16 => Ok(Self::Au206),
            207u16 => Ok(Self::Au207),
            208u16 => Ok(Self::Au208),
            209u16 => Ok(Self::Au209),
            210u16 => Ok(Self::Au210),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Au, value)),
        }
    }
}
impl std::fmt::Display for GoldIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Au169 => write!(f, "Au169"),
            Self::Au170 => write!(f, "Au170"),
            Self::Au171 => write!(f, "Au171"),
            Self::Au172 => write!(f, "Au172"),
            Self::Au173 => write!(f, "Au173"),
            Self::Au174 => write!(f, "Au174"),
            Self::Au175 => write!(f, "Au175"),
            Self::Au176 => write!(f, "Au176"),
            Self::Au177 => write!(f, "Au177"),
            Self::Au178 => write!(f, "Au178"),
            Self::Au179 => write!(f, "Au179"),
            Self::Au180 => write!(f, "Au180"),
            Self::Au181 => write!(f, "Au181"),
            Self::Au182 => write!(f, "Au182"),
            Self::Au183 => write!(f, "Au183"),
            Self::Au184 => write!(f, "Au184"),
            Self::Au185 => write!(f, "Au185"),
            Self::Au186 => write!(f, "Au186"),
            Self::Au187 => write!(f, "Au187"),
            Self::Au188 => write!(f, "Au188"),
            Self::Au189 => write!(f, "Au189"),
            Self::Au190 => write!(f, "Au190"),
            Self::Au191 => write!(f, "Au191"),
            Self::Au192 => write!(f, "Au192"),
            Self::Au193 => write!(f, "Au193"),
            Self::Au194 => write!(f, "Au194"),
            Self::Au195 => write!(f, "Au195"),
            Self::Au196 => write!(f, "Au196"),
            Self::Au197 => write!(f, "Au197"),
            Self::Au198 => write!(f, "Au198"),
            Self::Au199 => write!(f, "Au199"),
            Self::Au200 => write!(f, "Au200"),
            Self::Au201 => write!(f, "Au201"),
            Self::Au202 => write!(f, "Au202"),
            Self::Au203 => write!(f, "Au203"),
            Self::Au204 => write!(f, "Au204"),
            Self::Au205 => write!(f, "Au205"),
            Self::Au206 => write!(f, "Au206"),
            Self::Au207 => write!(f, "Au207"),
            Self::Au208 => write!(f, "Au208"),
            Self::Au209 => write!(f, "Au209"),
            Self::Au210 => write!(f, "Au210"),
        }
    }
}
