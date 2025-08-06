diesel::table! {
    freezing_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float,
    kelvin_tolerance_percentage -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Nullable < diesel::sql_types::Float >, frozen_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_frozen_with ->
    diesel::sql_types::Integer, frozen_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_frozen_container_id ->
    diesel::sql_types::Integer }
}
