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
    table_name = crate::codegen::diesel_codegen::tables::directus_folders::directus_folders
)]
pub struct DirectusFolder {
    pub id: ::rosetta_uuid::Uuid,
    pub name: String,
    pub parent: Option<::rosetta_uuid::Uuid>,
}
impl web_common_traits::prelude::TableName for DirectusFolder {
    const TABLE_NAME: &'static str = "directus_folders";
}
impl<C> web_common_traits::prelude::Ancestor<C> for DirectusFolder
where
    Self: web_common_traits::prelude::TableName + Sized,
    C: diesel::connection::LoadConnection,
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization
        + diesel::sql_types::HasSqlType<::rosetta_uuid::diesel_impls::Uuid>
        + 'static,
    web_common_traits::prelude::AncestorExists: diesel::deserialize::FromSqlRow<
            diesel::sql_types::Untyped,
            <C as diesel::Connection>::Backend,
        >,
    for<'a> &'a Self: diesel::Identifiable,
    for<'a> <&'a Self as diesel::Identifiable>::Id:
        diesel::serialize::ToSql<::rosetta_uuid::diesel_impls::Uuid, C::Backend>,
{
    const PARENT_ID: &'static str = "parent";
    const ID: &'static str = "id";
    type SqlType = ::rosetta_uuid::diesel_impls::Uuid;
}
impl web_common_traits::prelude::Descendant<DirectusFolder> for DirectusFolder {
    fn parent_id(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
        self.parent.as_ref()
    }
}
impl diesel::Identifiable for DirectusFolder {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusFolder {
    pub fn parent<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(parent) = self.parent else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::table(),
                parent,
            ),
            conn,
        )
        .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_folders::directus_folders;
        Self::table()
            .filter(directus_folders::name.eq(name))
            .order_by(directus_folders::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent(
        parent: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_folders::directus_folders;
        Self::table()
            .filter(directus_folders::parent.eq(parent))
            .order_by(directus_folders::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusFolder> for DirectusFolder {
    fn as_ref(&self) -> &DirectusFolder {
        self
    }
}
