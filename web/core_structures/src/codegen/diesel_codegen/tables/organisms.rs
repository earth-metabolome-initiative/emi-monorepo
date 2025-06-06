diesel::table! {
    organisms(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, nameplate_category ->
    ::nameplate_categories::diesel_impls::NameplateCategory }
}
