#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
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
    pub fn email<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::user_emails::UserEmail: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::user_emails::UserEmail::table(),
                self.email_id,
            ),
            conn,
        )
    }
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
pub struct InsertableEmailProviderBuilder {
    pub(crate) email_id: Option<i32>,
    pub(crate) login_provider_id: Option<i16>,
}
impl InsertableEmailProviderBuilder {
    pub fn email_id<P>(
        mut self,
        email_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableEmailProviderAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let email_id = email_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableEmailProviderAttributes::EmailId)
        })?;
        self.email_id = Some(email_id);
        Ok(self)
    }
    pub fn login_provider_id<P>(
        mut self,
        login_provider_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableEmailProviderAttributes>>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let login_provider_id =
            login_provider_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
                Into::into(err).rename_field(InsertableEmailProviderAttributes::LoginProviderId)
            })?;
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
}
impl TryFrom<InsertableEmailProviderBuilder> for InsertableEmailProvider {
    type Error = common_traits::prelude::BuilderError<InsertableEmailProviderAttributes>;
    fn try_from(
        builder: InsertableEmailProviderBuilder,
    ) -> Result<InsertableEmailProvider, Self::Error> {
        Ok(Self {
            email_id: builder.email_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableEmailProviderAttributes::EmailId,
                ),
            )?,
            login_provider_id: builder.login_provider_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableEmailProviderAttributes::LoginProviderId,
                ),
            )?,
        })
    }
}
