diesel::table! {
    pouring_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    poured_from -> ::rosetta_uuid::diesel_impls::Uuid, procedure_template ->
    diesel::sql_types::Integer, foreign_procedure_template -> diesel::sql_types::Integer,
    foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid, measured_with_model ->
    diesel::sql_types::Integer, measured_with -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, poured_into ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
