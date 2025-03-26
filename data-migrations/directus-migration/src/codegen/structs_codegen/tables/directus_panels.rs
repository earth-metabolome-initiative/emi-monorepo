#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_panels::directus_panels
)]
pub struct DirectusPanel {
    pub id: uuid::Uuid,
    pub dashboard: uuid::Uuid,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub show_header: bool,
    pub note: Option<String>,
    pub r#type: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub options: Option<serde_json::Value>,
    pub date_created: Option<chrono::NaiveDateTime>,
    pub user_created: Option<uuid::Uuid>,
}
impl DirectusPanel {
    #[cfg(feature = "postgres")]
    pub async fn dashboard(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard::table()
            .find(&self.dashboard)
            .first::<
                crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn user_created(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(user_created) = self.user_created.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .find(user_created)
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
}
