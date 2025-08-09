#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTemporaryUserAttributes {
    Id,
    Email,
    LoginProviderId,
}
impl core::fmt::Display for InsertableTemporaryUserAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Email => write!(f, "email"),
            Self::LoginProviderId => write!(f, "login_provider_id"),
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
    pub(crate) email: String,
    pub(crate) login_provider_id: i16,
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
pub struct InsertableTemporaryUserBuilder {
    pub(crate) email: Option<String>,
    pub(crate) login_provider_id: Option<i16>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableTemporaryUserBuilder {
    type Attributes = InsertableTemporaryUserAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(email) = other.email {
            self = self.email(email)?;
        }
        if let Some(login_provider_id) = other.login_provider_id {
            self = self.login_provider(login_provider_id)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTemporaryUserBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder {
    /// Sets the value of the `temporary_user.email` column from table
    /// `temporary_user`.
    pub fn email<Email>(
        mut self,
        email: Email,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTemporaryUserAttributes>>
    where
        Email: TryInto<String>,
        <Email as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let email = email.try_into().map_err(|err: <Email as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTemporaryUserAttributes::Email)
        })?;
        pgrx_validation::must_be_email(email.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserAttributes::Email,
                    )
            })?;
        self.email = Some(email);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder {
    /// Sets the value of the `temporary_user.login_provider_id` column from
    /// table `temporary_user`.
    pub fn login_provider(
        mut self,
        login_provider_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTemporaryUserAttributes>>
    {
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableTemporaryUserBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser,
            Error = web_common_traits::database::InsertError<InsertableTemporaryUserAttributes>,
        >,
{
    type Attributes = InsertableTemporaryUserAttributes;
    fn is_complete(&self) -> bool {
        self.email.is_some() && self.login_provider_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
