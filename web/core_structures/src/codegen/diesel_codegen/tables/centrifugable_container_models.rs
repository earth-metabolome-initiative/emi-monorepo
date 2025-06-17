diesel::table! {
    centrifugable_container_models(centrifuged_with, container_model_id) {
    centrifuged_with -> ::rosetta_uuid::diesel_impls::Uuid, container_model_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
