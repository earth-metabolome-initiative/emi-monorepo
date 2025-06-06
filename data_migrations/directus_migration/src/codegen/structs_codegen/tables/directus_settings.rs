#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_settings::directus_settings
)]
pub struct DirectusSetting {
    pub id: i32,
    pub project_name: String,
    pub project_url: Option<String>,
    pub project_color: String,
    pub project_logo: Option<rosetta_uuid::Uuid>,
    pub public_foreground: Option<rosetta_uuid::Uuid>,
    pub public_background: Option<rosetta_uuid::Uuid>,
    pub public_note: Option<String>,
    pub auth_login_attempts: Option<i32>,
    pub auth_password_policy: Option<String>,
    pub storage_asset_transform: Option<String>,
    pub storage_asset_presets: Option<serde_json::Value>,
    pub custom_css: Option<String>,
    pub storage_default_folder: Option<rosetta_uuid::Uuid>,
    pub basemaps: Option<serde_json::Value>,
    pub mapbox_key: Option<String>,
    pub module_bar: Option<serde_json::Value>,
    pub project_descriptor: Option<String>,
    pub default_language: String,
    pub custom_aspect_ratios: Option<serde_json::Value>,
    pub public_favicon: Option<rosetta_uuid::Uuid>,
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
    pub public_registration_role: Option<rosetta_uuid::Uuid>,
    pub public_registration_email_filter: Option<serde_json::Value>,
}
impl diesel::Identifiable for DirectusSetting {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusSetting {
    #[cfg(feature = "postgres")]
    pub async fn project_logo(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(project_logo) = self.project_logo.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::dsl::id
                    .eq(project_logo),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_foreground(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(public_foreground) = self.public_foreground.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::dsl::id
                    .eq(public_foreground),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_background(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(public_background) = self.public_background.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::dsl::id
                    .eq(public_background),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn storage_default_folder(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(storage_default_folder) = self.storage_default_folder.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_folders::directus_folders::dsl::id
                    .eq(storage_default_folder),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>(
                conn,
            )
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_favicon(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(public_favicon) = self.public_favicon.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::dsl::id
                    .eq(public_favicon),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn public_registration_role(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(public_registration_role) = self.public_registration_role.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_roles::directus_roles::dsl::id
                    .eq(public_registration_role),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_project_logo(
        conn: &mut diesel::PgConnection,
        project_logo: &crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::dsl::project_logo
                    .eq(project_logo.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_public_foreground(
        conn: &mut diesel::PgConnection,
        public_foreground: &crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::dsl::public_foreground
                    .eq(public_foreground.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_public_background(
        conn: &mut diesel::PgConnection,
        public_background: &crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::dsl::public_background
                    .eq(public_background.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_storage_default_folder(
        conn: &mut diesel::PgConnection,
        storage_default_folder: &crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::dsl::storage_default_folder
                    .eq(storage_default_folder.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_public_favicon(
        conn: &mut diesel::PgConnection,
        public_favicon: &crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::dsl::public_favicon
                    .eq(public_favicon.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_public_registration_role(
        conn: &mut diesel::PgConnection,
        public_registration_role: &crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::dsl::public_registration_role
                    .eq(public_registration_role.id),
            )
            .load::<Self>(conn)
            .await
    }
}
