#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableEmailProviderAttributes {
    EmailId,
    LoginProviderId,
}
impl core::fmt::Display for InsertableEmailProviderAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableEmailProviderAttributes::EmailId => write!(f, "email_id"),
            InsertableEmailProviderAttributes::LoginProviderId => {
                write!(f, "login_provider_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::email_providers::email_providers
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableEmailProvider {
    email_id: i32,
    login_provider_id: i16,
}
impl InsertableEmailProvider {
    #[cfg(feature = "postgres")]
    pub async fn email(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::user_emails::UserEmail::table()
            .filter(
                crate::codegen::diesel_codegen::tables::user_emails::user_emails::dsl::id
                    .eq(&self.email_id),
            )
            .first::<crate::codegen::structs_codegen::tables::user_emails::UserEmail>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn login_provider(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider::table()
            .filter(
                crate::codegen::diesel_codegen::tables::login_providers::login_providers::dsl::id
                    .eq(&self.login_provider_id),
            )
            .first::<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableEmailProviderBuilder {
    email_id: Option<i32>,
    login_provider_id: Option<i16>,
}
impl InsertableEmailProviderBuilder {
    pub fn email_id(
        mut self,
        email_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.email_id = Some(email_id);
        Ok(self)
    }
    pub fn login_provider_id(
        mut self,
        login_provider_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableEmailProviderBuilder {
    type Error = web_common_traits::database::InsertError<InsertableEmailProviderAttributes>;
    type Object = InsertableEmailProvider;
    type Attribute = InsertableEmailProviderAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            email_id: self.email_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableEmailProviderAttributes::EmailId,
                ),
            )?,
            login_provider_id: self.login_provider_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableEmailProviderAttributes::LoginProviderId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableEmailProvider> for InsertableEmailProviderBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableEmailProvider) -> Result<Self, Self::Error> {
        Self::default()
            .email_id(insertable_variant.email_id)?
            .login_provider_id(insertable_variant.login_provider_id)
    }
}
