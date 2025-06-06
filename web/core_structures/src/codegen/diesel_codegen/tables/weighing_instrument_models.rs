diesel::table! {
    weighing_instrument_models(id) { id -> ::rosetta_uuid::diesel_impls::Uuid,
    error_kilograms -> diesel::sql_types::Float, minimum_measurable_kilograms ->
    diesel::sql_types::Float, maximum_measurable_kilograms -> diesel::sql_types::Float }
}
