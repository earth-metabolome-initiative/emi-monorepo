diesel::table! {
    public.directus_users(id) { id -> rosetta_uuid::diesel_impls::Uuid, first_name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, last_name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, email ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, password ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, location ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, title ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, description ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, tags ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, avatar ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, language ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, tfa_secret ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, status ->
    diesel::sql_types::Text, role -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, token -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, last_access -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, last_page ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, provider ->
    diesel::sql_types::Text, external_identifier -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, auth_data -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, email_notifications -> diesel::sql_types::Nullable <
    diesel::sql_types::Bool >, appearance -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, theme_dark -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, theme_light -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, theme_light_overrides -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, theme_dark_overrides -> diesel::sql_types::Nullable <
    diesel::sql_types::Json > }
}
