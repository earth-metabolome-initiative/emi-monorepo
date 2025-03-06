#[cfg(feature = "diesel")]
diesel::table! { public . observation_subjects (id) { name -> diesel :: sql_types :: Text , description -> diesel :: sql_types :: Text , icon -> diesel :: sql_types :: Text , color -> diesel :: sql_types :: Text , id -> diesel :: sql_types :: SmallInt } }
