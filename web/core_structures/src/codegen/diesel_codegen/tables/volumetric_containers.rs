diesel::table! {
    volumetric_containers(id) { id -> ::rosetta_uuid::diesel_impls::Uuid,
    volumetric_container_model_id -> diesel::sql_types::Integer }
}
