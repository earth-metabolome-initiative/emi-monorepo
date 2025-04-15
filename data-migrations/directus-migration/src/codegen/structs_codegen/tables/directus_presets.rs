#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_presets::directus_presets
)]
pub struct DirectusPreset {
    pub id: i32,
    pub bookmark: Option<String>,
    pub user: Option<rosetta_uuid::Uuid>,
    pub role: Option<rosetta_uuid::Uuid>,
    pub collection: Option<String>,
    pub search: Option<String>,
    pub layout: Option<String>,
    pub layout_query: Option<serde_json::Value>,
    pub layout_options: Option<serde_json::Value>,
    pub refresh_interval: Option<i32>,
    pub filter: Option<serde_json::Value>,
    pub icon: Option<String>,
    pub color: Option<String>,
}
impl DirectusPreset {
    #[cfg(feature = "postgres")]
    pub async fn user(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(user) = self.user.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn role(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(role) = self.role.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_roles::directus_roles::dsl::id
                    .eq(role),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user(
        conn: &mut diesel_async::AsyncPgConnection,
        user: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_presets::directus_presets::dsl::user
                    .eq(user.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_role(
        conn: &mut diesel_async::AsyncPgConnection,
        role: &crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_presets::directus_presets::dsl::role
                    .eq(role.id),
            )
            .load::<Self>(conn)
            .await
    }
}
