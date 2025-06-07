diesel::table! {
    aliquoting_instrument_models(id) { id -> ::rosetta_uuid::diesel_impls::Uuid,
    error_liters -> diesel::sql_types::Float, minimum_measurable_liters ->
    diesel::sql_types::Float, maximum_measurable_liters -> diesel::sql_types::Float }
}
