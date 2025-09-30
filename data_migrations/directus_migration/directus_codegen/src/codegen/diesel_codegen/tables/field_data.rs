diesel::table! {
    #[sql_name = "Field_Data"] field_data(id) { id -> diesel::sql_types::Integer,
    user_created -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    date_created -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, user_updated ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_updated ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    collector_fullname -> diesel::sql_types::Nullable < diesel::sql_types::Text >,
    observation_subject -> diesel::sql_types::Nullable < diesel::sql_types::Text >,
    inat_upload -> diesel::sql_types::Nullable < diesel::sql_types::Integer >, is_wild ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, taxon_name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, no_name_on_list ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, sample_id ->
    diesel::sql_types::Text, picture_panel -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, picture_general -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, picture_detail -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, picture_cut -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, picture_panel_label -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, x_coord -> diesel::sql_types::Nullable <
    diesel::sql_types::Float >, y_coord -> diesel::sql_types::Nullable <
    diesel::sql_types::Float >, collector_orcid -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, collector_inat -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, latitude -> diesel::sql_types::Nullable <
    diesel::sql_types::Float >, longitude -> diesel::sql_types::Nullable <
    diesel::sql_types::Float >, qfield_project -> diesel::sql_types::Text, picture_free
    -> diesel::sql_types::Nullable < diesel::sql_types::Text >, comment_eco ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, weather ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, sample_name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, name_proposition ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, ipen ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, match_name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, ott_id ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, rank ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, #[sql_name = "wikidataID"]
    wikidata_id -> diesel::sql_types::Nullable < diesel::sql_types::Text >, unknown ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, comment_env ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, herbivory_percent ->
    diesel::sql_types::Nullable < diesel::sql_types::Float >, #[sql_name =
    "temperature_°C"] temperature_celsius -> diesel::sql_types::Nullable <
    diesel::sql_types::Float >, geometry -> diesel::sql_types::Nullable <
    ::postgis_diesel::sql_types::Geometry >, date -> diesel::sql_types::Nullable <
    diesel::sql_types::BigInt >, soil_type -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, catalogue_number -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, extracted_id -> diesel::sql_types::Nullable <
    diesel::sql_types::Text > }
}
