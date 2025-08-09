#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAddressAttributes {
    Id,
    CityId,
    StreetName,
    StreetNumber,
    PostalCode,
    Geolocation,
}
impl core::fmt::Display for InsertableAddressAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CityId => write!(f, "city_id"),
            Self::StreetName => write!(f, "street_name"),
            Self::StreetNumber => write!(f, "street_number"),
            Self::PostalCode => write!(f, "postal_code"),
            Self::Geolocation => write!(f, "geolocation"),
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
    pub(crate) city_id: i32,
    pub(crate) street_name: String,
    pub(crate) street_number: String,
    pub(crate) postal_code: String,
    pub(crate) geolocation: postgis_diesel::types::Point,
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::cities::City::table(),
                self.city_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAddressBuilder {
    pub(crate) city_id: Option<i32>,
    pub(crate) street_name: Option<String>,
    pub(crate) street_number: Option<String>,
    pub(crate) postal_code: Option<String>,
    pub(crate) geolocation: Option<postgis_diesel::types::Point>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableAddressBuilder {
    type Attributes = InsertableAddressAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(city_id) = other.city_id {
            self = self.city(city_id)?;
        }
        if let Some(street_name) = other.street_name {
            self = self.street_name(street_name)?;
        }
        if let Some(street_number) = other.street_number {
            self = self.street_number(street_number)?;
        }
        if let Some(postal_code) = other.postal_code {
            self = self.postal_code(postal_code)?;
        }
        if let Some(geolocation) = other.geolocation {
            self = self.geolocation(geolocation)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableAddressBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder {
    /// Sets the value of the `addresses.city_id` column from table `addresses`.
    pub fn city(
        mut self,
        city_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>> {
        self.city_id = Some(city_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder {
    /// Sets the value of the `addresses.geolocation` column from table
    /// `addresses`.
    pub fn geolocation<Geolocation>(
        mut self,
        geolocation: Geolocation,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        Geolocation: TryInto<postgis_diesel::types::Point>,
        <Geolocation as TryInto<postgis_diesel::types::Point>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let geolocation = geolocation.try_into().map_err(
            |err: <Geolocation as TryInto<postgis_diesel::types::Point>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::Geolocation)
            },
        )?;
        self.geolocation = Some(geolocation);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder {
    /// Sets the value of the `addresses.postal_code` column from table
    /// `addresses`.
    pub fn postal_code<PostalCode>(
        mut self,
        postal_code: PostalCode,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        PostalCode: TryInto<String>,
        <PostalCode as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let postal_code =
            postal_code.try_into().map_err(|err: <PostalCode as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::PostalCode)
            })?;
        self.postal_code = Some(postal_code);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder {
    /// Sets the value of the `addresses.street_name` column from table
    /// `addresses`.
    pub fn street_name<StreetName>(
        mut self,
        street_name: StreetName,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        StreetName: TryInto<String>,
        <StreetName as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let street_name =
            street_name.try_into().map_err(|err: <StreetName as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::StreetName)
            })?;
        self.street_name = Some(street_name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder {
    /// Sets the value of the `addresses.street_number` column from table
    /// `addresses`.
    pub fn street_number<StreetNumber>(
        mut self,
        street_number: StreetNumber,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableAddressAttributes>>
    where
        StreetNumber: TryInto<String>,
        <StreetNumber as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let street_number =
            street_number.try_into().map_err(|err: <StreetNumber as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableAddressAttributes::StreetNumber)
            })?;
        self.street_number = Some(street_number);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableAddressBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::addresses::Address,
            Error = web_common_traits::database::InsertError<InsertableAddressAttributes>,
        >,
{
    type Attributes = InsertableAddressAttributes;
    fn is_complete(&self) -> bool {
        self.city_id.is_some()
            && self.street_name.is_some()
            && self.street_number.is_some()
            && self.postal_code.is_some()
            && self.geolocation.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::addresses::Address =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
