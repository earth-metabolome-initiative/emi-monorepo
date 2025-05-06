diesel::table! {
    public.organisms(id) { id -> rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, nameplate_category ->
    nameplate_categories::diesel_impls::NameplateCategory }
}
