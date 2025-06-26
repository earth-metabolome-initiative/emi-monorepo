diesel::table! {
    geolocation_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, geolocated_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_geolocated_with -> diesel::sql_types::Integer, trackable_id ->
    diesel::sql_types::Integer }
}
