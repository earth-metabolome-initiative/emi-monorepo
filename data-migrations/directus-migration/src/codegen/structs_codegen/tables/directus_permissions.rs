#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions
)]
pub struct DirectusPermission {
    pub id: i32,
    pub collection: String,
    pub action: String,
    pub permissions: Option<serde_json::Value>,
    pub validation: Option<serde_json::Value>,
    pub presets: Option<serde_json::Value>,
    pub fields: Option<String>,
    pub policy: rosetta_uuid::Uuid,
}
impl DirectusPermission {
    #[cfg(feature = "postgres")]
    pub async fn policy(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_policies::directus_policies::dsl::id
                    .eq(&self.policy),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_policy(
        conn: &mut diesel_async::AsyncPgConnection,
        policy: &crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions::dsl::policy
                    .eq(policy.id),
            )
            .load::<Self>(conn)
            .await
    }
}
