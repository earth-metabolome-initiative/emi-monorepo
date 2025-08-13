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
impl web_common_traits::database::ExtendableBuilder for InsertableCountryBuilder {
    type Attributes = InsertableCountryAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(iso) = other.iso {
            self = self.iso(iso)?;
        }
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableCountryBuilder {
    type PrimaryKey = ::iso_codes::CountryCode;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCountryBuilder {
    /// Sets the value of the `countries.iso` column from table `countries`.
    pub fn iso<Iso>(
        mut self,
        iso: Iso,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCountryAttributes>>
    where
        Iso: TryInto<::iso_codes::CountryCode>,
        <Iso as TryInto<::iso_codes::CountryCode>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let iso =
            iso.try_into().map_err(|err: <Iso as TryInto<::iso_codes::CountryCode>>::Error| {
                Into::into(err).rename_field(InsertableCountryAttributes::Iso)
            })?;
        self.iso = Some(iso);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCountryBuilder {
    /// Sets the value of the `countries.name` column from table `countries`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCountryAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableCountryAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
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
