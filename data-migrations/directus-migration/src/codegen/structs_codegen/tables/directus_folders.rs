#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_folders::directus_folders
)]
pub struct DirectusFolder {
    pub id: uuid::Uuid,
    pub name: String,
    pub parent: Option<uuid::Uuid>,
}
impl DirectusFolder {
    #[cfg(feature = "postgres")]
    pub async fn parent(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
        >,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(parent) = self.parent.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::table()
            .find(parent)
            .first::<
                crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent(
        conn: &mut diesel_async::AsyncPgConnection,
        parent: &crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_folders::directus_folders::dsl::parent
                    .eq(&parent.id),
            )
            .load::<Self>(conn)
            .await
    }
}
