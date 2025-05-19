#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTemporaryUserEmailAttributes {
    Id,
    Email,
    LoginProviderId,
    Validated,
}
impl core::fmt::Display for InsertableTemporaryUserEmailAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableTemporaryUserEmailAttributes::Id => write!(f, "id"),
            InsertableTemporaryUserEmailAttributes::Email => write!(f, "email"),
            InsertableTemporaryUserEmailAttributes::LoginProviderId => {
                write!(f, "login_provider_id")
            }
            InsertableTemporaryUserEmailAttributes::Validated => write!(f, "validated"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::temporary_user_emails::temporary_user_emails
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTemporaryUserEmail {
    id: rosetta_uuid::Uuid,
    email: String,
    login_provider_id: i16,
    validated: bool,
}
impl InsertableTemporaryUserEmail {
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
pub struct InsertableTemporaryUserEmailBuilder {
    id: Option<rosetta_uuid::Uuid>,
    email: Option<String>,
    login_provider_id: Option<i16>,
    validated: Option<bool>,
}
impl Default for InsertableTemporaryUserEmailBuilder {
    fn default() -> Self {
        Self {
            id: Some(rosetta_uuid::Uuid::new_v4()),
            email: None,
            login_provider_id: None,
            validated: Some(false),
        }
    }
}
impl InsertableTemporaryUserEmailBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableTemporaryUserEmailAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn email<P>(
        mut self,
        email: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let email = email.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTemporaryUserEmailAttributes::Email)
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
                Into::into(err)
                    .rename_field(InsertableTemporaryUserEmailAttributes::LoginProviderId)
            })?;
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
    pub fn validated<P>(
        mut self,
        validated: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let validated = validated.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableTemporaryUserEmailAttributes::Validated)
        })?;
        self.validated = Some(validated);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableTemporaryUserEmailBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTemporaryUserEmailAttributes>;
    type Object = InsertableTemporaryUserEmail;
    type Attribute = InsertableTemporaryUserEmailAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTemporaryUserEmailAttributes::Id,
            ))?,
            email: self.email.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTemporaryUserEmailAttributes::Email,
            ))?,
            login_provider_id: self.login_provider_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTemporaryUserEmailAttributes::LoginProviderId,
                ),
            )?,
            validated: self.validated.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTemporaryUserEmailAttributes::Validated,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableTemporaryUserEmail> for InsertableTemporaryUserEmailBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTemporaryUserEmail) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .email(insertable_variant.email)?
            .login_provider_id(insertable_variant.login_provider_id)?
            .validated(insertable_variant.validated)
    }
}
