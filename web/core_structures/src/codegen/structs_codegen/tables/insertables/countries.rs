#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CountryAttribute {
    Iso,
    Name,
}
impl core::str::FromStr for CountryAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Iso" => Ok(Self::Iso),
            "Name" => Ok(Self::Name),
            "iso" => Ok(Self::Iso),
            "name" => Ok(Self::Name),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableCountryBuilder
{
    type Attribute = CountryAttribute;
}
impl core::fmt::Display for CountryAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Iso => write!(f, "countries.iso"),
            Self::Name => write!(f, "countries.name"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::countries::countries)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCountry {
    pub(crate) iso: ::iso_codes::CountryCode,
    pub(crate) name: String,
}
impl InsertableCountry {}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Country`](crate::codegen::structs_codegen::tables::countries::Country).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Country;
/// use core_structures::tables::insertables::CountrySettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let country = Country::new()
///    // Set mandatory fields
///    .iso(iso)?
///    .name(name)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableCountryBuilder {
    pub(crate) iso: Option<::iso_codes::CountryCode>,
    pub(crate) name: Option<String>,
}
impl diesel::associations::HasTable for InsertableCountryBuilder {
    type Table = crate::codegen::diesel_codegen::tables::countries::countries::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::countries::countries::table
    }
}
impl From<InsertableCountryBuilder>
    for web_common_traits::database::IdOrBuilder<::iso_codes::CountryCode, InsertableCountryBuilder>
{
    fn from(builder: InsertableCountryBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCountryBuilder
{
    fn is_complete(&self) -> bool {
        self.iso.is_some() && self.name.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Country` or
/// descendant tables.
pub trait CountrySettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.countries.iso` column.
    ///
    /// # Arguments
    /// * `iso`: The value to set for the `public.countries.iso` column.
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
    ///   `::iso_codes::CountryCode`.
    /// * If the provided value does not pass schema-defined validation.
    fn iso<I>(self, iso: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::iso_codes::CountryCode>;
    /// Sets the value of the `public.countries.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.countries.name` column.
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
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
}
impl CountrySettable for InsertableCountryBuilder
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::CountryAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.countries.iso` column.
    fn iso<I>(mut self, iso: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::iso_codes::CountryCode>,
    {
        let iso = <I as web_common_traits::database::PrimaryKeyLike>::primary_key(&iso);
        self.iso = Some(iso);
        Ok(self)
    }
    /// Sets the value of the `public.countries.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(CountryAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableCountryBuilder {
    type PrimaryKey = ::iso_codes::CountryCode;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableCountryBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::countries::Country,
            Error = web_common_traits::database::InsertError<CountryAttribute>,
        >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<CountryAttribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::countries::Country =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
