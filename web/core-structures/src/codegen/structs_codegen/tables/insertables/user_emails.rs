#[derive(Clone, core::fmt::Debug)]
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
    created_at: rosetta_timestamp::TimestampUTC,
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
#[derive(Default)]
pub struct InsertableUserEmailBuilder {
    email: Option<String>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    primary_email: Option<bool>,
}
impl InsertableUserEmailBuilder {
    pub fn email(
        mut self,
        email: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.email = Some(email);
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at(
        mut self,
        created_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn primary_email(
        mut self,
        primary_email: bool,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
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
            .primary_email(insertable_variant.primary_email)?
    }
}
