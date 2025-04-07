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
    pub created_on: chrono::DateTime<chrono::Utc>,
    pub modified_by: Option<uuid::Uuid>,
    pub modified_on: chrono::DateTime<chrono::Utc>,
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
    pub uploaded_on: Option<chrono::DateTime<chrono::Utc>>,
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
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(folder) = self.folder.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_folders::directus_folders::dsl::id
                    .eq(folder),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_folder(
        conn: &mut diesel_async::AsyncPgConnection,
        folder: &crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::dsl::folder
                    .eq(&folder.id),
            )
            .load::<Self>(conn)
            .await
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
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(uploaded_by) = self.uploaded_by.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(uploaded_by),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_uploaded_by(
        conn: &mut diesel_async::AsyncPgConnection,
        uploaded_by: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::dsl::uploaded_by
                    .eq(&uploaded_by.id),
            )
            .load::<Self>(conn)
            .await
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
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(modified_by) = self.modified_by.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(modified_by),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_modified_by(
        conn: &mut diesel_async::AsyncPgConnection,
        modified_by: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::dsl::modified_by
                    .eq(&modified_by.id),
            )
            .load::<Self>(conn)
            .await
    }
}
