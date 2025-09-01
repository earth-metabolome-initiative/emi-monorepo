diesel::table! {
    freeze_drying_procedures(procedure) { procedure ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template -> diesel::sql_types::Integer,
    foreign_procedure_template -> diesel::sql_types::Integer, foreign_procedure ->
    ::rosetta_uuid::diesel_impls::Uuid, freeze_dryed_container ->
    ::rosetta_uuid::diesel_impls::Uuid, freeze_dryed_with -> diesel::sql_types::Nullable
    < ::rosetta_uuid::diesel_impls::Uuid >, freeze_dryed_with_model ->
    diesel::sql_types::Integer }
}
