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
impl<'a> From<&'a User>
    for web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder,
    >
{
    fn from(value: &'a User) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<crate::codegen::structs_codegen::tables::users::User>
    for User
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for User {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for User {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl User {
    #[cfg(feature = "postgres")]
    pub fn from_first_name_and_last_name(
        first_name: &str,
        last_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::users::users;
        Self::table()
            .filter(users::first_name.eq(first_name).and(users::last_name.eq(last_name)))
            .order_by(users::id.asc())
            .load::<Self>(conn)
    }
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
}
impl AsRef<User> for User {
    fn as_ref(&self) -> &User {
        self
    }
}
