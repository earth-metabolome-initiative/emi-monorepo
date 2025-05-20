#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTemporaryUserAttributes {
    Email,
    LoginProviderId,
}
impl core::fmt::Display for InsertableTemporaryUserAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableTemporaryUserAttributes::Email => write!(f, "email"),
            InsertableTemporaryUserAttributes::LoginProviderId => {
                write!(f, "login_provider_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::temporary_user::temporary_user
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTemporaryUser {
    email: String,
    login_provider_id: i16,
}
impl InsertableTemporaryUser {
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
pub struct InsertableTemporaryUserBuilder {
    email: Option<String>,
    login_provider_id: Option<i16>,
}
impl InsertableTemporaryUserBuilder {
    pub fn email<P>(
        mut self,
        email: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let email = email.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTemporaryUserAttributes::Email)
        })?;
        self.email = Some(email);
        Ok(self)
    }
    pub fn login_provider_id<P>(
        mut self,
        login_provider_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let login_provider_id =
            login_provider_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
                Into::into(err).rename_field(InsertableTemporaryUserAttributes::LoginProviderId)
            })?;
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableTemporaryUserBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTemporaryUserAttributes>;
    type Object = InsertableTemporaryUser;
    type Attribute = InsertableTemporaryUserAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            email: self.email.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTemporaryUserAttributes::Email,
            ))?,
            login_provider_id: self.login_provider_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTemporaryUserAttributes::LoginProviderId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableTemporaryUser> for InsertableTemporaryUserBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTemporaryUser) -> Result<Self, Self::Error> {
        Self::default()
            .email(insertable_variant.email)?
            .login_provider_id(insertable_variant.login_provider_id)
    }
}
