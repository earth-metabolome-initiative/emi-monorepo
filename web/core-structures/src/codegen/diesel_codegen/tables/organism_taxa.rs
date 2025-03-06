#[cfg(feature = "diesel")]
diesel::table! { public . organism_taxa (organism_id , taxon_id) { created_by -> diesel :: sql_types :: Integer , organism_id -> diesel :: sql_types :: Uuid , taxon_id -> diesel :: sql_types :: Integer , created_at -> diesel :: sql_types :: Timestamp } }
