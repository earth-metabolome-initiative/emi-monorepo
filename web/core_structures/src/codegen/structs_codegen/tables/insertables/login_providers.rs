#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoginProviderAttribute {
    Id,
    Name,
    Icon,
    ClientId,
    RedirectUri,
    OauthUrl,
    Scope,
}
impl core::str::FromStr for LoginProviderAttribute {
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
impl core::fmt::Display for LoginProviderAttribute {
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
impl From<InsertableLoginProviderBuilder>
    for web_common_traits::database::IdOrBuilder<i16, InsertableLoginProviderBuilder>
{
    fn from(builder: InsertableLoginProviderBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of `LoginProvider` or
/// descendant tables.
pub trait LoginProviderSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.login_providers.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.login_providers.name` column.
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
    /// Sets the value of the `public.login_providers.icon` column.
    ///
    /// # Arguments
    /// * `icon`: The value to set for the `public.login_providers.icon` column.
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
    fn icon<I>(
        self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>;
    /// Sets the value of the `public.login_providers.client_id` column.
    ///
    /// # Arguments
    /// * `client_id`: The value to set for the
    ///   `public.login_providers.client_id` column.
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
    fn client<CI>(
        self,
        client_id: CI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CI: TryInto<String>,
        validation_errors::SingleFieldError: From<<CI as TryInto<String>>::Error>;
    /// Sets the value of the `public.login_providers.redirect_uri` column.
    ///
    /// # Arguments
    /// * `redirect_uri`: The value to set for the
    ///   `public.login_providers.redirect_uri` column.
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
    fn redirect_uri<RU>(
        self,
        redirect_uri: RU,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        RU: TryInto<String>,
        validation_errors::SingleFieldError: From<<RU as TryInto<String>>::Error>;
    /// Sets the value of the `public.login_providers.oauth_url` column.
    ///
    /// # Arguments
    /// * `oauth_url`: The value to set for the
    ///   `public.login_providers.oauth_url` column.
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
    fn oauth_url<OU>(
        self,
        oauth_url: OU,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        OU: TryInto<String>,
        validation_errors::SingleFieldError: From<<OU as TryInto<String>>::Error>;
    /// Sets the value of the `public.login_providers.scope` column.
    ///
    /// # Arguments
    /// * `scope`: The value to set for the `public.login_providers.scope`
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
    fn scope<S>(
        self,
        scope: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<String>,
        validation_errors::SingleFieldError: From<<S as TryInto<String>>::Error>;
}
impl LoginProviderSettable for InsertableLoginProviderBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute;
    /// Sets the value of the `public.login_providers.name` column.
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
                .rename_field(LoginProviderAttribute::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::Name,
            )
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.login_providers.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        let icon = icon.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(LoginProviderAttribute::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::Icon,
            )
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    /// Sets the value of the `public.login_providers.client_id` column.
    fn client<CI>(
        mut self,
        client_id: CI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CI: TryInto<String>,
        validation_errors::SingleFieldError: From<<CI as TryInto<String>>::Error>,
    {
        let client_id = client_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(LoginProviderAttribute::ClientId)
        })?;
        pgrx_validation::must_be_paragraph(client_id.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::ClientId,
                    )
            })?;
        self.client_id = Some(client_id);
        Ok(self)
    }
    /// Sets the value of the `public.login_providers.redirect_uri` column.
    fn redirect_uri<RU>(
        mut self,
        redirect_uri: RU,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        RU: TryInto<String>,
        validation_errors::SingleFieldError: From<<RU as TryInto<String>>::Error>,
    {
        let redirect_uri = redirect_uri.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(LoginProviderAttribute::RedirectUri)
        })?;
        self.redirect_uri = Some(redirect_uri);
        Ok(self)
    }
    /// Sets the value of the `public.login_providers.oauth_url` column.
    fn oauth_url<OU>(
        mut self,
        oauth_url: OU,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        OU: TryInto<String>,
        validation_errors::SingleFieldError: From<<OU as TryInto<String>>::Error>,
    {
        let oauth_url = oauth_url.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(LoginProviderAttribute::OauthUrl)
        })?;
        self.oauth_url = Some(oauth_url);
        Ok(self)
    }
    /// Sets the value of the `public.login_providers.scope` column.
    fn scope<S>(
        mut self,
        scope: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<String>,
        validation_errors::SingleFieldError: From<<S as TryInto<String>>::Error>,
    {
        let scope = scope.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(LoginProviderAttribute::Scope)
        })?;
        pgrx_validation::must_be_paragraph(scope.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::Scope,
            )
        })?;
        self.scope = Some(scope);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableLoginProviderBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableLoginProviderBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
            Error = web_common_traits::database::InsertError<LoginProviderAttribute>,
        >,
{
    type Attributes = LoginProviderAttribute;
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
