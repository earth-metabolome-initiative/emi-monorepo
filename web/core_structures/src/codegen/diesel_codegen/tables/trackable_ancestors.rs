diesel::table! {
    trackable_ancestors(trackable_id, ancestor_id) { trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, ancestor_id -> ::rosetta_uuid::diesel_impls::Uuid
    }
}
