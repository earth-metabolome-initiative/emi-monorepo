diesel::table! {
    ball_mill_container_models(milled_with, container_model_id) { milled_with ->
    ::rosetta_uuid::diesel_impls::Uuid, container_model_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
