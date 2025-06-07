diesel::table! {
    organism_taxa(organism_id, taxon_id) { created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC, organism_id ->
    ::rosetta_uuid::diesel_impls::Uuid, taxon_id -> diesel::sql_types::Integer }
}
