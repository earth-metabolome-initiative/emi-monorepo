#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CityAttribute {
    Id,
    Name,
    Iso,
}
impl core::str::FromStr for CityAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "Iso" => Ok(Self::Iso),
            "name" => Ok(Self::Name),
            "iso" => Ok(Self::Iso),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for CityAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "cities.id"),
            Self::Name => write!(f, "cities.name"),
            Self::Iso => write!(f, "cities.iso"),
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
    pub(crate) name: String,
    pub(crate) iso: ::iso_codes::CountryCode,
}
impl InsertableCity {
    pub fn iso<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::countries::Country, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::countries::Country:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::countries::Country::read(self.iso, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCityBuilder {
    pub(crate) name: Option<String>,
    pub(crate) iso: Option<::iso_codes::CountryCode>,
}
impl From<InsertableCityBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCityBuilder>
{
    fn from(builder: InsertableCityBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder
{
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.iso.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `City` or descendant
/// tables.
pub trait CitySettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.cities.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.cities.name` column.
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
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.cities.iso` column.
    ///
    /// # Arguments
    /// * `iso`: The value to set for the `public.cities.iso` column.
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
    fn iso<I>(
        self,
        iso: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::iso_codes::CountryCode>;
}
impl CitySettable for InsertableCityBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CityAttribute;
    /// Sets the value of the `public.cities.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(CityAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.cities.iso` column.
    fn iso<I>(
        mut self,
        iso: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::iso_codes::CountryCode>,
    {
        let iso = <I as web_common_traits::database::PrimaryKeyLike>::primary_key(&iso);
        self.iso = Some(iso);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableCityBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableCityBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::cities::City,
            Error = web_common_traits::database::InsertError<CityAttribute>,
        >,
{
    type Attribute = CityAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::cities::City =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
