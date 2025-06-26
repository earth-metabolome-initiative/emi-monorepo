diesel::table! {
    weighing_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, weighed_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_weighed_with -> diesel::sql_types::Integer, sample_container ->
    diesel::sql_types::Integer }
}
