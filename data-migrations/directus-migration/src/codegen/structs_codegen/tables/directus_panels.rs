#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_panels::directus_panels
)]
pub struct DirectusPanel {
    pub id: rosetta_uuid::Uuid,
    pub dashboard: rosetta_uuid::Uuid,
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
    pub date_created: Option<rosetta_timestamp::TimestampUTC>,
    pub user_created: Option<rosetta_uuid::Uuid>,
}
impl diesel::Identifiable for DirectusPanel {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards::dsl::id
                    .eq(&self.dashboard),
            )
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(user_created) = self.user_created.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user_created),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_dashboard(
        conn: &mut diesel_async::AsyncPgConnection,
        dashboard: &crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_panels::directus_panels::dsl::dashboard
                    .eq(dashboard.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user_created(
        conn: &mut diesel_async::AsyncPgConnection,
        user_created: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_panels::directus_panels::dsl::user_created
                    .eq(user_created.id),
            )
            .load::<Self>(conn)
            .await
    }
}
