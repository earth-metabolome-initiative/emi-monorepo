diesel::table! {
    aliquoting_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    aliquoted_with -> ::rosetta_uuid::diesel_impls::Uuid, pipette_tip_model ->
    diesel::sql_types::Integer, aliquoted_from -> ::rosetta_uuid::diesel_impls::Uuid }
}
