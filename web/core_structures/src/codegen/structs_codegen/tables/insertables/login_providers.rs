#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableLoginProviderAttributes {
    Id,
    Name,
    Icon,
    ClientId,
    RedirectUri,
    OauthUrl,
    Scope,
}
impl core::str::FromStr for InsertableLoginProviderAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "Icon" => Ok(Self::Icon),
            "ClientId" => Ok(Self::ClientId),
            "RedirectUri" => Ok(Self::RedirectUri),
            "OauthUrl" => Ok(Self::OauthUrl),
            "Scope" => Ok(Self::Scope),
            "name" => Ok(Self::Name),
            "icon" => Ok(Self::Icon),
            "client_id" => Ok(Self::ClientId),
            "redirect_uri" => Ok(Self::RedirectUri),
            "oauth_url" => Ok(Self::OauthUrl),
            "scope" => Ok(Self::Scope),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableLoginProviderAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::Icon => write!(f, "icon"),
            Self::ClientId => write!(f, "client_id"),
            Self::RedirectUri => write!(f, "redirect_uri"),
            Self::OauthUrl => write!(f, "oauth_url"),
            Self::Scope => write!(f, "scope"),
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
    pub(crate) name: String,
    pub(crate) icon: String,
    pub(crate) client_id: String,
    pub(crate) redirect_uri: String,
    pub(crate) oauth_url: String,
    pub(crate) scope: String,
}
impl InsertableLoginProvider {}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableLoginProviderBuilder {
    pub(crate) name: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) client_id: Option<String>,
    pub(crate) redirect_uri: Option<String>,
    pub(crate) oauth_url: Option<String>,
    pub(crate) scope: Option<String>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableLoginProviderBuilder {
    type Attributes = InsertableLoginProviderAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(icon) = other.icon {
            self = self.icon(icon)?;
        }
        if let Some(client_id) = other.client_id {
            self = self.client(client_id)?;
        }
        if let Some(redirect_uri) = other.redirect_uri {
            self = self.redirect_uri(redirect_uri)?;
        }
        if let Some(oauth_url) = other.oauth_url {
            self = self.oauth_url(oauth_url)?;
        }
        if let Some(scope) = other.scope {
            self = self.scope(scope)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableLoginProviderBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder {
    /// Sets the value of the `login_providers.client_id` column from table
    /// `login_providers`.
    pub fn client<ClientId>(
        mut self,
        client_id: ClientId,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableLoginProviderAttributes>>
    where
        ClientId: TryInto<String>,
        <ClientId as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let client_id =
            client_id.try_into().map_err(|err: <ClientId as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableLoginProviderAttributes::ClientId)
            })?;
        pgrx_validation::must_be_paragraph(client_id.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderAttributes::ClientId,
                    )
            })?;
        self.client_id = Some(client_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder {
    /// Sets the value of the `login_providers.icon` column from table
    /// `login_providers`.
    pub fn icon<Icon>(
        mut self,
        icon: Icon,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableLoginProviderAttributes>>
    where
        Icon: TryInto<String>,
        <Icon as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <Icon as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderAttributes::Icon,
                    )
            })?;
        self.icon = Some(icon);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder {
    /// Sets the value of the `login_providers.name` column from table
    /// `login_providers`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableLoginProviderAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderAttributes::Name,
                    )
            })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder {
    /// Sets the value of the `login_providers.oauth_url` column from table
    /// `login_providers`.
    pub fn oauth_url<OauthUrl>(
        mut self,
        oauth_url: OauthUrl,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableLoginProviderAttributes>>
    where
        OauthUrl: TryInto<String>,
        <OauthUrl as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let oauth_url =
            oauth_url.try_into().map_err(|err: <OauthUrl as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableLoginProviderAttributes::OauthUrl)
            })?;
        self.oauth_url = Some(oauth_url);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder {
    /// Sets the value of the `login_providers.redirect_uri` column from table
    /// `login_providers`.
    pub fn redirect_uri<RedirectUri>(
        mut self,
        redirect_uri: RedirectUri,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableLoginProviderAttributes>>
    where
        RedirectUri: TryInto<String>,
        <RedirectUri as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let redirect_uri =
            redirect_uri.try_into().map_err(|err: <RedirectUri as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableLoginProviderAttributes::RedirectUri)
            })?;
        self.redirect_uri = Some(redirect_uri);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder {
    /// Sets the value of the `login_providers.scope` column from table
    /// `login_providers`.
    pub fn scope<Scope>(
        mut self,
        scope: Scope,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableLoginProviderAttributes>>
    where
        Scope: TryInto<String>,
        <Scope as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let scope = scope.try_into().map_err(|err: <Scope as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableLoginProviderAttributes::Scope)
        })?;
        pgrx_validation::must_be_paragraph(scope.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderAttributes::Scope,
                    )
            })?;
        self.scope = Some(scope);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableLoginProviderBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
            Error = web_common_traits::database::InsertError<InsertableLoginProviderAttributes>,
        >,
{
    type Attributes = InsertableLoginProviderAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.icon.is_some()
            && self.client_id.is_some()
            && self.redirect_uri.is_some()
            && self.oauth_url.is_some()
            && self.scope.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::login_providers::LoginProvider =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
