diesel::table! {
    directus_files(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, storage ->
    diesel::sql_types::Text, filename_disk -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, filename_download -> diesel::sql_types::Text, title ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, #[sql_name = "type"] r#type
    -> diesel::sql_types::Nullable < diesel::sql_types::Text >, folder ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, uploaded_by ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, created_on ->
    rosetta_timestamp::diesel_impls::TimestampUTC, modified_by ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, modified_on ->
    rosetta_timestamp::diesel_impls::TimestampUTC, charset -> diesel::sql_types::Nullable
    < diesel::sql_types::Text >, filesize -> diesel::sql_types::Nullable <
    diesel::sql_types::BigInt >, width -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, height -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, duration -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, embed -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, description -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, location -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, tags -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, metadata -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, focal_point_x -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, focal_point_y -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, tus_id -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, tus_data -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, uploaded_on -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC > }
}
