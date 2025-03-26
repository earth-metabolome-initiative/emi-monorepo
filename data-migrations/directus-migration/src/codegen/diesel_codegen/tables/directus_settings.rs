#[cfg(feature = "64-column-tables")]
diesel::table! {
    public.directus_settings(id) { id -> diesel::sql_types::Integer, project_name ->
    diesel::sql_types::Text, project_url -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, project_color -> diesel::sql_types::Text, project_logo ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, public_foreground ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, public_background ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, public_note ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, auth_login_attempts ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, auth_password_policy ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, storage_asset_transform ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, storage_asset_presets ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, custom_css ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, storage_default_folder ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, basemaps ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, mapbox_key ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, module_bar ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, project_descriptor ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, default_language ->
    diesel::sql_types::Text, custom_aspect_ratios -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, public_favicon -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, default_appearance -> diesel::sql_types::Text,
    default_theme_light -> diesel::sql_types::Nullable < diesel::sql_types::Text >,
    theme_light_overrides -> diesel::sql_types::Nullable < diesel::sql_types::Json >,
    default_theme_dark -> diesel::sql_types::Nullable < diesel::sql_types::Text >,
    theme_dark_overrides -> diesel::sql_types::Nullable < diesel::sql_types::Json >,
    report_error_url -> diesel::sql_types::Nullable < diesel::sql_types::Text >,
    report_bug_url -> diesel::sql_types::Nullable < diesel::sql_types::Text >,
    report_feature_url -> diesel::sql_types::Nullable < diesel::sql_types::Text >,
    public_registration -> diesel::sql_types::Bool, public_registration_verify_email ->
    diesel::sql_types::Bool, public_registration_role -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, public_registration_email_filter ->
    diesel::sql_types::Nullable < diesel::sql_types::Json > }
}
