#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AddressAttribute {
    Id,
    CityId,
    StreetName,
    HouseNumber,
    PostalCode,
    Geolocation,
}
impl core::str::FromStr for AddressAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CityId" => Ok(Self::CityId),
            "StreetName" => Ok(Self::StreetName),
            "HouseNumber" => Ok(Self::HouseNumber),
            "PostalCode" => Ok(Self::PostalCode),
            "Geolocation" => Ok(Self::Geolocation),
            "city_id" => Ok(Self::CityId),
            "street_name" => Ok(Self::StreetName),
            "house_number" => Ok(Self::HouseNumber),
            "postal_code" => Ok(Self::PostalCode),
            "geolocation" => Ok(Self::Geolocation),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
{
    type Attribute = AddressAttribute;
}
impl core::fmt::Display for AddressAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "addresses.id"),
            Self::CityId => write!(f, "addresses.city_id"),
            Self::StreetName => write!(f, "addresses.street_name"),
            Self::HouseNumber => write!(f, "addresses.house_number"),
            Self::PostalCode => write!(f, "addresses.postal_code"),
            Self::Geolocation => write!(f, "addresses.geolocation"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::addresses::addresses)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAddress {
    pub(crate) city_id: i32,
    pub(crate) street_name: String,
    pub(crate) house_number: String,
    pub(crate) postal_code: String,
    pub(crate) geolocation: postgis_diesel::types::Point,
}
impl InsertableAddress {
    pub fn city<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::cities::City, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::cities::City: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::cities::City::read(self.city_id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Address`](crate::codegen::structs_codegen::tables::addresses::Address).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Address;
/// use core_structures::tables::insertables::AddressSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let address = Address::new()
///    // Set mandatory fields
///    .city(city_id)?
///    .geolocation(geolocation)?
///    .house_number(house_number)?
///    .postal_code(postal_code)?
///    .street_name(street_name)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableAddressBuilder {
    pub(crate) city_id: Option<i32>,
    pub(crate) street_name: Option<String>,
    pub(crate) house_number: Option<String>,
    pub(crate) postal_code: Option<String>,
    pub(crate) geolocation: Option<postgis_diesel::types::Point>,
}
impl diesel::associations::HasTable for InsertableAddressBuilder {
    type Table = crate::codegen::diesel_codegen::tables::addresses::addresses::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::addresses::addresses::table
    }
}
impl From<InsertableAddressBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableAddressBuilder>
{
    fn from(builder: InsertableAddressBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
{
    fn is_complete(&self) -> bool {
        self.city_id.is_some()
            && self.street_name.is_some()
            && self.house_number.is_some()
            && self.postal_code.is_some()
            && self.geolocation.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Address` or
/// descendant tables.
pub trait AddressSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
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
    fn city<CI>(self, city_id: CI) -> Result<Self, Self::Error>
    where
        CI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
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
    fn street_name<SN>(self, street_name: SN) -> Result<Self, Self::Error>
    where
        SN: TryInto<String>,
        validation_errors::SingleFieldError: From<<SN as TryInto<String>>::Error>;
    /// Sets the value of the `public.addresses.house_number` column.
    ///
    /// # Arguments
    /// * `house_number`: The value to set for the
    ///   `public.addresses.house_number` column.
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
    fn house_number<HN>(self, house_number: HN) -> Result<Self, Self::Error>
    where
        HN: TryInto<String>,
        validation_errors::SingleFieldError: From<<HN as TryInto<String>>::Error>;
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
    fn postal_code<PC>(self, postal_code: PC) -> Result<Self, Self::Error>
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
    fn geolocation<G>(self, geolocation: G) -> Result<Self, Self::Error>
    where
        G: TryInto<postgis_diesel::types::Point>,
        validation_errors::SingleFieldError:
            From<<G as TryInto<postgis_diesel::types::Point>>::Error>;
}
impl AddressSettable for InsertableAddressBuilder
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::AddressAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.addresses.city_id` column.
    fn city<CI>(mut self, city_id: CI) -> Result<Self, Self::Error>
    where
        CI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let city_id = <CI as web_common_traits::database::PrimaryKeyLike>::primary_key(&city_id);
        self.city_id = Some(city_id);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.street_name` column.
    fn street_name<SN>(mut self, street_name: SN) -> Result<Self, Self::Error>
    where
        SN: TryInto<String>,
        validation_errors::SingleFieldError: From<<SN as TryInto<String>>::Error>,
    {
        let street_name = street_name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(AddressAttribute::StreetName)
        })?;
        self.street_name = Some(street_name);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.house_number` column.
    fn house_number<HN>(mut self, house_number: HN) -> Result<Self, Self::Error>
    where
        HN: TryInto<String>,
        validation_errors::SingleFieldError: From<<HN as TryInto<String>>::Error>,
    {
        let house_number = house_number.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(AddressAttribute::HouseNumber)
        })?;
        self.house_number = Some(house_number);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.postal_code` column.
    fn postal_code<PC>(mut self, postal_code: PC) -> Result<Self, Self::Error>
    where
        PC: TryInto<String>,
        validation_errors::SingleFieldError: From<<PC as TryInto<String>>::Error>,
    {
        let postal_code = postal_code.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(AddressAttribute::PostalCode)
        })?;
        self.postal_code = Some(postal_code);
        Ok(self)
    }
    /// Sets the value of the `public.addresses.geolocation` column.
    fn geolocation<G>(mut self, geolocation: G) -> Result<Self, Self::Error>
    where
        G: TryInto<postgis_diesel::types::Point>,
        validation_errors::SingleFieldError:
            From<<G as TryInto<postgis_diesel::types::Point>>::Error>,
    {
        let geolocation = geolocation.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(AddressAttribute::Geolocation)
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
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::addresses::Address,
            Error = web_common_traits::database::InsertError<AddressAttribute>,
        >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<AddressAttribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::addresses::Address =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
