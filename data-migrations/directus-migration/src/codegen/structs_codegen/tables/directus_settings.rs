#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "64-column-tables",
    derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)
)]
#[cfg_attr(feature = "64-column-tables", diesel(primary_key(id)))]
#[cfg_attr(
    feature = "64-column-tables",
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::directus_settings::directus_settings
    )
)]
pub struct DirectusSetting {
    pub id: i32,
    pub project_name: String,
    pub project_url: Option<String>,
    pub project_color: String,
    pub project_logo: Option<uuid::Uuid>,
    pub public_foreground: Option<uuid::Uuid>,
    pub public_background: Option<uuid::Uuid>,
    pub public_note: Option<String>,
    pub auth_login_attempts: Option<i32>,
    pub auth_password_policy: Option<String>,
    pub storage_asset_transform: Option<String>,
    pub storage_asset_presets: Option<serde_json::Value>,
    pub custom_css: Option<String>,
    pub storage_default_folder: Option<uuid::Uuid>,
    pub basemaps: Option<serde_json::Value>,
    pub mapbox_key: Option<String>,
    pub module_bar: Option<serde_json::Value>,
    pub project_descriptor: Option<String>,
    pub default_language: String,
    pub custom_aspect_ratios: Option<serde_json::Value>,
    pub public_favicon: Option<uuid::Uuid>,
    pub default_appearance: String,
    pub default_theme_light: Option<String>,
    pub theme_light_overrides: Option<serde_json::Value>,
    pub default_theme_dark: Option<String>,
    pub theme_dark_overrides: Option<serde_json::Value>,
    pub report_error_url: Option<String>,
    pub report_bug_url: Option<String>,
    pub report_feature_url: Option<String>,
    pub public_registration: bool,
    pub public_registration_verify_email: bool,
    pub public_registration_role: Option<uuid::Uuid>,
    pub public_registration_email_filter: Option<serde_json::Value>,
}
impl DirectusSetting {
    #[cfg(feature = "postgres")]
    pub async fn project_logo(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(project_logo) = self.project_logo.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .find(project_logo)
            .first::<
                crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_foreground(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(public_foreground) = self.public_foreground.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .find(public_foreground)
            .first::<
                crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_background(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(public_background) = self.public_background.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .find(public_background)
            .first::<
                crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn storage_default_folder(
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
        let Some(storage_default_folder) = self.storage_default_folder.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::table()
            .find(storage_default_folder)
            .first::<
                crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_favicon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(public_favicon) = self.public_favicon.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .find(public_favicon)
            .first::<
                crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_registration_role(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(public_registration_role) = self.public_registration_role.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::table()
            .find(public_registration_role)
            .first::<
                crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
            >(conn)
            .await
            .map(Some)
    }
}
