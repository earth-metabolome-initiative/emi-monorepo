#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrganizationAttribute {
    Name,
    Url,
    Country,
    AlphaTwoCode,
    StateProvince,
    Domain,
    Id,
}
impl core::str::FromStr for OrganizationAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "Url" => Ok(Self::Url),
            "Country" => Ok(Self::Country),
            "AlphaTwoCode" => Ok(Self::AlphaTwoCode),
            "StateProvince" => Ok(Self::StateProvince),
            "Domain" => Ok(Self::Domain),
            "name" => Ok(Self::Name),
            "url" => Ok(Self::Url),
            "country" => Ok(Self::Country),
            "alpha_two_code" => Ok(Self::AlphaTwoCode),
            "state_province" => Ok(Self::StateProvince),
            "domain" => Ok(Self::Domain),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for OrganizationAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "organizations.name"),
            Self::Url => write!(f, "organizations.url"),
            Self::Country => write!(f, "organizations.country"),
            Self::AlphaTwoCode => write!(f, "organizations.alpha_two_code"),
            Self::StateProvince => write!(f, "organizations.state_province"),
            Self::Domain => write!(f, "organizations.domain"),
            Self::Id => write!(f, "organizations.id"),
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
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganizationBuilder {
    pub(crate) name: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) country: Option<String>,
    pub(crate) alpha_two_code: Option<::iso_codes::CountryCode>,
    pub(crate) state_province: Option<String>,
    pub(crate) domain: Option<String>,
}
impl From<InsertableOrganizationBuilder>
    for web_common_traits::database::IdOrBuilder<i16, InsertableOrganizationBuilder>
{
    fn from(builder: InsertableOrganizationBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
{
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.url.is_some()
            && self.country.is_some()
            && self.alpha_two_code.is_some()
            && self.domain.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Organization` or
/// descendant tables.
pub trait OrganizationSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.organizations.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.organizations.name` column.
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
    /// Sets the value of the `public.organizations.url` column.
    ///
    /// # Arguments
    /// * `url`: The value to set for the `public.organizations.url` column.
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
    fn url<U>(
        self,
        url: U,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        U: TryInto<String>,
        validation_errors::SingleFieldError: From<<U as TryInto<String>>::Error>;
    /// Sets the value of the `public.organizations.country` column.
    ///
    /// # Arguments
    /// * `country`: The value to set for the `public.organizations.country`
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
    fn country<C>(
        self,
        country: C,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        C: TryInto<String>,
        validation_errors::SingleFieldError: From<<C as TryInto<String>>::Error>;
    /// Sets the value of the `public.organizations.alpha_two_code` column.
    ///
    /// # Arguments
    /// * `alpha_two_code`: The value to set for the
    ///   `public.organizations.alpha_two_code` column.
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
    fn alpha_two_code<ATC>(
        self,
        alpha_two_code: ATC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        ATC: TryInto<::iso_codes::CountryCode>,
        validation_errors::SingleFieldError:
            From<<ATC as TryInto<::iso_codes::CountryCode>>::Error>;
    /// Sets the value of the `public.organizations.state_province` column.
    ///
    /// # Arguments
    /// * `state_province`: The value to set for the
    ///   `public.organizations.state_province` column.
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
    fn state_province<SP>(
        self,
        state_province: SP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SP: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<SP as TryInto<Option<String>>>::Error>;
    /// Sets the value of the `public.organizations.domain` column.
    ///
    /// # Arguments
    /// * `domain`: The value to set for the `public.organizations.domain`
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
    fn domain<D>(
        self,
        domain: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>;
}
impl OrganizationSettable for InsertableOrganizationBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute;
    /// Sets the value of the `public.organizations.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(OrganizationAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.organizations.url` column.
    fn url<U>(
        mut self,
        url: U,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        U: TryInto<String>,
        validation_errors::SingleFieldError: From<<U as TryInto<String>>::Error>,
    {
        let url = url.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(OrganizationAttribute::Url)
        })?;
        self.url = Some(url);
        Ok(self)
    }
    /// Sets the value of the `public.organizations.country` column.
    fn country<C>(
        mut self,
        country: C,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        C: TryInto<String>,
        validation_errors::SingleFieldError: From<<C as TryInto<String>>::Error>,
    {
        let country = country.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(OrganizationAttribute::Country)
        })?;
        self.country = Some(country);
        Ok(self)
    }
    /// Sets the value of the `public.organizations.alpha_two_code` column.
    fn alpha_two_code<ATC>(
        mut self,
        alpha_two_code: ATC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        ATC: TryInto<::iso_codes::CountryCode>,
        validation_errors::SingleFieldError:
            From<<ATC as TryInto<::iso_codes::CountryCode>>::Error>,
    {
        let alpha_two_code = alpha_two_code.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(OrganizationAttribute::AlphaTwoCode)
        })?;
        self.alpha_two_code = Some(alpha_two_code);
        Ok(self)
    }
    /// Sets the value of the `public.organizations.state_province` column.
    fn state_province<SP>(
        mut self,
        state_province: SP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SP: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<SP as TryInto<Option<String>>>::Error>,
    {
        let state_province = state_province.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(OrganizationAttribute::StateProvince)
        })?;
        self.state_province = state_province;
        Ok(self)
    }
    /// Sets the value of the `public.organizations.domain` column.
    fn domain<D>(
        mut self,
        domain: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        let domain = domain.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(OrganizationAttribute::Domain)
        })?;
        self.domain = Some(domain);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableOrganizationBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableOrganizationBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::organizations::Organization,
            Error = web_common_traits::database::InsertError<OrganizationAttribute>,
        >,
{
    type Attribute = OrganizationAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::organizations::Organization =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
