#[derive(Clone, core::fmt::Debug)]
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
    icon: font_awesome_icons::FAIcon,
    client_id: String,
    redirect_uri: String,
    oauth_url: String,
    scope: String,
}
impl InsertableLoginProvider {}
#[derive(Default)]
pub struct InsertableLoginProviderBuilder {
    name: Option<String>,
    icon: Option<font_awesome_icons::FAIcon>,
    client_id: Option<String>,
    redirect_uri: Option<String>,
    oauth_url: Option<String>,
    scope: Option<String>,
}
impl InsertableLoginProviderBuilder {
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
    pub fn icon(
        mut self,
        icon: font_awesome_icons::FAIcon,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn client_id(
        mut self,
        client_id: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.client_id = Some(client_id);
        Ok(self)
    }
    pub fn redirect_uri(
        mut self,
        redirect_uri: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.redirect_uri = Some(redirect_uri);
        Ok(self)
    }
    pub fn oauth_url(
        mut self,
        oauth_url: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.oauth_url = Some(oauth_url);
        Ok(self)
    }
    pub fn scope(
        mut self,
        scope: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
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
