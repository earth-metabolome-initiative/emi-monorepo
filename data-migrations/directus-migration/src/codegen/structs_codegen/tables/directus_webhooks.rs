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
    pub migrated_flow: Option<uuid::Uuid>,
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
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(migrated_flow) = self.migrated_flow.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow::table()
            .find(migrated_flow)
            .first::<
                crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
            >(conn)
            .await
            .map(Some)
    }
}
