diesel::table! {
    procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, most_concrete_table ->
    diesel::sql_types::Text, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
