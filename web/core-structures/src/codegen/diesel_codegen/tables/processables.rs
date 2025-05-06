diesel::table! {
    public.processables(id) { id -> rosetta_uuid::diesel_impls::Uuid, kilograms ->
    diesel::sql_types::Float }
}
