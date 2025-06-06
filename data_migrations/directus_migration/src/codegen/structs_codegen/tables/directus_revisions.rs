#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions
)]
pub struct DirectusRevision {
    pub id: i32,
    pub activity: i32,
    pub collection: String,
    pub item: String,
    pub data: Option<::serde_json::Value>,
    pub delta: Option<::serde_json::Value>,
    pub parent: Option<i32>,
    pub version: Option<::rosetta_uuid::Uuid>,
}
impl web_common_traits::prelude::TableName for DirectusRevision {
    const TABLE_NAME: &'static str = "directus_revisions";
}
impl<C> web_common_traits::prelude::Ancestor<C> for DirectusRevision
where
    Self: web_common_traits::prelude::TableName + Sized,
    C: diesel::connection::LoadConnection,
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization
        + diesel::sql_types::HasSqlType<diesel::sql_types::Integer> + 'static,
    web_common_traits::prelude::AncestorExists: diesel::deserialize::FromSqlRow<
        diesel::sql_types::Untyped,
        <C as diesel::Connection>::Backend,
    >,
    for<'a> &'a Self: diesel::Identifiable,
    for<'a> <&'a Self as diesel::Identifiable>::Id: diesel::serialize::ToSql<
        diesel::sql_types::Integer,
        C::Backend,
    >,
{
    const PARENT_ID: &'static str = "parent";
    const ID: &'static str = "id";
    type SqlType = diesel::sql_types::Integer;
}
impl web_common_traits::prelude::Descendant<DirectusRevision> for DirectusRevision {
    fn parent_id(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
        self.parent.as_ref()
    }
}
impl diesel::Identifiable for DirectusRevision {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusRevision {
    pub fn activity<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{RunQueryDsl, QueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity::table(),
                self.activity,
            ),
            conn,
        )
    }
    pub fn parent<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{RunQueryDsl, QueryDsl};
        let Some(parent) = self.parent else {
            return Ok(None);
        };
        RunQueryDsl::first(
                QueryDsl::find(
                    crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision::table(),
                    parent,
                ),
                conn,
            )
            .map(Some)
    }
    pub fn version<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{RunQueryDsl, QueryDsl};
        let Some(version) = self.version else {
            return Ok(None);
        };
        RunQueryDsl::first(
                QueryDsl::find(
                    crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion::table(),
                    version,
                ),
                conn,
            )
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_activity(
        activity: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions;
        Self::table()
            .filter(directus_revisions::activity.eq(activity))
            .order_by(directus_revisions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_collection(
        collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions;
        Self::table()
            .filter(directus_revisions::collection.eq(collection))
            .order_by(directus_revisions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_item(
        item: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions;
        Self::table()
            .filter(directus_revisions::item.eq(item))
            .order_by(directus_revisions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent(
        parent: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions;
        Self::table()
            .filter(directus_revisions::parent.eq(parent))
            .order_by(directus_revisions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_version(
        version: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions;
        Self::table()
            .filter(directus_revisions::version.eq(version))
            .order_by(directus_revisions::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusRevision> for DirectusRevision {
    fn as_ref(&self) -> &DirectusRevision {
        self
    }
}
