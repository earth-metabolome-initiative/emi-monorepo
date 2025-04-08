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
    pub version: Option<rosetta_uuid::Uuid>,
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
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_activity::directus_activity::dsl::id
                    .eq(&self.activity),
            )
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
        Option<crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(parent) = self.parent.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions::dsl::id
                    .eq(parent),
            )
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
        Option<crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(version) = self.version.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_versions::directus_versions::dsl::id
                    .eq(version),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_activity(
        conn: &mut diesel_async::AsyncPgConnection,
        activity: &crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions::dsl::activity
                    .eq(activity.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent(
        conn: &mut diesel_async::AsyncPgConnection,
        parent: &crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions::dsl::parent
                    .eq(parent.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_version(
        conn: &mut diesel_async::AsyncPgConnection,
        version: &crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions::dsl::version
                    .eq(version.id),
            )
            .load::<Self>(conn)
            .await
    }
}
