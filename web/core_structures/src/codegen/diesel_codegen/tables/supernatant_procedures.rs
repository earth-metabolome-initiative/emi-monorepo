diesel::table! {
    supernatant_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    stratified_source -> ::rosetta_uuid::diesel_impls::Uuid, supernatant_destination ->
    ::rosetta_uuid::diesel_impls::Uuid, transferred_with ->
    ::rosetta_uuid::diesel_impls::Uuid, pipette_tip_model -> diesel::sql_types::Integer }
}
