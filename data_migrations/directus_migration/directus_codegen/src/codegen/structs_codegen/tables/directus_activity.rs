#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_activity::directus_activity
)]
pub struct DirectusActivity {
    pub id: i32,
    pub action: String,
    pub user: Option<::rosetta_uuid::Uuid>,
    pub timestamp: ::rosetta_timestamp::TimestampUTC,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub collection: String,
    pub item: String,
    pub origin: Option<String>,
}
impl web_common_traits::prelude::TableName for DirectusActivity {
    const TABLE_NAME: &'static str = "directus_activity";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
    > for DirectusActivity
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for DirectusActivity {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusActivity {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusActivity {
    #[cfg(feature = "postgres")]
    pub fn from_action(
        action: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::action.eq(action))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user(
        user: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::user.eq(user))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_timestamp<C>(
        timestamp: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_activity::directus_activity::timestamp as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_activity::directus_activity::timestamp as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_activity::directus_activity::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_activity::directus_activity::timestamp as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_activity::directus_activity::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::timestamp.eq(timestamp))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_ip(
        ip: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::ip.eq(ip))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_agent(
        user_agent: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::user_agent.eq(user_agent))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_collection(
        collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::collection.eq(collection))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_item(
        item: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::item.eq(item))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_origin(
        origin: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
        Self::table()
            .filter(directus_activity::origin.eq(origin))
            .order_by(directus_activity::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusActivity> for DirectusActivity {
    fn as_ref(&self) -> &DirectusActivity {
        self
    }
}
