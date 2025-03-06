#[cfg(feature = "diesel")]
diesel::table! { public . sample_taxa (sample_id , taxon_id) { created_by -> diesel :: sql_types :: Integer , sample_id -> diesel :: sql_types :: Uuid , taxon_id -> diesel :: sql_types :: Integer , created_at -> diesel :: sql_types :: Timestamp } }
