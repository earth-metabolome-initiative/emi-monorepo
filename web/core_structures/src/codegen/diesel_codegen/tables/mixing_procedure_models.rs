diesel::table! {
    mixing_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, measured_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_measured_with -> diesel::sql_types::Integer, source ->
    diesel::sql_types::Integer, mixed_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_mixed_into -> diesel::sql_types::Integer, kilograms ->
    diesel::sql_types::Float }
}
