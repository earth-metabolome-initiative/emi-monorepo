#[cfg(feature = "diesel")]
diesel::table! { public . taxa (id) { id -> diesel :: sql_types :: Integer , name -> diesel :: sql_types :: Text , parent_id -> diesel :: sql_types :: Nullable < diesel :: sql_types :: Integer > , rank_id -> diesel :: sql_types :: SmallInt } }
