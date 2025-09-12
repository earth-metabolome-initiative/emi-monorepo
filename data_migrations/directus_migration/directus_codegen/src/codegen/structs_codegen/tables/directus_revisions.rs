#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
        foreign_key = activity
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision,
        foreign_key = parent
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
        foreign_key = version
    )
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
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision,
> for DirectusRevision
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
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
    fn parent(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
        self.parent.as_ref()
    }
}
impl diesel::Identifiable for DirectusRevision {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusRevision {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
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
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity::read(
            self.activity,
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
        crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        use diesel::OptionalExtension;
        let Some(parent) = self.parent else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision::read(
                parent,
                conn,
            )
            .optional()
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
        crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        use diesel::OptionalExtension;
        let Some(version) = self.version else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion::read(
                version,
                conn,
            )
            .optional()
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
}
impl AsRef<DirectusRevision> for DirectusRevision {
    fn as_ref(&self) -> &DirectusRevision {
        self
    }
}
