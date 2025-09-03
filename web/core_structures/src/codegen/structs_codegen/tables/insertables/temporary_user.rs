#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTemporaryUserAttributes {
    Id,
    Email,
    LoginProviderId,
}
impl core::str::FromStr for InsertableTemporaryUserAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Email" => Ok(Self::Email),
            "LoginProviderId" => Ok(Self::LoginProviderId),
            "email" => Ok(Self::Email),
            "login_provider_id" => Ok(Self::LoginProviderId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
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
/// Trait defining setters for attributes of an instance of `TemporaryUser` or
/// descendant tables.
pub trait TemporaryUserBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.temporary_user.email` column.
    ///
    /// # Arguments
    /// * `email`: The value to set for the `public.temporary_user.email`
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
    fn email<E>(
        self,
        email: E,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        E: TryInto<String>,
        validation_errors::SingleFieldError: From<<E as TryInto<String>>::Error>;
    /// Sets the value of the `public.temporary_user.login_provider_id` column.
    ///
    /// # Arguments
    /// * `login_provider_id`: The value to set for the
    ///   `public.temporary_user.login_provider_id` column.
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
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn login_provider(
        self,
        login_provider_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl TemporaryUserBuildable for InsertableTemporaryUserBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserAttributes;
    /// Sets the value of the `public.temporary_user.email` column.
    fn email<E>(
        mut self,
        email: E,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        E: TryInto<String>,
        validation_errors::SingleFieldError: From<<E as TryInto<String>>::Error>,
    {
        let email = email.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTemporaryUserAttributes::Email)
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
    /// Sets the value of the `public.temporary_user.login_provider_id` column.
    fn login_provider(
        mut self,
        login_provider_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTemporaryUserBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
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
