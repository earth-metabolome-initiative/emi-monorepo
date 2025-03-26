#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "32-column-tables",
    derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)
)]
#[cfg_attr(feature = "32-column-tables", diesel(primary_key(id)))]
#[cfg_attr(
    feature = "32-column-tables",
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::directus_files::directus_files
    )
)]
pub struct DirectusFile {
    pub id: uuid::Uuid,
    pub storage: String,
    pub filename_disk: Option<String>,
    pub filename_download: String,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub folder: Option<uuid::Uuid>,
    pub uploaded_by: Option<uuid::Uuid>,
    pub created_on: chrono::NaiveDateTime,
    pub modified_by: Option<uuid::Uuid>,
    pub modified_on: chrono::NaiveDateTime,
    pub charset: Option<String>,
    pub filesize: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub embed: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub tags: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub focal_point_x: Option<i32>,
    pub focal_point_y: Option<i32>,
    pub tus_id: Option<String>,
    pub tus_data: Option<serde_json::Value>,
    pub uploaded_on: Option<chrono::NaiveDateTime>,
}
impl DirectusFile {
    #[cfg(feature = "postgres")]
    pub async fn folder(
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
        let Some(folder) = self.folder.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::table()
            .find(folder)
            .first::<
                crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn uploaded_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(uploaded_by) = self.uploaded_by.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .find(uploaded_by)
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn modified_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(modified_by) = self.modified_by.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .find(modified_by)
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
}
