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
    pub id: rosetta_uuid::Uuid,
    pub name: String,
    pub parent: Option<rosetta_uuid::Uuid>,
}
impl diesel::Identifiable for DirectusFolder {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusFolder {
    #[cfg(feature = "postgres")]
    pub async fn parent(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(parent) = self.parent.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_folders::directus_folders::dsl::id
                    .eq(parent),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>(
                conn,
            )
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent(
        conn: &mut diesel::PgConnection,
        parent: &crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_folders::directus_folders::dsl::parent
                    .eq(parent.id),
            )
            .load::<Self>(conn)
            .await
    }
}
