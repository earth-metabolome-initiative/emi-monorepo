#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
        foreign_key = storage_default_folder
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
        foreign_key = public_registration_role
    )
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
    pub project_logo: Option<::rosetta_uuid::Uuid>,
    pub public_foreground: Option<::rosetta_uuid::Uuid>,
    pub public_background: Option<::rosetta_uuid::Uuid>,
    pub public_note: Option<String>,
    pub auth_login_attempts: Option<i32>,
    pub auth_password_policy: Option<String>,
    pub storage_asset_transform: Option<String>,
    pub storage_asset_presets: Option<::serde_json::Value>,
    pub custom_css: Option<String>,
    pub storage_default_folder: Option<::rosetta_uuid::Uuid>,
    pub basemaps: Option<::serde_json::Value>,
    pub mapbox_key: Option<String>,
    pub module_bar: Option<::serde_json::Value>,
    pub project_descriptor: Option<String>,
    pub default_language: String,
    pub custom_aspect_ratios: Option<::serde_json::Value>,
    pub public_favicon: Option<::rosetta_uuid::Uuid>,
    pub default_appearance: String,
    pub default_theme_light: Option<String>,
    pub theme_light_overrides: Option<::serde_json::Value>,
    pub default_theme_dark: Option<String>,
    pub theme_dark_overrides: Option<::serde_json::Value>,
    pub report_error_url: Option<String>,
    pub report_bug_url: Option<String>,
    pub report_feature_url: Option<String>,
    pub public_registration: bool,
    pub public_registration_verify_email: bool,
    pub public_registration_role: Option<::rosetta_uuid::Uuid>,
    pub public_registration_email_filter: Option<::serde_json::Value>,
    pub visual_editor_urls: Option<::serde_json::Value>,
}
impl web_common_traits::prelude::TableName for DirectusSetting {
    const TABLE_NAME: &'static str = "directus_settings";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_settings::DirectusSetting,
    > for DirectusSetting
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for DirectusSetting {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusSetting {
    pub fn project_logo<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(project_logo) = self.project_logo else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::read(
            project_logo,
            conn,
        )
        .optional()
    }
    pub fn public_background<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(public_background) = self.public_background else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::read(
            public_background,
            conn,
        )
        .optional()
    }
    pub fn public_favicon<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(public_favicon) = self.public_favicon else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::read(
            public_favicon,
            conn,
        )
        .optional()
    }
    pub fn public_foreground<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(public_foreground) = self.public_foreground else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_files::DirectusFile::read(
            public_foreground,
            conn,
        )
        .optional()
    }
    pub fn storage_default_folder<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(storage_default_folder) = self.storage_default_folder else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::read(
            storage_default_folder,
            conn,
        )
        .optional()
    }
    pub fn public_registration_role<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(public_registration_role) = self.public_registration_role else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::read(
            public_registration_role,
            conn,
        )
        .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_project_logo(
        project_logo: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::project_logo.eq(project_logo))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_public_background(
        public_background: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::public_background.eq(public_background))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_public_favicon(
        public_favicon: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::public_favicon.eq(public_favicon))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_public_foreground(
        public_foreground: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::public_foreground.eq(public_foreground))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_project_name(
        project_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::project_name.eq(project_name))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_project_url(
        project_url: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::project_url.eq(project_url))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_project_color(
        project_color: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::project_color.eq(project_color))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_public_note(
        public_note: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::public_note.eq(public_note))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_auth_login_attempts(
        auth_login_attempts: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::auth_login_attempts.eq(auth_login_attempts))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_auth_password_policy(
        auth_password_policy: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::auth_password_policy.eq(auth_password_policy))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_storage_asset_transform(
        storage_asset_transform: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::storage_asset_transform.eq(storage_asset_transform))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_custom_css(
        custom_css: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::custom_css.eq(custom_css))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_mapbox_key(
        mapbox_key: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::mapbox_key.eq(mapbox_key))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_project_descriptor(
        project_descriptor: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::project_descriptor.eq(project_descriptor))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_default_language(
        default_language: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::default_language.eq(default_language))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_default_appearance(
        default_appearance: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::default_appearance.eq(default_appearance))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_default_theme_light(
        default_theme_light: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::default_theme_light.eq(default_theme_light))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_default_theme_dark(
        default_theme_dark: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::default_theme_dark.eq(default_theme_dark))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_report_error_url(
        report_error_url: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::report_error_url.eq(report_error_url))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_report_bug_url(
        report_bug_url: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::report_bug_url.eq(report_bug_url))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_report_feature_url(
        report_feature_url: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::report_feature_url.eq(report_feature_url))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_public_registration<C>(
        public_registration: bool,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::public_registration as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::public_registration as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::public_registration as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(directus_settings::public_registration.eq(public_registration))
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_public_registration_verify_email<C>(
        public_registration_verify_email: bool,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::public_registration_verify_email as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::public_registration_verify_email as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::public_registration_verify_email as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_settings::directus_settings::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
        Self::table()
            .filter(
                directus_settings::public_registration_verify_email
                    .eq(public_registration_verify_email),
            )
            .order_by(directus_settings::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusSetting> for DirectusSetting {
    fn as_ref(&self) -> &DirectusSetting {
        self
    }
}
