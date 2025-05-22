#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganizationAttributes {
    Name,
    Url,
    Country,
    AlphaTwoCode,
    StateProvince,
    Domain,
}
impl core::fmt::Display for InsertableOrganizationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganizationAttributes::Name => write!(f, "name"),
            InsertableOrganizationAttributes::Url => write!(f, "url"),
            InsertableOrganizationAttributes::Country => write!(f, "country"),
            InsertableOrganizationAttributes::AlphaTwoCode => write!(f, "alpha_two_code"),
            InsertableOrganizationAttributes::StateProvince => {
                write!(f, "state_province")
            }
            InsertableOrganizationAttributes::Domain => write!(f, "domain"),
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
    name: String,
    url: String,
    country: String,
    alpha_two_code: ::iso_codes::CountryCode,
    state_province: Option<String>,
    domain: String,
}
impl InsertableOrganization {}
#[derive(Default)]
pub struct InsertableOrganizationBuilder {
    name: Option<String>,
    url: Option<String>,
    country: Option<String>,
    alpha_two_code: Option<::iso_codes::CountryCode>,
    state_province: Option<String>,
    domain: Option<String>,
}
impl InsertableOrganizationBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn url<P>(
        mut self,
        url: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn country<P>(
        mut self,
        country: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn alpha_two_code<P>(
        mut self,
        alpha_two_code: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn state_province<P>(
        mut self,
        state_province: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn domain<P>(
        mut self,
        domain: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
impl common_traits::prelude::Builder for InsertableOrganizationBuilder {
    type Error = web_common_traits::database::InsertError<InsertableOrganizationAttributes>;
    type Object = InsertableOrganization;
    type Attribute = InsertableOrganizationAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganizationAttributes::Name,
            ))?,
            url: self.url.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganizationAttributes::Url,
            ))?,
            country: self.country.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganizationAttributes::Country,
            ))?,
            alpha_two_code: self.alpha_two_code.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganizationAttributes::AlphaTwoCode,
                ),
            )?,
            state_province: self.state_province,
            domain: self.domain.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganizationAttributes::Domain,
            ))?,
        })
    }
}
impl TryFrom<InsertableOrganization> for InsertableOrganizationBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableOrganization) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .url(insertable_variant.url)?
            .country(insertable_variant.country)?
            .alpha_two_code(insertable_variant.alpha_two_code)?
            .state_province(insertable_variant.state_province)?
            .domain(insertable_variant.domain)
    }
}
