#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCountryAttributes {
    Iso,
    Name,
}
impl core::str::FromStr for InsertableCountryAttributes {
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
impl core::fmt::Display for InsertableCountryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Iso => write!(f, "iso"),
            Self::Name => write!(f, "name"),
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
    pub(crate) iso: ::iso_codes::CountryCode,
    pub(crate) name: String,
}
impl InsertableCountry {}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCountryBuilder {
    pub(crate) iso: Option<::iso_codes::CountryCode>,
    pub(crate) name: Option<String>,
}
/// Trait defining setters for attributes of an instance of `Country` or
/// descendant tables.
pub trait CountryBuildable: std::marker::Sized {
    /// Attributes required to build the insertable.
    type Attributes;
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
    fn iso(
        self,
        iso: ::iso_codes::CountryCode,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
}
impl CountryBuildable for Option<::iso_codes::CountryCode> {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableCountryAttributes;
    fn iso(
        self,
        iso: ::iso_codes::CountryCode,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(Some(iso.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(Self::Attributes::Iso)
        })?))
    }
    fn name<N>(
        self,
        _name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        Ok(self)
    }
}
impl CountryBuildable for InsertableCountryBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableCountryAttributes;
    /// Sets the value of the `public.countries.iso` column.
    fn iso(
        mut self,
        iso: ::iso_codes::CountryCode,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let iso = iso.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableCountryAttributes::Iso)
        })?;
        self.iso = Some(iso);
        Ok(self)
    }
    /// Sets the value of the `public.countries.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableCountryAttributes::Name)
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
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::countries::Country,
            Error = web_common_traits::database::InsertError<InsertableCountryAttributes>,
        >,
{
    type Attributes = InsertableCountryAttributes;
    fn is_complete(&self) -> bool {
        self.iso.is_some() && self.name.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::countries::Country =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
