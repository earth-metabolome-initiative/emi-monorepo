#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::users::users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
    pub updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for User {
    const TABLE_NAME: &'static str = "users";
}
impl diesel::Identifiable for User {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl User {
    #[cfg(feature = "postgres")]
    pub fn from_first_name(
        first_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::users::users;
        Self::table()
            .filter(users::first_name.eq(first_name))
            .order_by(users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_last_name(
        last_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::users::users;
        Self::table()
            .filter(users::last_name.eq(last_name))
            .order_by(users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::users::users;
        Self::table()
            .filter(users::created_at.eq(created_at))
            .order_by(users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::users::users;
        Self::table()
            .filter(users::updated_at.eq(updated_at))
            .order_by(users::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<User> for User {
    fn as_ref(&self) -> &User {
        self
    }
}
