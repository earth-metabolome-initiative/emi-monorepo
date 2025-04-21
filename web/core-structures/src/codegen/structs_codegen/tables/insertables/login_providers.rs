#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableLoginProviderAttributes {
    Name,
    IconId,
    ColorId,
    ClientIdVarName,
    RedirectUriVarName,
    OauthUrl,
    Scope,
}
impl core::fmt::Display for InsertableLoginProviderAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableLoginProviderAttributes::Name => write!(f, "name"),
            InsertableLoginProviderAttributes::IconId => write!(f, "icon_id"),
            InsertableLoginProviderAttributes::ColorId => write!(f, "color_id"),
            InsertableLoginProviderAttributes::ClientIdVarName => {
                write!(f, "client_id_var_name")
            }
            InsertableLoginProviderAttributes::RedirectUriVarName => {
                write!(f, "redirect_uri_var_name")
            }
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
    icon_id: i16,
    color_id: i16,
    client_id_var_name: String,
    redirect_uri_var_name: String,
    oauth_url: String,
    scope: String,
}
impl InsertableLoginProvider {
    #[cfg(feature = "postgres")]
    pub async fn icon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::icons::Icon, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::icons::Icon::table()
            .filter(crate::codegen::diesel_codegen::tables::icons::icons::dsl::id.eq(&self.icon_id))
            .first::<crate::codegen::structs_codegen::tables::icons::Icon>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn color(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::colors::Color::table()
            .filter(
                crate::codegen::diesel_codegen::tables::colors::colors::dsl::id.eq(&self.color_id),
            )
            .first::<crate::codegen::structs_codegen::tables::colors::Color>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableLoginProviderBuilder {
    name: Option<String>,
    icon_id: Option<i16>,
    color_id: Option<i16>,
    client_id_var_name: Option<String>,
    redirect_uri_var_name: Option<String>,
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
    pub fn icon_id(
        mut self,
        icon_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon_id = Some(icon_id);
        Ok(self)
    }
    pub fn color_id(
        mut self,
        color_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.color_id = Some(color_id);
        Ok(self)
    }
    pub fn client_id_var_name(
        mut self,
        client_id_var_name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.client_id_var_name = Some(client_id_var_name);
        Ok(self)
    }
    pub fn redirect_uri_var_name(
        mut self,
        redirect_uri_var_name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.redirect_uri_var_name = Some(redirect_uri_var_name);
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
            icon_id: self.icon_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableLoginProviderAttributes::IconId,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableLoginProviderAttributes::ColorId,
                ),
            )?,
            client_id_var_name: self.client_id_var_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableLoginProviderAttributes::ClientIdVarName,
                ),
            )?,
            redirect_uri_var_name: self.redirect_uri_var_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableLoginProviderAttributes::RedirectUriVarName,
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
            .icon_id(insertable_variant.icon_id)?
            .color_id(insertable_variant.color_id)?
            .client_id_var_name(insertable_variant.client_id_var_name)?
            .redirect_uri_var_name(insertable_variant.redirect_uri_var_name)?
            .oauth_url(insertable_variant.oauth_url)?
            .scope(insertable_variant.scope)
    }
}
