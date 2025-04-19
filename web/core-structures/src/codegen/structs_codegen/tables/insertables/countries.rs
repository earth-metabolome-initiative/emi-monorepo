#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCountryAttributes {
    Iso,
    Emoji,
    Unicode,
    Name,
}
impl core::fmt::Display for InsertableCountryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCountryAttributes::Iso => write!(f, "iso"),
            InsertableCountryAttributes::Emoji => write!(f, "emoji"),
            InsertableCountryAttributes::Unicode => write!(f, "unicode"),
            InsertableCountryAttributes::Name => write!(f, "name"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::countries::countries)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCountry {
    iso: String,
    emoji: String,
    unicode: String,
    name: String,
}
impl InsertableCountry {}
#[derive(Default)]
pub struct InsertableCountryBuilder {
    iso: Option<String>,
    emoji: Option<String>,
    unicode: Option<String>,
    name: Option<String>,
}
impl InsertableCountryBuilder {
    pub fn iso(
        mut self,
        iso: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.iso = Some(iso);
        Ok(self)
    }
    pub fn emoji(
        mut self,
        emoji: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.emoji = Some(emoji);
        Ok(self)
    }
    pub fn unicode(
        mut self,
        unicode: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.unicode = Some(unicode);
        Ok(self)
    }
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableCountryBuilder {
    type Error = web_common_traits::database::InsertError<InsertableCountryAttributes>;
    type Object = InsertableCountry;
    type Attribute = InsertableCountryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            iso: self.iso.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCountryAttributes::Iso,
                )
            })?,
            emoji: self.emoji.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCountryAttributes::Emoji,
                )
            })?,
            unicode: self.unicode.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCountryAttributes::Unicode,
                )
            })?,
            name: self.name.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCountryAttributes::Name,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableCountry> for InsertableCountryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCountry) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .iso(insertable_variant.iso)?
            .emoji(insertable_variant.emoji)?
            .unicode(insertable_variant.unicode)?
            .name(insertable_variant.name)?)
    }
}
