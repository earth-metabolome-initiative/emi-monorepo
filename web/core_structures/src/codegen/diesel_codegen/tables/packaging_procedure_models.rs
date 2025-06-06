diesel::table! {
    packaging_procedure_models(id) { id -> diesel::sql_types::Integer, packaging_model_id
    -> ::rosetta_uuid::diesel_impls::Uuid }
}
