diesel::table! {
    public.organisms(id) { id -> rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, nameplate_category_id ->
    diesel::sql_types::SmallInt }
}
