diesel::table! {
    storage_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    stored_asset -> ::rosetta_uuid::diesel_impls::Uuid, stored_with ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
