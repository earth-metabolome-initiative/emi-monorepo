//! Isotopes of the element Iridium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Iridium
pub enum IridiumIsotope {
    /// Isotope Ir164 of Iridium
    Ir164,
    /// Isotope Ir165 of Iridium
    Ir165,
    /// Isotope Ir166 of Iridium
    Ir166,
    /// Isotope Ir167 of Iridium
    Ir167,
    /// Isotope Ir168 of Iridium
    Ir168,
    /// Isotope Ir169 of Iridium
    Ir169,
    /// Isotope Ir170 of Iridium
    Ir170,
    /// Isotope Ir171 of Iridium
    Ir171,
    /// Isotope Ir172 of Iridium
    Ir172,
    /// Isotope Ir173 of Iridium
    Ir173,
    /// Isotope Ir174 of Iridium
    Ir174,
    /// Isotope Ir175 of Iridium
    Ir175,
    /// Isotope Ir176 of Iridium
    Ir176,
    /// Isotope Ir177 of Iridium
    Ir177,
    /// Isotope Ir178 of Iridium
    Ir178,
    /// Isotope Ir179 of Iridium
    Ir179,
    /// Isotope Ir180 of Iridium
    Ir180,
    /// Isotope Ir181 of Iridium
    Ir181,
    /// Isotope Ir182 of Iridium
    Ir182,
    /// Isotope Ir183 of Iridium
    Ir183,
    /// Isotope Ir184 of Iridium
    Ir184,
    /// Isotope Ir185 of Iridium
    Ir185,
    /// Isotope Ir186 of Iridium
    Ir186,
    /// Isotope Ir187 of Iridium
    Ir187,
    /// Isotope Ir188 of Iridium
    Ir188,
    /// Isotope Ir189 of Iridium
    Ir189,
    /// Isotope Ir190 of Iridium
    Ir190,
    /// Isotope Ir191 of Iridium
    Ir191,
    /// Isotope Ir192 of Iridium
    Ir192,
    /// Isotope Ir193 of Iridium
    Ir193,
    /// Isotope Ir194 of Iridium
    Ir194,
    /// Isotope Ir195 of Iridium
    Ir195,
    /// Isotope Ir196 of Iridium
    Ir196,
    /// Isotope Ir197 of Iridium
    Ir197,
    /// Isotope Ir198 of Iridium
    Ir198,
    /// Isotope Ir199 of Iridium
    Ir199,
    /// Isotope Ir200 of Iridium
    Ir200,
    /// Isotope Ir201 of Iridium
    Ir201,
    /// Isotope Ir202 of Iridium
    Ir202,
    /// Isotope Ir203 of Iridium
    Ir203,
    /// Isotope Ir204 of Iridium
    Ir204,
}
impl super::RelativeAtomicMass for IridiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ir164 => 163.99191f64,
            Self::Ir165 => 164.9875f64,
            Self::Ir166 => 165.98566f64,
            Self::Ir167 => 166.981666f64,
            Self::Ir168 => 167.979907f64,
            Self::Ir169 => 168.976298f64,
            Self::Ir170 => 169.974922f64,
            Self::Ir171 => 170.97164f64,
            Self::Ir172 => 171.970607f64,
            Self::Ir173 => 172.967506f64,
            Self::Ir174 => 173.966861f64,
            Self::Ir175 => 174.96415f64,
            Self::Ir176 => 175.96365f64,
            Self::Ir177 => 176.961301f64,
            Self::Ir178 => 177.961082f64,
            Self::Ir179 => 178.95912f64,
            Self::Ir180 => 179.959229f64,
            Self::Ir181 => 180.957625f64,
            Self::Ir182 => 181.958076f64,
            Self::Ir183 => 182.95684f64,
            Self::Ir184 => 183.957476f64,
            Self::Ir185 => 184.956698f64,
            Self::Ir186 => 185.957944f64,
            Self::Ir187 => 186.957542f64,
            Self::Ir188 => 187.958828f64,
            Self::Ir189 => 188.958715f64,
            Self::Ir190 => 189.9605412f64,
            Self::Ir191 => 190.9605893f64,
            Self::Ir192 => 191.9626002f64,
            Self::Ir193 => 192.9629216f64,
            Self::Ir194 => 193.9650735f64,
            Self::Ir195 => 194.9659747f64,
            Self::Ir196 => 195.968397f64,
            Self::Ir197 => 196.969655f64,
            Self::Ir198 => 197.97228f64,
            Self::Ir199 => 198.973805f64,
            Self::Ir200 => 199.9768f64,
            Self::Ir201 => 200.97864f64,
            Self::Ir202 => 201.98199f64,
            Self::Ir203 => 202.98423f64,
            Self::Ir204 => 203.9896f64,
        }
    }
}
impl super::ElementVariant for IridiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ir
    }
}
impl super::MassNumber for IridiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ir164 => 164u16,
            Self::Ir165 => 165u16,
            Self::Ir166 => 166u16,
            Self::Ir167 => 167u16,
            Self::Ir168 => 168u16,
            Self::Ir169 => 169u16,
            Self::Ir170 => 170u16,
            Self::Ir171 => 171u16,
            Self::Ir172 => 172u16,
            Self::Ir173 => 173u16,
            Self::Ir174 => 174u16,
            Self::Ir175 => 175u16,
            Self::Ir176 => 176u16,
            Self::Ir177 => 177u16,
            Self::Ir178 => 178u16,
            Self::Ir179 => 179u16,
            Self::Ir180 => 180u16,
            Self::Ir181 => 181u16,
            Self::Ir182 => 182u16,
            Self::Ir183 => 183u16,
            Self::Ir184 => 184u16,
            Self::Ir185 => 185u16,
            Self::Ir186 => 186u16,
            Self::Ir187 => 187u16,
            Self::Ir188 => 188u16,
            Self::Ir189 => 189u16,
            Self::Ir190 => 190u16,
            Self::Ir191 => 191u16,
            Self::Ir192 => 192u16,
            Self::Ir193 => 193u16,
            Self::Ir194 => 194u16,
            Self::Ir195 => 195u16,
            Self::Ir196 => 196u16,
            Self::Ir197 => 197u16,
            Self::Ir198 => 198u16,
            Self::Ir199 => 199u16,
            Self::Ir200 => 200u16,
            Self::Ir201 => 201u16,
            Self::Ir202 => 202u16,
            Self::Ir203 => 203u16,
            Self::Ir204 => 204u16,
        }
    }
}
impl super::IsotopicComposition for IridiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ir191 => Some(0.373f64),
            Self::Ir193 => Some(0.627f64),
            Self::Ir164
            | Self::Ir165
            | Self::Ir166
            | Self::Ir167
            | Self::Ir168
            | Self::Ir169
            | Self::Ir170
            | Self::Ir171
            | Self::Ir172
            | Self::Ir173
            | Self::Ir174
            | Self::Ir175
            | Self::Ir176
            | Self::Ir177
            | Self::Ir178
            | Self::Ir179
            | Self::Ir180
            | Self::Ir181
            | Self::Ir182
            | Self::Ir183
            | Self::Ir184
            | Self::Ir185
            | Self::Ir186
            | Self::Ir187
            | Self::Ir188
            | Self::Ir189
            | Self::Ir190
            | Self::Ir192
            | Self::Ir194
            | Self::Ir195
            | Self::Ir196
            | Self::Ir197
            | Self::Ir198
            | Self::Ir199
            | Self::Ir200
            | Self::Ir201
            | Self::Ir202
            | Self::Ir203
            | Self::Ir204 => None,
        }
    }
}
impl super::MostAbundantIsotope for IridiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ir193
    }
}
impl From<IridiumIsotope> for crate::Isotope {
    fn from(isotope: IridiumIsotope) -> Self {
        crate::Isotope::Ir(isotope)
    }
}
impl From<IridiumIsotope> for crate::Element {
    fn from(_isotope: IridiumIsotope) -> Self {
        crate::Element::Ir
    }
}
impl TryFrom<u16> for IridiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            164u16 => Ok(Self::Ir164),
            165u16 => Ok(Self::Ir165),
            166u16 => Ok(Self::Ir166),
            167u16 => Ok(Self::Ir167),
            168u16 => Ok(Self::Ir168),
            169u16 => Ok(Self::Ir169),
            170u16 => Ok(Self::Ir170),
            171u16 => Ok(Self::Ir171),
            172u16 => Ok(Self::Ir172),
            173u16 => Ok(Self::Ir173),
            174u16 => Ok(Self::Ir174),
            175u16 => Ok(Self::Ir175),
            176u16 => Ok(Self::Ir176),
            177u16 => Ok(Self::Ir177),
            178u16 => Ok(Self::Ir178),
            179u16 => Ok(Self::Ir179),
            180u16 => Ok(Self::Ir180),
            181u16 => Ok(Self::Ir181),
            182u16 => Ok(Self::Ir182),
            183u16 => Ok(Self::Ir183),
            184u16 => Ok(Self::Ir184),
            185u16 => Ok(Self::Ir185),
            186u16 => Ok(Self::Ir186),
            187u16 => Ok(Self::Ir187),
            188u16 => Ok(Self::Ir188),
            189u16 => Ok(Self::Ir189),
            190u16 => Ok(Self::Ir190),
            191u16 => Ok(Self::Ir191),
            192u16 => Ok(Self::Ir192),
            193u16 => Ok(Self::Ir193),
            194u16 => Ok(Self::Ir194),
            195u16 => Ok(Self::Ir195),
            196u16 => Ok(Self::Ir196),
            197u16 => Ok(Self::Ir197),
            198u16 => Ok(Self::Ir198),
            199u16 => Ok(Self::Ir199),
            200u16 => Ok(Self::Ir200),
            201u16 => Ok(Self::Ir201),
            202u16 => Ok(Self::Ir202),
            203u16 => Ok(Self::Ir203),
            204u16 => Ok(Self::Ir204),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ir, value)),
        }
    }
}
impl std::fmt::Display for IridiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ir164 => write!(f, "Ir164"),
            Self::Ir165 => write!(f, "Ir165"),
            Self::Ir166 => write!(f, "Ir166"),
            Self::Ir167 => write!(f, "Ir167"),
            Self::Ir168 => write!(f, "Ir168"),
            Self::Ir169 => write!(f, "Ir169"),
            Self::Ir170 => write!(f, "Ir170"),
            Self::Ir171 => write!(f, "Ir171"),
            Self::Ir172 => write!(f, "Ir172"),
            Self::Ir173 => write!(f, "Ir173"),
            Self::Ir174 => write!(f, "Ir174"),
            Self::Ir175 => write!(f, "Ir175"),
            Self::Ir176 => write!(f, "Ir176"),
            Self::Ir177 => write!(f, "Ir177"),
            Self::Ir178 => write!(f, "Ir178"),
            Self::Ir179 => write!(f, "Ir179"),
            Self::Ir180 => write!(f, "Ir180"),
            Self::Ir181 => write!(f, "Ir181"),
            Self::Ir182 => write!(f, "Ir182"),
            Self::Ir183 => write!(f, "Ir183"),
            Self::Ir184 => write!(f, "Ir184"),
            Self::Ir185 => write!(f, "Ir185"),
            Self::Ir186 => write!(f, "Ir186"),
            Self::Ir187 => write!(f, "Ir187"),
            Self::Ir188 => write!(f, "Ir188"),
            Self::Ir189 => write!(f, "Ir189"),
            Self::Ir190 => write!(f, "Ir190"),
            Self::Ir191 => write!(f, "Ir191"),
            Self::Ir192 => write!(f, "Ir192"),
            Self::Ir193 => write!(f, "Ir193"),
            Self::Ir194 => write!(f, "Ir194"),
            Self::Ir195 => write!(f, "Ir195"),
            Self::Ir196 => write!(f, "Ir196"),
            Self::Ir197 => write!(f, "Ir197"),
            Self::Ir198 => write!(f, "Ir198"),
            Self::Ir199 => write!(f, "Ir199"),
            Self::Ir200 => write!(f, "Ir200"),
            Self::Ir201 => write!(f, "Ir201"),
            Self::Ir202 => write!(f, "Ir202"),
            Self::Ir203 => write!(f, "Ir203"),
            Self::Ir204 => write!(f, "Ir204"),
        }
    }
}
