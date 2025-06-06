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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCountryAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCountryAttributes>>
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
impl TryFrom<InsertableCountryBuilder> for InsertableCountry {
    type Error = common_traits::prelude::BuilderError<InsertableCountryAttributes>;
    fn try_from(builder: InsertableCountryBuilder) -> Result<InsertableCountry, Self::Error> {
        Ok(Self {
            iso: builder.iso.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCountryAttributes::Iso,
            ))?,
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCountryAttributes::Name,
            ))?,
        })
    }
}
