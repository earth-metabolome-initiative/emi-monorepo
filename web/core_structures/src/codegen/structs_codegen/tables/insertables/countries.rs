#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCountryAttributes {
    Iso,
    Name,
}
impl core::fmt::Display for InsertableCountryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCountryAttributes::Iso => write!(f, "iso"),
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
    iso: ::iso_codes::CountryCode,
    name: String,
}
impl InsertableCountry {}
#[derive(Default)]
pub struct InsertableCountryBuilder {
    iso: Option<::iso_codes::CountryCode>,
    name: Option<String>,
}
impl InsertableCountryBuilder {
    pub fn iso<P>(
        mut self,
        iso: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::iso_codes::CountryCode>,
        <P as TryInto<::iso_codes::CountryCode>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let iso =
            iso.try_into().map_err(|err: <P as TryInto<::iso_codes::CountryCode>>::Error| {
                Into::into(err).rename_field(InsertableCountryAttributes::Iso)
            })?;
        self.iso = Some(iso);
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableCountryAttributes::Name)
        })?;
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
            iso: self.iso.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCountryAttributes::Iso,
            ))?,
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCountryAttributes::Name,
            ))?,
        })
    }
}
impl TryFrom<InsertableCountry> for InsertableCountryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCountry) -> Result<Self, Self::Error> {
        Self::default().iso(insertable_variant.iso)?.name(insertable_variant.name)
    }
}
