//! Isotopes of the element Mercury
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Mercury
pub enum MercuryIsotope {
    /// Isotope Hg171 of Mercury
    Hg171,
    /// Isotope Hg172 of Mercury
    Hg172,
    /// Isotope Hg173 of Mercury
    Hg173,
    /// Isotope Hg174 of Mercury
    Hg174,
    /// Isotope Hg175 of Mercury
    Hg175,
    /// Isotope Hg176 of Mercury
    Hg176,
    /// Isotope Hg177 of Mercury
    Hg177,
    /// Isotope Hg178 of Mercury
    Hg178,
    /// Isotope Hg179 of Mercury
    Hg179,
    /// Isotope Hg180 of Mercury
    Hg180,
    /// Isotope Hg181 of Mercury
    Hg181,
    /// Isotope Hg182 of Mercury
    Hg182,
    /// Isotope Hg183 of Mercury
    Hg183,
    /// Isotope Hg184 of Mercury
    Hg184,
    /// Isotope Hg185 of Mercury
    Hg185,
    /// Isotope Hg186 of Mercury
    Hg186,
    /// Isotope Hg187 of Mercury
    Hg187,
    /// Isotope Hg188 of Mercury
    Hg188,
    /// Isotope Hg189 of Mercury
    Hg189,
    /// Isotope Hg190 of Mercury
    Hg190,
    /// Isotope Hg191 of Mercury
    Hg191,
    /// Isotope Hg192 of Mercury
    Hg192,
    /// Isotope Hg193 of Mercury
    Hg193,
    /// Isotope Hg194 of Mercury
    Hg194,
    /// Isotope Hg195 of Mercury
    Hg195,
    /// Isotope Hg196 of Mercury
    Hg196,
    /// Isotope Hg197 of Mercury
    Hg197,
    /// Isotope Hg198 of Mercury
    Hg198,
    /// Isotope Hg199 of Mercury
    Hg199,
    /// Isotope Hg200 of Mercury
    Hg200,
    /// Isotope Hg201 of Mercury
    Hg201,
    /// Isotope Hg202 of Mercury
    Hg202,
    /// Isotope Hg203 of Mercury
    Hg203,
    /// Isotope Hg204 of Mercury
    Hg204,
    /// Isotope Hg205 of Mercury
    Hg205,
    /// Isotope Hg206 of Mercury
    Hg206,
    /// Isotope Hg207 of Mercury
    Hg207,
    /// Isotope Hg208 of Mercury
    Hg208,
    /// Isotope Hg209 of Mercury
    Hg209,
    /// Isotope Hg210 of Mercury
    Hg210,
    /// Isotope Hg211 of Mercury
    Hg211,
    /// Isotope Hg212 of Mercury
    Hg212,
    /// Isotope Hg213 of Mercury
    Hg213,
    /// Isotope Hg214 of Mercury
    Hg214,
    /// Isotope Hg215 of Mercury
    Hg215,
    /// Isotope Hg216 of Mercury
    Hg216,
}
impl super::RelativeAtomicMass for MercuryIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Hg171 => 171.00353f64,
            Self::Hg172 => 171.99881f64,
            Self::Hg173 => 172.99709f64,
            Self::Hg174 => 173.992865f64,
            Self::Hg175 => 174.991441f64,
            Self::Hg176 => 175.987361f64,
            Self::Hg177 => 176.986277f64,
            Self::Hg178 => 177.982484f64,
            Self::Hg179 => 178.981831f64,
            Self::Hg180 => 179.97826f64,
            Self::Hg181 => 180.977819f64,
            Self::Hg182 => 181.974689f64,
            Self::Hg183 => 182.9744448f64,
            Self::Hg184 => 183.971714f64,
            Self::Hg185 => 184.971899f64,
            Self::Hg186 => 185.969362f64,
            Self::Hg187 => 186.969814f64,
            Self::Hg188 => 187.967567f64,
            Self::Hg189 => 188.968195f64,
            Self::Hg190 => 189.966323f64,
            Self::Hg191 => 190.967157f64,
            Self::Hg192 => 191.965635f64,
            Self::Hg193 => 192.966653f64,
            Self::Hg194 => 193.9654491f64,
            Self::Hg195 => 194.966721f64,
            Self::Hg196 => 195.9658326f64,
            Self::Hg197 => 196.9672128f64,
            Self::Hg198 => 197.9667686f64,
            Self::Hg199 => 198.96828064f64,
            Self::Hg200 => 199.96832659f64,
            Self::Hg201 => 200.97030284f64,
            Self::Hg202 => 201.9706434f64,
            Self::Hg203 => 202.9728728f64,
            Self::Hg204 => 203.97349398f64,
            Self::Hg205 => 204.9760734f64,
            Self::Hg206 => 205.977514f64,
            Self::Hg207 => 206.9823f64,
            Self::Hg208 => 207.985759f64,
            Self::Hg209 => 208.99072f64,
            Self::Hg210 => 209.99424f64,
            Self::Hg211 => 210.99933f64,
            Self::Hg212 => 212.00296f64,
            Self::Hg213 => 213.00823f64,
            Self::Hg214 => 214.012f64,
            Self::Hg215 => 215.0174f64,
            Self::Hg216 => 216.02132f64,
        }
    }
}
impl super::ElementVariant for MercuryIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Hg
    }
}
impl super::MassNumber for MercuryIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Hg171 => 171u16,
            Self::Hg172 => 172u16,
            Self::Hg173 => 173u16,
            Self::Hg174 => 174u16,
            Self::Hg175 => 175u16,
            Self::Hg176 => 176u16,
            Self::Hg177 => 177u16,
            Self::Hg178 => 178u16,
            Self::Hg179 => 179u16,
            Self::Hg180 => 180u16,
            Self::Hg181 => 181u16,
            Self::Hg182 => 182u16,
            Self::Hg183 => 183u16,
            Self::Hg184 => 184u16,
            Self::Hg185 => 185u16,
            Self::Hg186 => 186u16,
            Self::Hg187 => 187u16,
            Self::Hg188 => 188u16,
            Self::Hg189 => 189u16,
            Self::Hg190 => 190u16,
            Self::Hg191 => 191u16,
            Self::Hg192 => 192u16,
            Self::Hg193 => 193u16,
            Self::Hg194 => 194u16,
            Self::Hg195 => 195u16,
            Self::Hg196 => 196u16,
            Self::Hg197 => 197u16,
            Self::Hg198 => 198u16,
            Self::Hg199 => 199u16,
            Self::Hg200 => 200u16,
            Self::Hg201 => 201u16,
            Self::Hg202 => 202u16,
            Self::Hg203 => 203u16,
            Self::Hg204 => 204u16,
            Self::Hg205 => 205u16,
            Self::Hg206 => 206u16,
            Self::Hg207 => 207u16,
            Self::Hg208 => 208u16,
            Self::Hg209 => 209u16,
            Self::Hg210 => 210u16,
            Self::Hg211 => 211u16,
            Self::Hg212 => 212u16,
            Self::Hg213 => 213u16,
            Self::Hg214 => 214u16,
            Self::Hg215 => 215u16,
            Self::Hg216 => 216u16,
        }
    }
}
impl super::IsotopicComposition for MercuryIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Hg196 => Some(0.0015f64),
            Self::Hg198 => Some(0.0997f64),
            Self::Hg199 => Some(0.1687f64),
            Self::Hg200 => Some(0.231f64),
            Self::Hg201 => Some(0.1318f64),
            Self::Hg202 => Some(0.2986f64),
            Self::Hg204 => Some(0.0687f64),
            Self::Hg171
            | Self::Hg172
            | Self::Hg173
            | Self::Hg174
            | Self::Hg175
            | Self::Hg176
            | Self::Hg177
            | Self::Hg178
            | Self::Hg179
            | Self::Hg180
            | Self::Hg181
            | Self::Hg182
            | Self::Hg183
            | Self::Hg184
            | Self::Hg185
            | Self::Hg186
            | Self::Hg187
            | Self::Hg188
            | Self::Hg189
            | Self::Hg190
            | Self::Hg191
            | Self::Hg192
            | Self::Hg193
            | Self::Hg194
            | Self::Hg195
            | Self::Hg197
            | Self::Hg203
            | Self::Hg205
            | Self::Hg206
            | Self::Hg207
            | Self::Hg208
            | Self::Hg209
            | Self::Hg210
            | Self::Hg211
            | Self::Hg212
            | Self::Hg213
            | Self::Hg214
            | Self::Hg215
            | Self::Hg216 => None,
        }
    }
}
impl super::MostAbundantIsotope for MercuryIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Hg202
    }
}
impl From<MercuryIsotope> for crate::Isotope {
    fn from(isotope: MercuryIsotope) -> Self {
        crate::Isotope::Hg(isotope)
    }
}
impl From<MercuryIsotope> for crate::Element {
    fn from(_isotope: MercuryIsotope) -> Self {
        crate::Element::Hg
    }
}
impl TryFrom<u16> for MercuryIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            171u16 => Ok(Self::Hg171),
            172u16 => Ok(Self::Hg172),
            173u16 => Ok(Self::Hg173),
            174u16 => Ok(Self::Hg174),
            175u16 => Ok(Self::Hg175),
            176u16 => Ok(Self::Hg176),
            177u16 => Ok(Self::Hg177),
            178u16 => Ok(Self::Hg178),
            179u16 => Ok(Self::Hg179),
            180u16 => Ok(Self::Hg180),
            181u16 => Ok(Self::Hg181),
            182u16 => Ok(Self::Hg182),
            183u16 => Ok(Self::Hg183),
            184u16 => Ok(Self::Hg184),
            185u16 => Ok(Self::Hg185),
            186u16 => Ok(Self::Hg186),
            187u16 => Ok(Self::Hg187),
            188u16 => Ok(Self::Hg188),
            189u16 => Ok(Self::Hg189),
            190u16 => Ok(Self::Hg190),
            191u16 => Ok(Self::Hg191),
            192u16 => Ok(Self::Hg192),
            193u16 => Ok(Self::Hg193),
            194u16 => Ok(Self::Hg194),
            195u16 => Ok(Self::Hg195),
            196u16 => Ok(Self::Hg196),
            197u16 => Ok(Self::Hg197),
            198u16 => Ok(Self::Hg198),
            199u16 => Ok(Self::Hg199),
            200u16 => Ok(Self::Hg200),
            201u16 => Ok(Self::Hg201),
            202u16 => Ok(Self::Hg202),
            203u16 => Ok(Self::Hg203),
            204u16 => Ok(Self::Hg204),
            205u16 => Ok(Self::Hg205),
            206u16 => Ok(Self::Hg206),
            207u16 => Ok(Self::Hg207),
            208u16 => Ok(Self::Hg208),
            209u16 => Ok(Self::Hg209),
            210u16 => Ok(Self::Hg210),
            211u16 => Ok(Self::Hg211),
            212u16 => Ok(Self::Hg212),
            213u16 => Ok(Self::Hg213),
            214u16 => Ok(Self::Hg214),
            215u16 => Ok(Self::Hg215),
            216u16 => Ok(Self::Hg216),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Hg, value)),
        }
    }
}
impl std::fmt::Display for MercuryIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hg171 => write!(f, "Hg171"),
            Self::Hg172 => write!(f, "Hg172"),
            Self::Hg173 => write!(f, "Hg173"),
            Self::Hg174 => write!(f, "Hg174"),
            Self::Hg175 => write!(f, "Hg175"),
            Self::Hg176 => write!(f, "Hg176"),
            Self::Hg177 => write!(f, "Hg177"),
            Self::Hg178 => write!(f, "Hg178"),
            Self::Hg179 => write!(f, "Hg179"),
            Self::Hg180 => write!(f, "Hg180"),
            Self::Hg181 => write!(f, "Hg181"),
            Self::Hg182 => write!(f, "Hg182"),
            Self::Hg183 => write!(f, "Hg183"),
            Self::Hg184 => write!(f, "Hg184"),
            Self::Hg185 => write!(f, "Hg185"),
            Self::Hg186 => write!(f, "Hg186"),
            Self::Hg187 => write!(f, "Hg187"),
            Self::Hg188 => write!(f, "Hg188"),
            Self::Hg189 => write!(f, "Hg189"),
            Self::Hg190 => write!(f, "Hg190"),
            Self::Hg191 => write!(f, "Hg191"),
            Self::Hg192 => write!(f, "Hg192"),
            Self::Hg193 => write!(f, "Hg193"),
            Self::Hg194 => write!(f, "Hg194"),
            Self::Hg195 => write!(f, "Hg195"),
            Self::Hg196 => write!(f, "Hg196"),
            Self::Hg197 => write!(f, "Hg197"),
            Self::Hg198 => write!(f, "Hg198"),
            Self::Hg199 => write!(f, "Hg199"),
            Self::Hg200 => write!(f, "Hg200"),
            Self::Hg201 => write!(f, "Hg201"),
            Self::Hg202 => write!(f, "Hg202"),
            Self::Hg203 => write!(f, "Hg203"),
            Self::Hg204 => write!(f, "Hg204"),
            Self::Hg205 => write!(f, "Hg205"),
            Self::Hg206 => write!(f, "Hg206"),
            Self::Hg207 => write!(f, "Hg207"),
            Self::Hg208 => write!(f, "Hg208"),
            Self::Hg209 => write!(f, "Hg209"),
            Self::Hg210 => write!(f, "Hg210"),
            Self::Hg211 => write!(f, "Hg211"),
            Self::Hg212 => write!(f, "Hg212"),
            Self::Hg213 => write!(f, "Hg213"),
            Self::Hg214 => write!(f, "Hg214"),
            Self::Hg215 => write!(f, "Hg215"),
            Self::Hg216 => write!(f, "Hg216"),
        }
    }
}
