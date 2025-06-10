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
    pub fn login_provider<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::login_providers::LoginProvider::table(),
                self.login_provider_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableTemporaryUserBuilder {
    pub(crate) email: Option<String>,
    pub(crate) login_provider_id: Option<i16>,
}
impl InsertableTemporaryUserBuilder {
    pub fn email<P>(
        mut self,
        email: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTemporaryUserAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let email = email.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTemporaryUserAttributes::Email)
        })?;
        pgrx_validation::must_be_email(email.as_ref())
            .map_err(|e| e.rename_field(InsertableTemporaryUserAttributes::Email))?;
        self.email = Some(email);
        Ok(self)
    }
    pub fn login_provider_id<P>(
        mut self,
        login_provider_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTemporaryUserAttributes>>
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
impl TryFrom<InsertableTemporaryUserBuilder> for InsertableTemporaryUser {
    type Error = common_traits::prelude::BuilderError<InsertableTemporaryUserAttributes>;
    fn try_from(
        builder: InsertableTemporaryUserBuilder,
    ) -> Result<InsertableTemporaryUser, Self::Error> {
        Ok(Self {
            email: builder.email.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTemporaryUserAttributes::Email,
            ))?,
            login_provider_id: builder.login_provider_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTemporaryUserAttributes::LoginProviderId,
                ),
            )?,
        })
    }
}
