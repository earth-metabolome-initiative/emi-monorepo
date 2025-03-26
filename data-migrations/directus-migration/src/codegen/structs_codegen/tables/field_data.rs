#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "64-column-tables",
    derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)
)]
#[cfg_attr(feature = "64-column-tables", diesel(primary_key(id)))]
#[cfg_attr(
    feature = "64-column-tables",
    diesel(table_name = crate::codegen::diesel_codegen::tables::field_data::field_data)
)]
pub struct FieldDatum {
    pub id: i32,
    pub user_created: Option<uuid::Uuid>,
    pub date_created: Option<chrono::NaiveDateTime>,
    pub user_updated: Option<uuid::Uuid>,
    pub date_updated: Option<chrono::NaiveDateTime>,
    pub collector_fullname: Option<String>,
    pub observation_subject: Option<String>,
    pub inat_upload: Option<i32>,
    pub is_wild: Option<i32>,
    pub taxon_name: Option<String>,
    pub no_name_on_list: Option<i32>,
    pub sample_id: String,
    pub picture_panel: Option<String>,
    pub picture_general: Option<String>,
    pub picture_detail: Option<String>,
    pub picture_cut: Option<String>,
    pub picture_panel_label: Option<String>,
    pub x_coord: Option<f32>,
    pub y_coord: Option<f32>,
    pub collector_orcid: Option<String>,
    pub collector_inat: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub qfield_project: Option<String>,
    pub picture_free: Option<String>,
    pub comment_eco: Option<String>,
    pub weather: Option<String>,
    pub sample_name: Option<String>,
    pub name_proposition: Option<String>,
    pub ipen: Option<String>,
    pub match_name: Option<String>,
    pub ott_id: Option<String>,
    pub rank: Option<String>,
    pub wikidata_id: Option<String>,
    pub unknown: Option<String>,
    pub comment_env: Option<String>,
    pub herbivory_percent: Option<f32>,
    pub temperature_celsius: Option<f32>,
    pub geometry: Option<postgis_diesel::types::Point>,
    pub date: Option<i64>,
}
impl FieldDatum {
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
    #[cfg(feature = "postgres")]
    pub async fn user_updated(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(user_updated) = self.user_updated.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .find(user_updated)
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_sample_id(
        sample_id: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::field_data::field_data::sample_id,
                    sample_id,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
