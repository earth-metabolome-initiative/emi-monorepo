diesel::table! {
    procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    parent_procedure -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid
    >, parent_procedure_template -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, procedure_template -> diesel::sql_types::Integer,
    most_concrete_table -> diesel::sql_types::Text, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
