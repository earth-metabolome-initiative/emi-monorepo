diesel::table! {
    public.processables(id) { id -> rosetta_uuid::diesel_impls::Uuid }
}
