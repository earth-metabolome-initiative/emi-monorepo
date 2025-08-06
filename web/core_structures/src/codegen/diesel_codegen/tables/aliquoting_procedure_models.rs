diesel::table! {
    aliquoting_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, liters -> diesel::sql_types::Float, aliquoted_from ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_aliquoted_from ->
    diesel::sql_types::Integer, aliquoted_into -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_aliquoted_into -> diesel::sql_types::Integer, aliquoted_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_aliquoted_with ->
    diesel::sql_types::Integer, pipette_tip -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_pipette_tip -> diesel::sql_types::Integer }
}
