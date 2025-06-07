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
    pub(crate) name: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) country: Option<String>,
    pub(crate) alpha_two_code: Option<::iso_codes::CountryCode>,
    pub(crate) state_province: Option<String>,
    pub(crate) domain: Option<String>,
}
impl InsertableOrganizationBuilder {
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
impl TryFrom<InsertableOrganizationBuilder> for InsertableOrganization {
    type Error = common_traits::prelude::BuilderError<InsertableOrganizationAttributes>;
    fn try_from(
        builder: InsertableOrganizationBuilder,
    ) -> Result<InsertableOrganization, Self::Error> {
        Ok(Self {
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganizationAttributes::Name,
            ))?,
            url: builder.url.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganizationAttributes::Url,
            ))?,
            country: builder.country.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganizationAttributes::Country,
                ),
            )?,
            alpha_two_code: builder.alpha_two_code.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganizationAttributes::AlphaTwoCode,
                ),
            )?,
            state_province: builder.state_province,
            domain: builder.domain.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganizationAttributes::Domain,
            ))?,
        })
    }
}
