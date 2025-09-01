diesel::table! {
    capping_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    capped_container -> ::rosetta_uuid::diesel_impls::Uuid, capped_with_model ->
    diesel::sql_types::Integer }
}
