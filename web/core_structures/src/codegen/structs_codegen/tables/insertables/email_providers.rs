#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableEmailProviderAttributes {
    EmailId,
    LoginProviderId,
}
impl core::fmt::Display for InsertableEmailProviderAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::EmailId => write!(f, "email_id"),
            Self::LoginProviderId => write!(f, "login_provider_id"),
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
    pub(crate) email_id: i32,
    pub(crate) login_provider_id: i16,
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::login_providers::LoginProvider::table(),
                self.login_provider_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableEmailProviderBuilder {
    pub(crate) email_id: Option<i32>,
    pub(crate) login_provider_id: Option<i16>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableEmailProviderBuilder {
    type Attributes = InsertableEmailProviderAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(email_id) = other.email_id {
            self = self.email(email_id)?;
        }
        if let Some(login_provider_id) = other.login_provider_id {
            self = self.login_provider(login_provider_id)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableEmailProviderBuilder {
    type PrimaryKey = (i32, i16);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder {
    /// Sets the value of the `email_providers.email_id` column from table
    /// `email_providers`.
    pub fn email(
        mut self,
        email_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableEmailProviderAttributes>>
    {
        self.email_id = Some(email_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder {
    /// Sets the value of the `email_providers.login_provider_id` column from
    /// table `email_providers`.
    pub fn login_provider(
        mut self,
        login_provider_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableEmailProviderAttributes>>
    {
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableEmailProviderBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
            Error = web_common_traits::database::InsertError<InsertableEmailProviderAttributes>,
        >,
{
    type Attributes = InsertableEmailProviderAttributes;
    fn is_complete(&self) -> bool {
        self.email_id.is_some() && self.login_provider_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::email_providers::EmailProvider =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
