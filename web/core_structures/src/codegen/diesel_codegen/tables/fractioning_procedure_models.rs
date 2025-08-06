diesel::table! {
    fractioning_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, kilograms -> diesel::sql_types::Float,
    tolerance_percentage -> diesel::sql_types::Float, weighed_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_weighed_with ->
    diesel::sql_types::Integer, procedure_fragment_source -> diesel::sql_types::Integer,
    fragment_placed_into -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_fragment_placed_into -> diesel::sql_types::Integer }
}
