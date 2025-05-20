#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCityAttributes {
    Name,
    Iso,
}
impl core::fmt::Display for InsertableCityAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCityAttributes::Name => write!(f, "name"),
            InsertableCityAttributes::Iso => write!(f, "iso"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::cities::cities)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCity {
    name: String,
    iso: iso_codes::CountryCode,
}
impl InsertableCity {
    #[cfg(feature = "postgres")]
    pub async fn iso(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::countries::Country, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::countries::Country::table()
            .filter(
                crate::codegen::diesel_codegen::tables::countries::countries::dsl::iso
                    .eq(&self.iso),
            )
            .first::<crate::codegen::structs_codegen::tables::countries::Country>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableCityBuilder {
    name: Option<String>,
    iso: Option<iso_codes::CountryCode>,
}
impl InsertableCityBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableCityAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn iso<P>(
        mut self,
        iso: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<iso_codes::CountryCode>,
        <P as TryInto<iso_codes::CountryCode>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let iso =
            iso.try_into().map_err(|err: <P as TryInto<iso_codes::CountryCode>>::Error| {
                Into::into(err).rename_field(InsertableCityAttributes::Iso)
            })?;
        self.iso = Some(iso);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableCityBuilder {
    type Error = web_common_traits::database::InsertError<InsertableCityAttributes>;
    type Object = InsertableCity;
    type Attribute = InsertableCityAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCityAttributes::Name,
            ))?,
            iso: self.iso.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCityAttributes::Iso,
            ))?,
        })
    }
}
impl TryFrom<InsertableCity> for InsertableCityBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCity) -> Result<Self, Self::Error> {
        Self::default().name(insertable_variant.name)?.iso(insertable_variant.iso)
    }
}
