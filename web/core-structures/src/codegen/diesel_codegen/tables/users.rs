#[cfg(feature = "diesel")]
diesel::table! { public . users (id) { id -> diesel :: sql_types :: Integer , name -> diesel :: sql_types :: Text , picture -> diesel :: sql_types :: Binary , created_at -> diesel :: sql_types :: Timestamp , updated_at -> diesel :: sql_types :: Timestamp } }
