diesel::table! {
    freezing_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Float, frozen_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_frozen_with -> diesel::sql_types::Integer, source_container ->
    diesel::sql_types::Integer }
}
