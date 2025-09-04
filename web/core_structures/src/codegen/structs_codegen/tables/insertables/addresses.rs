#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAddressAttribute {
    Id,
    CityId,
    StreetName,
    StreetNumber,
    PostalCode,
    Geolocation,
}
impl core::str::FromStr for InsertableAddressAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CityId" => Ok(Self::CityId),
            "StreetName" => Ok(Self::StreetName),
            "StreetNumber" => Ok(Self::StreetNumber),
            "PostalCode" => Ok(Self::PostalCode),
            "Geolocation" => Ok(Self::Geolocation),
            "city_id" => Ok(Self::CityId),
            "street_name" => Ok(Self::StreetName),
            "street_number" => Ok(Self::StreetNumber),
            "postal_code" => Ok(Self::PostalCode),
            "geolocation" => Ok(Self::Geolocation),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableAddressAttribute {
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
/// Trait defining setters for attributes of an instance of `Address` or
/// descendant tables.
pub trait AddressSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.addresses.city_id` column.
    ///
    /// # Arguments
    /// * `city_id`: The value to set for the `public.addresses.city_id` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn city(
        self,
        city_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.addresses.street_name` column.
    ///
    /// # Arguments
    /// * `street_name`: The value to set for the `public.addresses.street_name`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn street_name<SN>(
        self,
        street_name: SN,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SN: TryInto<String>,
        validation_errors::SingleFieldError: From<<SN as TryInto<String>>::Error>;
    /// Sets the value of the `public.addresses.street_number` column.
    ///
    /// # Arguments
    /// * `street_number`: The value to set for the
    ///   `public.addresses.street_number` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn street_number<SN>(
        self,
        street_number: SN,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SN: TryInto<String>,
        validation_errors::SingleFieldError: From<<SN as TryInto<String>>::Error>;
    /// Sets the value of the `public.addresses.postal_code` column.
    ///
    /// # Arguments
    /// * `postal_code`: The value to set for the `public.addresses.postal_code`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn postal_code<PC>(
        self,
        postal_code: PC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PC: TryInto<String>,
        validation_errors::SingleFieldError: From<<PC as TryInto<String>>::Error>;
    /// Sets the value of the `public.addresses.geolocation` column.
    ///
    /// # Arguments
    /// * `geolocation`: The value to set for the `public.addresses.geolocation`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `postgis_diesel::types::Point`.
    /// * If the provided value does not pass schema-defined validation.
    fn geolocation<G>(
        self,
        geolocation: G,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        G: TryInto<postgis_diesel::types::Point>,
        validation_errors::SingleFieldError:
            From<<G as TryInto<postgis_diesel::types::Point>>::Error>;
}
impl AddressSettable for InsertableAddressBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableAddressAttribute;
    /// Sets the value of the `public.addresses.city_id` column.
    fn city(
        mut self,
        city_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.city_id = Some(city_id);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.street_name` column.
    fn street_name<SN>(
        mut self,
        street_name: SN,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SN: TryInto<String>,
        validation_errors::SingleFieldError: From<<SN as TryInto<String>>::Error>,
    {
        let street_name = street_name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableAddressAttribute::StreetName)
        })?;
        self.street_name = Some(street_name);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.street_number` column.
    fn street_number<SN>(
        mut self,
        street_number: SN,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SN: TryInto<String>,
        validation_errors::SingleFieldError: From<<SN as TryInto<String>>::Error>,
    {
        let street_number = street_number.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableAddressAttribute::StreetNumber)
        })?;
        self.street_number = Some(street_number);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.postal_code` column.
    fn postal_code<PC>(
        mut self,
        postal_code: PC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PC: TryInto<String>,
        validation_errors::SingleFieldError: From<<PC as TryInto<String>>::Error>,
    {
        let postal_code = postal_code.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableAddressAttribute::PostalCode)
        })?;
        self.postal_code = Some(postal_code);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.geolocation` column.
    fn geolocation<G>(
        mut self,
        geolocation: G,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        G: TryInto<postgis_diesel::types::Point>,
        validation_errors::SingleFieldError:
            From<<G as TryInto<postgis_diesel::types::Point>>::Error>,
    {
        let geolocation = geolocation.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableAddressAttribute::Geolocation)
        })?;
        self.geolocation = Some(geolocation);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableAddressBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableAddressBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::addresses::Address,
            Error = web_common_traits::database::InsertError<InsertableAddressAttribute>,
        >,
{
    type Attributes = InsertableAddressAttribute;
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
