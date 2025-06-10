#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAddressAttributes {
    CityId,
    StreetName,
    StreetNumber,
    PostalCode,
    Geolocation,
}
impl core::fmt::Display for InsertableAddressAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableAddressAttributes::CityId => write!(f, "city_id"),
            InsertableAddressAttributes::StreetName => write!(f, "street_name"),
            InsertableAddressAttributes::StreetNumber => write!(f, "street_number"),
            InsertableAddressAttributes::PostalCode => write!(f, "postal_code"),
            InsertableAddressAttributes::Geolocation => write!(f, "geolocation"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::addresses::addresses)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAddress {
    city_id: i32,
    street_name: String,
    street_number: String,
    postal_code: String,
    geolocation: postgis_diesel::types::Point,
}
impl InsertableAddress {
    pub fn city<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::cities::City,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cities::City: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::cities::City as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cities::City as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::cities::City as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cities::City as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::cities::City as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cities::City as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::cities::City,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::cities::City::table(),
                self.city_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableAddressBuilder {
    pub(crate) city_id: Option<i32>,
    pub(crate) street_name: Option<String>,
    pub(crate) street_number: Option<String>,
    pub(crate) postal_code: Option<String>,
    pub(crate) geolocation: Option<postgis_diesel::types::Point>,
}
impl InsertableAddressBuilder {
    pub fn city_id<P>(
        mut self,
        city_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let city_id = city_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableAddressAttributes::CityId)
        })?;
        self.city_id = Some(city_id);
        Ok(self)
    }
    pub fn street_name<P>(
        mut self,
        street_name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let street_name =
            street_name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::StreetName)
            })?;
        self.street_name = Some(street_name);
        Ok(self)
    }
    pub fn street_number<P>(
        mut self,
        street_number: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let street_number =
            street_number.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::StreetNumber)
            })?;
        self.street_number = Some(street_number);
        Ok(self)
    }
    pub fn postal_code<P>(
        mut self,
        postal_code: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let postal_code =
            postal_code.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::PostalCode)
            })?;
        self.postal_code = Some(postal_code);
        Ok(self)
    }
    pub fn geolocation<P>(
        mut self,
        geolocation: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        P: TryInto<postgis_diesel::types::Point>,
        <P as TryInto<postgis_diesel::types::Point>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let geolocation = geolocation.try_into().map_err(
            |err: <P as TryInto<postgis_diesel::types::Point>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::Geolocation)
            },
        )?;
        self.geolocation = Some(geolocation);
        Ok(self)
    }
}
impl TryFrom<InsertableAddressBuilder> for InsertableAddress {
    type Error = common_traits::prelude::BuilderError<InsertableAddressAttributes>;
    fn try_from(builder: InsertableAddressBuilder) -> Result<InsertableAddress, Self::Error> {
        Ok(Self {
            city_id: builder.city_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::CityId,
                ),
            )?,
            street_name: builder.street_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::StreetName,
                ),
            )?,
            street_number: builder.street_number.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::StreetNumber,
                ),
            )?,
            postal_code: builder.postal_code.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::PostalCode,
                ),
            )?,
            geolocation: builder.geolocation.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::Geolocation,
                ),
            )?,
        })
    }
}
