#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(belongs_to(crate::User, foreign_key = created_by))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::user_emails::user_emails)]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
    pub primary_email: bool,
}
impl web_common_traits::prelude::TableName for UserEmail {
    const TABLE_NAME: &'static str = "user_emails";
}
impl web_common_traits::prelude::ExtensionTable<crate::UserEmail> for UserEmail where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>
{
}
impl diesel::Identifiable for UserEmail {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl UserEmail {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::User, diesel::result::Error>
    where
        crate::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::User::read(self.created_by, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_email(
        email: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::user_emails::user_emails;
        Self::table()
            .filter(user_emails::email.eq(email))
            .order_by(user_emails::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_email_and_created_by(
        email: &str,
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::user_emails::user_emails;
        Self::table()
            .filter(user_emails::email.eq(email).and(user_emails::created_by.eq(created_by)))
            .order_by(user_emails::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by_and_primary_email(
        created_by: &i32,
        primary_email: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::user_emails::user_emails;
        Self::table()
            .filter(
                user_emails::created_by
                    .eq(created_by)
                    .and(user_emails::primary_email.eq(primary_email)),
            )
            .order_by(user_emails::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::user_emails::user_emails;
        Self::table()
            .filter(user_emails::created_at.eq(created_at))
            .order_by(user_emails::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_primary_email(
        primary_email: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::user_emails::user_emails;
        Self::table()
            .filter(user_emails::primary_email.eq(primary_email))
            .order_by(user_emails::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<UserEmail> for UserEmail {
    fn as_ref(&self) -> &UserEmail {
        self
    }
}
