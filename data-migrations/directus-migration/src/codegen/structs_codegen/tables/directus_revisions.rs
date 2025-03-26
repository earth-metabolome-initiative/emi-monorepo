#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions
)]
pub struct DirectusRevision {
    pub id: i32,
    pub activity: i32,
    pub collection: String,
    pub item: String,
    pub data: Option<serde_json::Value>,
    pub delta: Option<serde_json::Value>,
    pub parent: Option<i32>,
    pub version: Option<uuid::Uuid>,
}
impl DirectusRevision {
    #[cfg(feature = "postgres")]
    pub async fn activity(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity::table()
            .find(&self.activity)
            .first::<
                crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn parent(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision,
        >,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(parent) = self.parent.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision::table()
            .find(parent)
            .first::<
                crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn version(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
        >,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(version) = self.version.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion::table()
            .find(version)
            .first::<
                crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
            >(conn)
            .await
            .map(Some)
    }
}
