#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableLoginProviderAttributes {
    Name,
    Icon,
    ClientId,
    RedirectUri,
    OauthUrl,
    Scope,
}
impl core::fmt::Display for InsertableLoginProviderAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableLoginProviderAttributes::Name => write!(f, "name"),
            InsertableLoginProviderAttributes::Icon => write!(f, "icon"),
            InsertableLoginProviderAttributes::ClientId => write!(f, "client_id"),
            InsertableLoginProviderAttributes::RedirectUri => write!(f, "redirect_uri"),
            InsertableLoginProviderAttributes::OauthUrl => write!(f, "oauth_url"),
            InsertableLoginProviderAttributes::Scope => write!(f, "scope"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::login_providers::login_providers
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableLoginProvider {
    name: String,
    icon: String,
    client_id: String,
    redirect_uri: String,
    oauth_url: String,
    scope: String,
}
impl InsertableLoginProvider {}
#[derive(Default)]
pub struct InsertableLoginProviderBuilder {
    name: Option<String>,
    icon: Option<String>,
    client_id: Option<String>,
    redirect_uri: Option<String>,
    oauth_url: Option<String>,
    scope: Option<String>,
}
impl InsertableLoginProviderBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| e.rename_field(InsertableLoginProviderAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| e.rename_field(InsertableLoginProviderAttributes::Icon))?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn client_id<P>(
        mut self,
        client_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let client_id = client_id.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::ClientId)
        })?;
        pgrx_validation::must_be_paragraph(client_id.as_ref())
            .map_err(|e| e.rename_field(InsertableLoginProviderAttributes::ClientId))?;
        self.client_id = Some(client_id);
        Ok(self)
    }
    pub fn redirect_uri<P>(
        mut self,
        redirect_uri: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let redirect_uri =
            redirect_uri.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableLoginProviderAttributes::RedirectUri)
            })?;
        self.redirect_uri = Some(redirect_uri);
        Ok(self)
    }
    pub fn oauth_url<P>(
        mut self,
        oauth_url: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let oauth_url = oauth_url.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::OauthUrl)
        })?;
        self.oauth_url = Some(oauth_url);
        Ok(self)
    }
    pub fn scope<P>(
        mut self,
        scope: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let scope = scope.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::Scope)
        })?;
        pgrx_validation::must_be_paragraph(scope.as_ref())
            .map_err(|e| e.rename_field(InsertableLoginProviderAttributes::Scope))?;
        self.scope = Some(scope);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableLoginProviderBuilder {
    type Error = web_common_traits::database::InsertError<InsertableLoginProviderAttributes>;
    type Object = InsertableLoginProvider;
    type Attribute = InsertableLoginProviderAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableLoginProviderAttributes::Name,
            ))?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableLoginProviderAttributes::Icon,
            ))?,
            client_id: self.client_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableLoginProviderAttributes::ClientId,
                ),
            )?,
            redirect_uri: self.redirect_uri.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableLoginProviderAttributes::RedirectUri,
                ),
            )?,
            oauth_url: self.oauth_url.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableLoginProviderAttributes::OauthUrl,
                ),
            )?,
            scope: self.scope.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableLoginProviderAttributes::Scope,
            ))?,
        })
    }
}
impl TryFrom<InsertableLoginProvider> for InsertableLoginProviderBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableLoginProvider) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .icon(insertable_variant.icon)?
            .client_id(insertable_variant.client_id)?
            .redirect_uri(insertable_variant.redirect_uri)?
            .oauth_url(insertable_variant.oauth_url)?
            .scope(insertable_variant.scope)
    }
}
