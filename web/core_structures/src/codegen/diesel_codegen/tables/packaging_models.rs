diesel::table! {
    packaging_models(trackable_id) { trackable_id -> ::rosetta_uuid::diesel_impls::Uuid,
    material_id -> diesel::sql_types::SmallInt }
}
