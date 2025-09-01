diesel::table! {
    freeze_dryers(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, model_id ->
    diesel::sql_types::Integer }
}
