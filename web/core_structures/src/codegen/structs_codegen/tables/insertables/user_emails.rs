#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUserEmailAttributes {
    Id,
    Email,
    CreatedBy,
    CreatedAt,
    PrimaryEmail,
}
impl core::fmt::Display for InsertableUserEmailAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Email => write!(f, "email"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::PrimaryEmail => write!(f, "primary_email"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::user_emails::user_emails)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserEmail {
    pub(crate) email: String,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) primary_email: bool,
}
impl InsertableUserEmail {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserEmailBuilder {
    pub(crate) email: Option<String>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) primary_email: Option<bool>,
}
impl Default for InsertableUserEmailBuilder {
    fn default() -> Self {
        Self {
            email: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            primary_email: Some(true),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableUserEmailBuilder {
    type Attributes = InsertableUserEmailAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(email) = other.email {
            self = self.email(email)?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        if let Some(created_at) = other.created_at {
            self = self.created_at(created_at)?;
        }
        if let Some(primary_email) = other.primary_email {
            self = self.primary_email(primary_email)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableUserEmailBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder {
    /// Sets the value of the `user_emails.created_at` column from table
    /// `user_emails`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserEmailAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableUserEmailAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder {
    /// Sets the value of the `user_emails.created_by` column from table
    /// `user_emails`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserEmailAttributes>> {
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder {
    /// Sets the value of the `user_emails.email` column from table
    /// `user_emails`.
    pub fn email<P>(
        mut self,
        email: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserEmailAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let email = email.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableUserEmailAttributes::Email)
        })?;
        pgrx_validation::must_be_email(email.as_ref())
            .map_err(|e| e.rename_field(InsertableUserEmailAttributes::Email))?;
        self.email = Some(email);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder {
    /// Sets the value of the `user_emails.primary_email` column from table
    /// `user_emails`.
    pub fn primary_email<P>(
        mut self,
        primary_email: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserEmailAttributes>>
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let primary_email =
            primary_email.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
                Into::into(err).rename_field(InsertableUserEmailAttributes::PrimaryEmail)
            })?;
        self.primary_email = Some(primary_email);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableUserEmailBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::user_emails::UserEmail,
            Error = web_common_traits::database::InsertError<InsertableUserEmailAttributes>,
        >,
{
    type Attributes = InsertableUserEmailAttributes;
    fn is_complete(&self) -> bool {
        self.email.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.primary_email.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::user_emails::UserEmail =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
