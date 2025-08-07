#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganizationAttributes {
    Name,
    Url,
    Country,
    AlphaTwoCode,
    StateProvince,
    Domain,
    Id,
}
impl core::fmt::Display for InsertableOrganizationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::Url => write!(f, "url"),
            Self::Country => write!(f, "country"),
            Self::AlphaTwoCode => write!(f, "alpha_two_code"),
            Self::StateProvince => write!(f, "state_province"),
            Self::Domain => write!(f, "domain"),
            Self::Id => write!(f, "id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::organizations::organizations
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganization {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) country: String,
    pub(crate) alpha_two_code: ::iso_codes::CountryCode,
    pub(crate) state_province: Option<String>,
    pub(crate) domain: String,
}
impl InsertableOrganization {}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganizationBuilder {
    pub(crate) name: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) country: Option<String>,
    pub(crate) alpha_two_code: Option<::iso_codes::CountryCode>,
    pub(crate) state_province: Option<String>,
    pub(crate) domain: Option<String>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableOrganizationBuilder {
    type Attributes = InsertableOrganizationAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(url) = other.url {
            self = self.url(url)?;
        }
        if let Some(country) = other.country {
            self = self.country(country)?;
        }
        if let Some(alpha_two_code) = other.alpha_two_code {
            self = self.alpha_two_code(alpha_two_code)?;
        }
        if let Some(state_province) = other.state_province {
            self = self.state_province(Some(state_province))?;
        }
        if let Some(domain) = other.domain {
            self = self.domain(domain)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableOrganizationBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder {
    /// Sets the value of the `organizations.alpha_two_code` column from table
    /// `organizations`.
    pub fn alpha_two_code<P>(
        mut self,
        alpha_two_code: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganizationAttributes>>
    where
        P: TryInto<::iso_codes::CountryCode>,
        <P as TryInto<::iso_codes::CountryCode>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let alpha_two_code = alpha_two_code.try_into().map_err(
            |err: <P as TryInto<::iso_codes::CountryCode>>::Error| {
                Into::into(err).rename_field(InsertableOrganizationAttributes::AlphaTwoCode)
            },
        )?;
        self.alpha_two_code = Some(alpha_two_code);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder {
    /// Sets the value of the `organizations.country` column from table
    /// `organizations`.
    pub fn country<P>(
        mut self,
        country: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganizationAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let country = country.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableOrganizationAttributes::Country)
        })?;
        self.country = Some(country);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder {
    /// Sets the value of the `organizations.domain` column from table
    /// `organizations`.
    pub fn domain<P>(
        mut self,
        domain: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganizationAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let domain = domain.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableOrganizationAttributes::Domain)
        })?;
        self.domain = Some(domain);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder {
    /// Sets the value of the `organizations.name` column from table
    /// `organizations`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganizationAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableOrganizationAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder {
    /// Sets the value of the `organizations.state_province` column from table
    /// `organizations`.
    pub fn state_province<P>(
        mut self,
        state_province: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganizationAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let state_province =
            state_province.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
                Into::into(err).rename_field(InsertableOrganizationAttributes::StateProvince)
            })?;
        self.state_province = state_province;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder {
    /// Sets the value of the `organizations.url` column from table
    /// `organizations`.
    pub fn url<P>(
        mut self,
        url: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganizationAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let url = url.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableOrganizationAttributes::Url)
        })?;
        self.url = Some(url);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableOrganizationBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::organizations::Organization,
            Error = web_common_traits::database::InsertError<InsertableOrganizationAttributes>,
        >,
{
    type Attributes = InsertableOrganizationAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.url.is_some()
            && self.country.is_some()
            && self.alpha_two_code.is_some()
            && self.domain.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::organizations::Organization =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
