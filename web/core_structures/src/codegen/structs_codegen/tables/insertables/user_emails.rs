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
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
}
pub struct InsertableUserEmailBuilder {
    email: Option<String>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    primary_email: Option<bool>,
}
impl Default for InsertableUserEmailBuilder {
    fn default() -> Self {
        Self {
            email: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            primary_email: Some(true),
        }
    }
}
impl InsertableUserEmailBuilder {
    pub fn email<P>(
        mut self,
        email: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let email = email.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableUserEmailAttributes::Email)
        })?;
        self.email = Some(email);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
impl common_traits::prelude::Builder for InsertableUserEmailBuilder {
    type Error = web_common_traits::database::InsertError<InsertableUserEmailAttributes>;
    type Object = InsertableUserEmail;
    type Attribute = InsertableUserEmailAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            email: self.email.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableUserEmailAttributes::Email,
            ))?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::CreatedAt,
                ),
            )?,
            primary_email: self.primary_email.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::PrimaryEmail,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableUserEmail> for InsertableUserEmailBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableUserEmail) -> Result<Self, Self::Error> {
        Self::default()
            .email(insertable_variant.email)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .primary_email(insertable_variant.primary_email)
    }
}
