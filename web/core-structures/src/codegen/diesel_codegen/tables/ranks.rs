#[cfg(feature = "diesel")]
diesel::table! { public . ranks (id) { name -> diesel :: sql_types :: Text , description -> diesel :: sql_types :: Text , id -> diesel :: sql_types :: SmallInt } }
