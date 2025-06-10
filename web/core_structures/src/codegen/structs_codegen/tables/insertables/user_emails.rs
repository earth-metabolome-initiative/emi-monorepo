#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUserEmailAttributes {
    Email,
    CreatedBy,
    CreatedAt,
    PrimaryEmail,
}
impl core::fmt::Display for InsertableUserEmailAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableUserEmailAttributes::Email => write!(f, "email"),
            InsertableUserEmailAttributes::CreatedBy => write!(f, "created_by"),
            InsertableUserEmailAttributes::CreatedAt => write!(f, "created_at"),
            InsertableUserEmailAttributes::PrimaryEmail => write!(f, "primary_email"),
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
    email: String,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    primary_email: bool,
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
}
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
impl InsertableUserEmailBuilder {
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
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserEmailAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableUserEmailAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
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
impl TryFrom<InsertableUserEmailBuilder> for InsertableUserEmail {
    type Error = common_traits::prelude::BuilderError<InsertableUserEmailAttributes>;
    fn try_from(builder: InsertableUserEmailBuilder) -> Result<InsertableUserEmail, Self::Error> {
        Ok(Self {
            email: builder.email.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableUserEmailAttributes::Email,
            ))?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::CreatedAt,
                ),
            )?,
            primary_email: builder.primary_email.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::PrimaryEmail,
                ),
            )?,
        })
    }
}
