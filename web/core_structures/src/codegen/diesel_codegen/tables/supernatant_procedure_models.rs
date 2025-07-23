diesel::table! {
    supernatant_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, liters -> diesel::sql_types::Float, stratified_source ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_stratified_source ->
    diesel::sql_types::Integer, supernatant_destination ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_supernatant_destination ->
    diesel::sql_types::Integer, transferred_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_transferred_with -> diesel::sql_types::Integer, pipette_tip ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_pipette_tip ->
    diesel::sql_types::Integer }
}
