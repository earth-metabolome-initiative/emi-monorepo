#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks
)]
pub struct DirectusWebhook {
    pub id: i32,
    pub name: String,
    pub method: String,
    pub url: String,
    pub status: String,
    pub data: bool,
    pub actions: String,
    pub collections: String,
    pub headers: Option<serde_json::Value>,
    pub was_active_before_deprecation: bool,
    pub migrated_flow: Option<rosetta_uuid::Uuid>,
}
impl DirectusWebhook {
    #[cfg(feature = "postgres")]
    pub async fn migrated_flow(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(migrated_flow) = self.migrated_flow.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_flows::directus_flows::dsl::id
                    .eq(migrated_flow),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_migrated_flow(
        conn: &mut diesel_async::AsyncPgConnection,
        migrated_flow: &crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks::dsl::migrated_flow
                    .eq(migrated_flow.id),
            )
            .load::<Self>(conn)
            .await
    }
}
